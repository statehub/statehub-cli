//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::fmt;

use inspector::ResultInspector;
use serde::{de, ser, Serialize};

use crate::location::Location;
use crate::output::Output;
use crate::show::Show;
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

    pub(crate) async fn create_state(
        self,
        name: v1::StateName,
        owner: Option<v1::ClusterName>,
        locations: v1::Locations,
    ) -> anyhow::Result<()> {
        let text = |output| self.show(output);

        let body = v1::State {
            name,
            storage_class: None,
            owner,
            locations,
            allowed_clusters: None,
        };
        self.post::<_, _, v1::State>("/states", body)
            .await
            .map(text)
            .map(print)
    }

    pub(crate) async fn list_states(self) -> anyhow::Result<()> {
        let text = |output| self.show(output);

        self.get::<_, Vec<v1::State>>("/states")
            .await
            .map(text)
            .map(print)
    }

    pub(crate) async fn show_state(self, state: v1::StateName) -> anyhow::Result<()> {
        let text = |output| self.show(output);

        let path = format!("/states/{state}", state = state);
        self.get::<_, v1::State>(path).await.map(text).map(print)
    }

    pub(crate) async fn list_clusters(self) -> anyhow::Result<()> {
        let text = |output| self.show(output);

        self.get::<_, Vec<v1::Cluster>>("/clusters")
            .await
            .map(text)
            .map(print)
    }

    pub(crate) async fn register_cluster(self) -> anyhow::Result<()> {
        // let text = |output| self.show(output);
        // Ok(Output::<String>::todo()).map(text).map(print)
        anyhow::bail!(self.show(Output::<String>::todo()))
    }

    pub(crate) async fn unregister_cluster(self) -> anyhow::Result<()> {
        // let text = |output| self.show(output);
        // Ok(Output::<String>::todo()).map(text).map(print)
        anyhow::bail!(self.show(Output::<String>::todo()))
    }

    pub(crate) async fn create_volume(self) -> anyhow::Result<()> {
        // let text = |output| self.show(output);
        // Ok(Output::<String>::todo()).map(text).map(print)
        anyhow::bail!(self.show(Output::<String>::todo()))
    }

    pub(crate) async fn delete_volume(self) -> anyhow::Result<()> {
        // let text = |output| self.show(output);
        // Ok(Output::<String>::todo()).map(text).map(print)
        anyhow::bail!(self.show(Output::<String>::todo()))
    }

    pub(crate) async fn add_location(
        self,
        state: v1::StateName,
        location: Location,
    ) -> anyhow::Result<()> {
        let text = |output| self.show(output);

        #[derive(Debug, Serialize)]
        struct StateLocation {
            region: String,
        }

        let (path, body) = match location {
            Location::Aws(region) => (
                format!("/states/{state}/locations/aws", state = state),
                StateLocation {
                    region: region.to_string(),
                },
            ),
            Location::Azure(region) => (
                format!("/states/{state}/locations/azure", state = state),
                StateLocation {
                    region: region.to_string(),
                },
            ),
        };

        self.post::<_, _, v1::State>(path, body)
            .await
            .map(text)
            .map(print)
    }

    pub(crate) async fn remove_location(self) -> anyhow::Result<()> {
        // let text = |output| self.show(output);
        // Ok(Output::<String>::todo()).map(text).map(print)
        anyhow::bail!(self.show(Output::<String>::todo()))
    }

    pub(crate) async fn set_availability(self) -> anyhow::Result<()> {
        // let text = |output| self.show(output);
        // Ok(Output::<String>::todo()).map(text).map(print)
        anyhow::bail!(self.show(Output::<String>::todo()))
    }

    pub(crate) async fn set_owner(
        self,
        state: v1::StateName,
        cluster: v1::ClusterName,
    ) -> anyhow::Result<()> {
        let text = |output| self.show(output);

        let path = format!(
            "/states/{state}/owner/{cluster}",
            state = state,
            cluster = cluster,
        );
        self.put::<_, v1::State>(path).await.map(text).map(print)
    }

    pub(crate) async fn unset_owner(
        self,
        state: v1::StateName,
        cluster: v1::ClusterName,
    ) -> anyhow::Result<()> {
        let text = |output| self.show(output);

        let path = format!(
            "/states/{state}/owner/{cluster}",
            state = state,
            cluster = cluster,
        );
        self.del::<_, v1::State>(path).await.map(text).map(print)
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

    fn show<T>(&self, output: Output<T>) -> String
    where
        T: de::DeserializeOwned + ser::Serialize + Show,
    {
        if self.json {
            output.into_value().show()
        } else if !self.raw {
            output.into_typed().show()
        } else {
            output.show()
        }
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
        self.optionally(token, |this, token| this.bearer_auth(token))
    }
}

fn print(text: String) {
    println!("{}", text);
}
