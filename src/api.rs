//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::convert::TryInto;
use std::fmt;

use inspector::ResultInspector;
use secrecy::ExposeSecret;
use secrecy::SecretString;
use serde::{de, ser};
use serde_json as json;

use crate::location::Location;
use crate::output::Output;
use crate::v0;

pub(crate) type ApiResult<T> = Result<Output<T>, anyhow::Error>;
#[derive(Debug)]
pub(crate) struct Api {
    base: String,
    token: Option<SecretString>,
    user_agent: String,
}

impl Api {
    pub(crate) fn new(management: &str, token: Option<&str>) -> Self {
        let user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        let token = token.map(String::from).map(SecretString::new);

        let base = if management.starts_with("http") {
            format!("{}{}", management, v0::VERSION)
        } else if management.starts_with("api.") && management.ends_with(".statehub.io") {
            format!("https://{}{}", management, v0::VERSION)
        } else {
            format!("http://{}:3000{}", management, v0::VERSION)
        };

        log::debug!("Using API at {} with token {:?}", base, token);

        Self {
            base,
            token,
            user_agent,
        }
    }

    pub(crate) async fn is_unauthorized(&self) -> bool {
        self.get_all_clusters()
            .await
            .err()
            .and_then(|e| e.downcast::<reqwest::Error>().ok())
            .and_then(|e| e.status())
            .map_or(false, |status| status == reqwest::StatusCode::UNAUTHORIZED)
    }

    pub(crate) async fn create_state(&self, state: v0::CreateStateDto) -> ApiResult<v0::State> {
        self.post("/states", state).await
    }

    pub(crate) async fn create_volume(
        &self,
        state_name: v0::StateName,
        volume: v0::CreateVolumeDto,
    ) -> ApiResult<v0::Volume> {
        let path = format!("/states/{state_name}/volumes", state_name = state_name);
        self.post(path, volume).await
    }

    pub(crate) async fn delete_volume(
        &self,
        state: &v0::StateName,
        volume: &v0::VolumeName,
    ) -> ApiResult<v0::Volume> {
        let path = format!(
            "/states/{name}/volumes/{volume}",
            name = state,
            volume = volume
        );
        self.del(path).await
    }

    pub(crate) async fn set_volume_primary(
        &self,
        state: v0::StateName,
        volume: v0::VolumeName,
        primary: Location,
    ) -> ApiResult<v0::Volume> {
        let path = format!(
            "/states/{state}/volumes/{volume}/activeLocation/{primary:#}",
            state = state,
            volume = volume,
            primary = primary,
        );
        self.put(path).await
    }

    pub(crate) async fn get_volume(
        &self,
        state: &v0::StateName,
        volume: &v0::VolumeName,
    ) -> ApiResult<v0::Volume> {
        let path = format!(
            "/states/{state}/volumes/{volume}",
            state = state,
            volume = volume
        );
        self.get(path).await
    }

    pub(crate) async fn get_all_volumes(&self, state: v0::StateName) -> ApiResult<Vec<v0::Volume>> {
        let path = format!("/states/{state}/volumes", state = state,);
        self.get(path).await
    }

    pub(crate) async fn delete_state(&self, name: v0::StateName) -> ApiResult<()> {
        let path = format!("/states/{name}", name = name);
        self.del(path).await
    }

    pub(crate) async fn get_state(&self, name: &v0::StateName) -> ApiResult<v0::State> {
        let path = format!("/states/{name}", name = name);
        self.get(path).await
    }

    pub(crate) async fn get_all_states(&self) -> ApiResult<Vec<v0::State>> {
        self.get("/states").await
    }

    pub(crate) async fn get_cluster(&self, name: &v0::ClusterName) -> ApiResult<v0::Cluster> {
        let path = format!("/clusters/{name}", name = name);
        self.get(path).await
    }

    pub(crate) async fn get_all_clusters(&self) -> ApiResult<Vec<v0::Cluster>> {
        self.get("/clusters").await
    }

    pub(crate) async fn register_cluster(
        &self,
        name: &v0::ClusterName,
        provider: v0::Provider,
        locations: &[Location],
    ) -> ApiResult<v0::Cluster> {
        let name = name.clone();
        let locations = locations.into();
        let body = v0::CreateClusterDto {
            name,
            provider,
            locations,
        };
        self.post("/clusters", body).await
    }

    pub(crate) async fn unregister_cluster(&self, name: v0::ClusterName) -> ApiResult<()> {
        let path = format!("/clusters/{name}", name = name);
        self.del(path).await
    }

    pub(crate) async fn get_aws_location(
        &self,
        name: &v0::StateName,
        region: v0::AwsRegion,
    ) -> ApiResult<v0::StateLocationAws> {
        let path = format!(
            "/states/{name}/locations/aws/{region}",
            name = name,
            region = region
        );
        self.get(path).await
    }

    pub(crate) async fn add_aws_location(
        &self,
        name: &v0::StateName,
        region: v0::AwsRegion,
    ) -> ApiResult<v0::StateLocationAws> {
        let path = format!("/states/{name}/locations/aws", name = name);
        let body = v0::CreateStateLocationAwsDto { region };
        self.post(path, body).await
    }

    pub(crate) async fn del_aws_location(
        &self,
        name: v0::StateName,
        region: v0::AwsRegion,
    ) -> ApiResult<v0::StateLocationAws> {
        let path = format!(
            "/states/{name}/locations/aws/{region}",
            name = name,
            region = region
        );
        self.del(path).await
    }

    pub(crate) async fn get_azure_location(
        &self,
        name: &v0::StateName,
        region: v0::AzureRegion,
    ) -> ApiResult<v0::StateLocationAzure> {
        let path = format!(
            "/states/{name}/locations/azure/{region}",
            name = name,
            region = region
        );
        self.get(path).await
    }

