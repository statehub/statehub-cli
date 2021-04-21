//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::fmt;

use inspector::ResultInspector;
use serde::{de, ser};

use crate::output::Output;
use crate::v1;

pub(crate) type ApiResult<T> = Result<Output<T>, anyhow::Error>;

#[derive(Debug)]
pub(crate) struct Api {
    base: String,
    token: Option<String>,
    json: bool,
    raw: bool,
    verbose: bool,
    user_agent: String,
}

impl Api {
    pub(crate) fn new(
        management: String,
        token: Option<String>,
        json: bool,
        raw: bool,
        verbose: bool,
    ) -> Self {
        let user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        let endpoint = "endpoint";

        let base = if management.starts_with("http") {
            format!("{}/{}", management, endpoint)
        } else if management.starts_with("api.") && management.ends_with(".statehub.io") {
            format!("https://{}/{}", management, endpoint)
        } else {
            format!("http://{}:3000/{}", management, endpoint)
        };

        Self {
            base,
            token,
            json,
            raw,
            verbose,
            user_agent,
        }
    }

    pub(crate) async fn create_state(&self, state: v1::State) -> ApiResult<v1::State> {
        self.post("/states", state).await
    }

    pub(crate) async fn delete_state(&self, name: v1::StateName) -> ApiResult<v1::State> {
        let path = format!("/states/{name}", name = name);
        self.del(path).await
    }

    pub(crate) async fn get_states(&self) -> ApiResult<Vec<v1::State>> {
        self.get("/states").await
    }

    pub(crate) async fn show_state(&self, name: v1::StateName) -> ApiResult<v1::State> {
        let path = format!("/states/{name}", name = name);
        self.get(path).await
    }

    pub(crate) async fn list_clusters(&self) -> ApiResult<Vec<v1::Cluster>> {
        self.get("/clusters").await
    }

    pub(crate) async fn register_cluster(&self, name: String) -> ApiResult<v1::Cluster> {
        let name = v1::ClusterName(name);
        let body = v1::CreateClusterDto { name };
        self.post("/clusters", body).await
    }

    pub(crate) async fn add_aws_location(
        &self,
        name: v1::StateName,
        region: v1::AwsRegion,
    ) -> ApiResult<v1::StateLocationAws> {
        let path = format!("/states/{name}/locations/aws", name = name);
        let body = v1::CreateStateLocationAws { region };
        self.post(path, body).await
    }

    pub(crate) async fn add_azure_location(
        &self,
        name: v1::StateName,
        region: v1::AzureRegion,
    ) -> ApiResult<v1::StateLocationAzure> {
        let path = format!("/states/{name}/locations/azure", name = name);
        let body = v1::CreateStateLocationAzure { region };
        self.post(path, body).await
    }

    pub(crate) async fn set_owner(
        &self,
        state: v1::StateName,
        cluster: v1::ClusterName,
    ) -> ApiResult<v1::State> {
        let path = format!(
            "/states/{state}/owner/{cluster}",
            state = state,
            cluster = cluster,
        );
        self.put(path).await
    }

    pub(crate) async fn unset_owner(
        &self,
        state: v1::StateName,
        cluster: v1::ClusterName,
    ) -> ApiResult<v1::State> {
        let path = format!(
            "/states/{state}/owner/{cluster}",
            state = state,
            cluster = cluster,
        );
        self.del(path).await
    }

    fn url(&self, path: impl fmt::Display) -> String {
        format!("{}{}", self.base, path)
    }

    fn inspect<T>(&self, output: &Output<T>)
    where
        T: fmt::Debug,
    {
        if self.verbose {
            println!("{:#?}", output);
        }
    }

    async fn del<P, T>(&self, path: P) -> ApiResult<T>
    where
        P: fmt::Display,
        T: de::DeserializeOwned + fmt::Debug,
    {
        let url = self.url(path);
        let output = self
            .client()?
            .delete(url)
            .optionally_bearer_auth(self.token.as_ref())
            .send()
            .await?
            .bytes()
            .await
            .map(Output::Raw)
            .inspect(|output| self.inspect(output))?;
        Ok(output)
    }

    async fn get<P, T>(&self, path: P) -> ApiResult<T>
    where
        P: fmt::Display,
        T: de::DeserializeOwned + fmt::Debug,
    {
        let url = self.url(path);
        let output = self
            .client()?
            .get(url)
            .optionally_bearer_auth(self.token.as_ref())
            .send()
            .await?
            .bytes()
            .await
            .map(Output::Raw)
            .inspect(|output| self.inspect(output))?;

        Ok(output)
    }

    async fn post<P, T, U>(&self, path: P, body: T) -> ApiResult<U>
    where
        P: fmt::Display,
        T: ser::Serialize,
        U: de::DeserializeOwned + fmt::Debug,
    {
        let url = self.url(path);
        let output = self
            .client()?
            .post(url)
            .optionally_bearer_auth(self.token.as_ref())
            .json(&body)
            .send()
            .await?
            .bytes()
            .await
            .map(Output::Raw)
            .inspect(|output| self.inspect(output))?;

        Ok(output)
    }

    async fn put<P, T>(&self, path: P) -> ApiResult<T>
    where
        P: fmt::Display,
        T: de::DeserializeOwned + fmt::Debug,
    {
        let url = format!("{}{}", self.base, path);
        let output = self
            .client()?
            .put(url)
            .optionally_bearer_auth(self.token.as_ref())
            .send()
            .await?
            .bytes()
            .await
            .map(Output::Raw)
            .inspect(|output| self.inspect(output))?;
        Ok(output)
    }

    fn client(&self) -> reqwest::Result<reqwest::Client> {
        reqwest::Client::builder()
            .user_agent(&self.user_agent)
            .build()
    }
}

trait Optionally {
    fn optionally_bearer_auth(self, token: Option<impl fmt::Display>) -> Self;

    fn optionally<T, F>(self, option: Option<T>, f: F) -> Self
    where
        F: FnOnce(Self, T) -> Self,
        Self: Sized,
    {
        if let Some(option) = option {
            f(self, option)
        } else {
            self
        }
    }
}

impl Optionally for reqwest::RequestBuilder {
    fn optionally_bearer_auth(self, token: Option<impl fmt::Display>) -> Self {
        self.optionally(token, Self::bearer_auth)
    }
}
