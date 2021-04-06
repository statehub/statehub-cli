//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::borrow::Borrow;
use std::convert::TryInto;
use std::fmt;

use inspector::ResultInspector;
use serde::{de, ser};

use crate::output::Output;
// use crate::show::Show;
use crate::v1;

pub(crate) type ApiResult<T> = Result<Output<T>, anyhow::Error>;

#[derive(Debug)]
pub(crate) struct Api {
    base: String,
    token: Option<String>,
    json: bool,
    raw: bool,
    verbose: bool,
}

impl Api {
    pub(crate) fn new(
        management: String,
        token: Option<String>,
        json: bool,
        raw: bool,
        verbose: bool,
    ) -> Self {
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
        }
    }

    pub(crate) fn create_state(
        self,
        name: v1::StateName,
        owner: Option<v1::ClusterName>,
        locations: v1::Locations,
    ) -> anyhow::Result<()> {
        let body = v1::State {
            name,
            storage_class: None,
            owner,
            locations,
            allowed_clusters: None,
        };
        let output = self.post::<_, _, v1::State>("/states", body)?;
        let output = if self.json {
            output.into_value()
        } else if !self.raw {
            output.into_typed()
        } else {
            output
        };
        println!("{:?}", output);
        Ok(())
    }

    pub(crate) fn list_states(self) -> anyhow::Result<()> {
        self.get::<_, Vec<v1::State>>("/states")
            .map(|states| println!("{:?}", states))
    }

    pub(crate) fn list_clusters(self) -> anyhow::Result<()> {
        self.get::<_, Vec<v1::Cluster>>("/clusters")
            .map(|states| println!("{:?}", states))
    }

    pub(crate) fn register_cluster(self) -> anyhow::Result<()> {
        Ok(())
    }

    pub(crate) fn unregister_cluster(self) -> anyhow::Result<()> {
        Ok(())
    }

    pub(crate) fn create_volume(self) -> anyhow::Result<()> {
        Ok(())
    }

    pub(crate) fn delete_volume(self) -> anyhow::Result<()> {
        Ok(())
    }

    pub(crate) fn add_location(self) -> anyhow::Result<()> {
        Ok(())
    }

    pub(crate) fn remove_location(self) -> anyhow::Result<()> {
        Ok(())
    }

    pub(crate) fn set_availability(self) -> anyhow::Result<()> {
        Ok(())
    }

    pub(crate) fn set_owner(
        self,
        state: v1::StateName,
        cluster: v1::ClusterName,
    ) -> anyhow::Result<()> {
        let path = format!(
            "/states/{state}/owner/{cluster}",
            state = state,
            cluster = cluster,
        );
        self.put::<_, v1::State>(path)
            .map(|volume| println!("{:?}", volume))
    }

    pub(crate) fn unset_owner(
        self,
        state: v1::StateName,
        cluster: v1::ClusterName,
    ) -> anyhow::Result<()> {
        let path = format!(
            "/states/{state}/owner/{cluster}",
            state = state,
            cluster = cluster,
        );
        self.del::<_, Option<(&str, &str)>, &str, &str, v1::State>(path, None)
            .map(|volume| println!("{:?}", volume))
    }

    fn url(&self, path: impl fmt::Display) -> String {
        format!("{}{}", self.base, path)
    }

    // pub(crate) fn version(&self, _crates: bool) -> Result<String, anyhow::Error> {
    //     self.get_version().map(Show::show)
    // }

    // fn get_job(&self, job_id: impl fmt::Display) -> ApiResult<latest::Job> {
    //     self.get(format!("/internal/jobs/{}", job_id))
    // }

    // fn get_jobs(&self) -> ApiResult<Vec<latest::Job>> {
    //     self.get("/internal/jobs")
    // }

    // fn get_version(&self) -> ApiResult<latest::InternalVersion> {
    //     self.get("/internal/version")
    // }

    fn inspect<T>(&self, output: &Output<T>)
    where
        T: fmt::Debug,
    {
        if self.verbose {
            println!("{:#?}", output);
        }
    }

    fn del<P, I, K, V, T>(&self, path: P, params: I) -> ApiResult<T>
    where
        P: fmt::Display,
        T: de::DeserializeOwned + fmt::Debug,
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: ToString,
    {
        let url = self.url(path);
        let output = attohttpc::delete(url)
            .optionally_bearer_auth(self.token.as_ref())
            .params(params)
            .send()?
            .try_into()
            .inspect(|output| self.inspect(output))?;
        Ok(output)
    }

    fn get<P, T>(&self, path: P) -> ApiResult<T>
    where
        P: fmt::Display,
        T: de::DeserializeOwned + fmt::Debug,
    {
        let url = self.url(path);
        let output = attohttpc::get(url)
            .optionally_bearer_auth(self.token.as_ref())
            .send()?
            .try_into()
            .inspect(|output| self.inspect(output))?;

        Ok(output)
    }

    fn post<P, T, U>(&self, path: P, body: T) -> ApiResult<U>
    where
        P: fmt::Display,
        T: ser::Serialize,
        U: de::DeserializeOwned + fmt::Debug,
    {
        let url = self.url(path);
        let output = attohttpc::post(url)
            .optionally_bearer_auth(self.token.as_ref())
            .json(&body)?
            .send()?
            .try_into()
            .inspect(|output| self.inspect(output))?;

        Ok(output)
    }

    fn put<P, T>(&self, path: P) -> ApiResult<T>
    where
        P: fmt::Display,
        T: de::DeserializeOwned + fmt::Debug,
    {
        let url = format!("{}{}", self.base, path);
        let output = attohttpc::put(url)
            .optionally_bearer_auth(self.token.as_ref())
            .send()?
            .try_into()
            .inspect(|output| self.inspect(output))?;
        Ok(output)
    }
}

trait Optionally {
    fn optionally_bearer_auth(self, token: Option<impl Into<String>>) -> Self;

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

impl Optionally for attohttpc::RequestBuilder {
    fn optionally_bearer_auth(self, token: Option<impl Into<String>>) -> Self {
        self.optionally(token, |this, token| this.bearer_auth(token))
    }
}
