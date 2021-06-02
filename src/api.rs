//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::convert::TryInto;
use std::fmt;
use std::fmt::Debug;

use inspector::ResultInspector;
use serde::{de, ser};

use crate::output::Output;
use crate::v1;

pub(crate) type ApiResult<T> = Result<Output<T>, anyhow::Error>;

#[derive(Debug)]
pub(crate) struct Api {
    base: String,
    token: Option<String>,
    verbose: bool,
    user_agent: String,
}

impl Api {
    pub(crate) fn new(management: &str, token: Option<&str>, verbose: bool) -> Self {
        let user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        let token = token.map(|s| s.to_string());

        let base = if management.starts_with("http") {
            format!("{}{}", management, v1::VERSION)
        } else if management.starts_with("api.") && management.ends_with(".statehub.io") {
            format!("https://{}{}", management, v1::VERSION)
        } else {
            format!("http://{}:3000{}", management, v1::VERSION)
        };

        log::debug!("Using API at {} with token {:?}", base, token);

        Self {
            base,
            token,
            verbose,
            user_agent,
        }
    }

    pub(crate) async fn is_unauthorized(&self) -> bool {
        self.get_all_states()
            .await
            .err()
            .and_then(|e| e.downcast::<reqwest::Error>().ok())
            .and_then(|e| e.status())
            .map_or(false, |status| status == reqwest::StatusCode::UNAUTHORIZED)
    }

    pub(crate) async fn create_state(&self, state: v1::CreateStateDto) -> ApiResult<v1::State> {
        self.post("/states", state).await
    }

    pub(crate) async fn delete_state(&self, name: v1::StateName) -> ApiResult<v1::State> {
        let path = format!("/states/{name}", name = name);
        self.del(path).await
    }

    pub(crate) async fn get_state(&self, name: &v1::StateName) -> ApiResult<v1::State> {
        let path = format!("/states/{name}", name = name);
        self.get(path).await
    }

    pub(crate) async fn get_all_states(&self) -> ApiResult<Vec<v1::State>> {
        self.get("/states").await
    }

    pub(crate) async fn list_clusters(&self) -> ApiResult<Vec<v1::Cluster>> {
        self.get("/clusters").await
    }

    pub(crate) async fn register_cluster(&self, name: &v1::ClusterName) -> ApiResult<v1::Cluster> {
        let name = name.clone();
        let body = v1::CreateClusterDto { name };
        self.post("/clusters", body).await
    }

    pub(crate) async fn unregister_cluster(&self, name: v1::ClusterName) -> ApiResult<v1::Cluster> {
        let path = format!("/clusters/{name}", name = name);
        self.del(path).await
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

    pub(crate) async fn del_aws_location(
        &self,
        name: v1::StateName,
        region: v1::AwsRegion,
    ) -> ApiResult<v1::StateLocationAws> {
        let path = format!(
            "/states/{name}/locations/aws/{region}",
            name = name,
            region = region
        );
        self.del(path).await
    }

    pub(crate) async fn del_azure_location(
        &self,
        name: v1::StateName,
        region: v1::AzureRegion,
    ) -> ApiResult<v1::StateLocationAzure> {
        let path = format!(
            "/states/{name}/locations/azure/{region}",
            name = name,
            region = region
        );
        self.del(path).await
    }

    pub(crate) async fn set_owner(
        &self,
        state: impl AsRef<v1::StateName>,
        cluster: impl AsRef<v1::ClusterName>,
    ) -> ApiResult<v1::State> {
        let path = format!(
            "/states/{state}/owner/{cluster}",
            state = state.as_ref(),
            cluster = cluster.as_ref(),
        );
        self.put(path).await
    }

    pub(crate) async fn unset_owner(
        &self,
        state: impl AsRef<v1::StateName>,
        cluster: impl AsRef<v1::ClusterName>,
    ) -> ApiResult<v1::State> {
        let path = format!(
            "/states/{state}/owner/{cluster}",
            state = state.as_ref(),
            cluster = cluster.as_ref(),
        );
        self.del(path).await
    }

    pub(crate) async fn issue_cluster_token(
        &self,
        cluster: impl AsRef<v1::ClusterName>,
    ) -> ApiResult<v1::ClusterToken> {
        let path = format!("/clusters/{}/token", cluster.as_ref());
        self.post(path, "").await
    }

    fn url(&self, path: impl fmt::Display) -> String {
        format!("{}{}", self.base, path)
    }

    fn inspect<T>(&self, output: &Output<T>)
    where
        T: fmt::Debug,
    {
        if self.verbose {
            println!("{}", output);
        }
    }

    async fn del<P, T>(&self, path: P) -> ApiResult<T>
    where
        P: fmt::Display,
        T: de::DeserializeOwned + ser::Serialize + fmt::Debug,
    {
        let url = self.url(path);
        self.client()?
            .delete(url)
            .optionally_bearer_auth(self.token.as_ref())
            .inspect()
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?
            .try_into()
            .inspect(|output| self.inspect(output))
    }

    async fn get<P, T>(&self, path: P) -> ApiResult<T>
    where
        P: fmt::Display,
        T: de::DeserializeOwned + ser::Serialize + fmt::Debug,
    {
        let url = self.url(path);
        self.client()?
            .get(url)
            .optionally_bearer_auth(self.token.as_ref())
            .inspect()
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?
            .try_into()
            .inspect(|output| self.inspect(output))
    }

    async fn post<P, T, U>(&self, path: P, body: T) -> ApiResult<U>
    where
        P: fmt::Display,
        T: ser::Serialize,
        U: de::DeserializeOwned + ser::Serialize + fmt::Debug,
    {
        let url = self.url(path);
        self.client()?
            .post(url)
            .optionally_bearer_auth(self.token.as_ref())
            .inspect()
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?
            .try_into()
            .inspect(|output| self.inspect(output))
    }

    async fn put<P, T>(&self, path: P) -> ApiResult<T>
    where
        P: fmt::Display,
        T: de::DeserializeOwned + ser::Serialize + fmt::Debug,
    {
        let url = format!("{}{}", self.base, path);
        self.client()?
            .put(url)
            .optionally_bearer_auth(self.token.as_ref())
            .inspect()
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?
            .try_into()
            .inspect(|output| self.inspect(output))
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

trait Inspector {
    fn inspect(self) -> Self;
}

impl Inspector for reqwest::RequestBuilder {
    fn inspect(self) -> Self {
        if let Some(rb) = self.try_clone() {
            if let Ok(req) = rb.build() {
                let mut headers = String::from("");
                let mut host = String::from("");
                let method = req.method().as_str();
                let path = req.url().path();
                let scheme = req.url().scheme();

                if let Some(req_host) = req.url().host() {
                    host = req_host.to_string();
                }

                // append all headers one by one in a new line in string
                req.headers().into_iter().for_each(|(req_header, value)| {
                    if let Ok(str_value) = value.to_str() {
                        headers += format!("{}: {} \n", req_header, str_value).as_str();
                    }
                });
                headers.truncate(headers.len() - 1);

                let raw_req = format!(
                    "{} {} {} \n {} \n Host: {}",
                    method, path, scheme, headers, host
                );

                println!("\n Requested http querry is: \n {} \n", raw_req);
            }
        }

        self
    }
}