    pub(crate) async fn add_azure_location(
        &self,
        name: &v0::StateName,
        region: v0::AzureRegion,
    ) -> ApiResult<v0::StateLocationAzure> {
        let path = format!("/states/{name}/locations/azure", name = name);
        let body = v0::CreateStateLocationAzureDto { region };
        self.post(path, body).await
    }

    pub(crate) async fn del_azure_location(
        &self,
        name: v0::StateName,
        region: v0::AzureRegion,
    ) -> ApiResult<v0::StateLocationAzure> {
        let path = format!(
            "/states/{name}/locations/azure/{region}",
            name = name,
            region = region
        );
        self.del(path).await
    }

    pub(crate) async fn set_owner(
        &self,
        state: &v0::StateName,
        cluster: &v0::ClusterName,
    ) -> ApiResult<v0::State> {
        let path = format!(
            "/states/{state}/owner/{cluster}",
            state = state,
            cluster = cluster,
        );
        self.put(path).await
    }

    pub(crate) async fn unset_owner(&self, state: &v0::StateName) -> ApiResult<v0::State> {
        let path = format!("/states/{state}/owner", state = state);
        self.del(path).await
    }

    pub(crate) async fn issue_cluster_token(
        &self,
        cluster: &v0::ClusterName,
    ) -> ApiResult<v0::ClusterToken> {
        let path = format!("/clusters/{}/token", cluster);
        self.post::<_, _, (), _>(path, None).await
    }

    pub(crate) fn url(&self, path: impl fmt::Display) -> String {
        format!("{}{}", self.base, path)
    }

    fn inspect<T>(&self, output: &Output<T>)
    where
        T: fmt::Debug,
    {
        log::debug!("Output {:?}", output);
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
            .retry()
            .await?
            .error_for_status2()
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
            .retry()
            .await?
            .error_for_status2()
            .await?
            .try_into()
            .inspect(|output| self.inspect(output))
    }

    async fn post<P, B, T, U>(&self, path: P, body: B) -> ApiResult<U>
    where
        P: fmt::Display,
        B: Into<Option<T>>,
        T: ser::Serialize,
        U: de::DeserializeOwned + ser::Serialize + fmt::Debug,
    {
        let body = body.into();
        let url = self.url(path);
        self.client()?
            .post(url)
            .optionally_bearer_auth(self.token.as_ref())
            .inspect()
            .optionally_json(body.as_ref())
            // Don't retry post!
            .send()
            .await?
            .error_for_status2()
            .await?
            .try_into()
            .inspect(|output| self.inspect(output))
    }

    async fn put<P, T>(&self, path: P) -> ApiResult<T>
    where
        P: fmt::Display,
        T: de::DeserializeOwned + ser::Serialize + fmt::Debug,
    {
        let url = self.url(path);
        self.client()?
            .put(url)
            .optionally_bearer_auth(self.token.as_ref())
            .inspect()
            .retry()
            .await?
            .error_for_status2()
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
    fn optionally_bearer_auth(self, token: Option<&SecretString>) -> Self;
    fn optionally_json<T>(self, body: Option<&T>) -> Self
    where
        T: ser::Serialize;

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
    fn optionally_bearer_auth(self, token: Option<&SecretString>) -> Self {
        let token = token.map(ExposeSecret::expose_secret);
        self.optionally(token, Self::bearer_auth)
    }

    fn optionally_json<T>(self, body: Option<&T>) -> Self
    where
        T: ser::Serialize,
    {
        self.optionally(body, Self::json)
    }
}

trait Inspector {
    fn inspect(self) -> Self;
}

impl Inspector for reqwest::RequestBuilder {
    fn inspect(self) -> Self {
        if let Some(request) = self.try_clone().and_then(|builder| builder.build().ok()) {
            log::trace!("{} {}", request.method(), request.url());

            request.headers().iter().for_each(|(header, value)| {
                log::trace!("{}: {}", header, String::from_utf8_lossy(value.as_bytes()))
            });
        }

        self
    }
}

#[async_trait::async_trait]
trait Retry {
    async fn retry(self) -> reqwest::Result<reqwest::Response>;
}

#[async_trait::async_trait]
impl Retry for reqwest::RequestBuilder {
    async fn retry(self) -> reqwest::Result<reqwest::Response> {
        loop {
            if let Some(builder) = self.try_clone() {
                let response = builder.send().await?;
                if response.status().is_server_error() {
                    log::trace!("Retrying on server error {}", response.status());
                    continue;
                }
                break Ok(response);
            }
            break self.send().await;
        }
    }
}

#[async_trait::async_trait]
trait ResponseExt: Sized {
    async fn split_response(self) -> reqwest::Result<(reqwest::StatusCode, bytes::Bytes)>;
    async fn error_for_status2(self) -> anyhow::Result<bytes::Bytes> {
        let (status, bytes) = self.split_response().await?;

        log::trace!("{}", String::from_utf8_lossy(&bytes));

        if status.is_server_error() {
            anyhow::bail!("Server error {}", status)
        }

        if status.is_client_error() {
            let unknown_error = |_| v0::Error::unknown_error(status, &bytes);
            let err = json::from_slice::<v0::Error>(&bytes).unwrap_or_else(unknown_error);

            anyhow::bail!(err);
        }

        Ok(bytes)
    }
}

#[async_trait::async_trait]
impl ResponseExt for reqwest::Response {
    async fn split_response(self) -> reqwest::Result<(reqwest::StatusCode, bytes::Bytes)> {
        let status = self.status();
        let bytes = self.bytes().await?;
        Ok((status, bytes))
    }
}
