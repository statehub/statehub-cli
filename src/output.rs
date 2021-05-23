//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::convert::TryFrom;
use std::{fmt, ops};

use bytes::Bytes;
use serde::{de, ser};
use serde_json as json;

use crate::show::Show;

/// `Output` wraps different form of data returned by the API
#[derive(Debug)]
pub(crate) struct Output<T>(T);

impl<T> Output<T>
where
    T: de::DeserializeOwned + ser::Serialize,
{
    pub(crate) fn todo() -> Output<String> {
        let text = String::from("NOT IMPLEMENTED YET");
        Output(text)
    }

    pub(crate) fn into_text(self, json: bool) -> String
    where
        T: Show,
    {
        if json {
            match json::to_value(&self.0) {
                Ok(value) => value.to_string(),
                Err(_) => self.0.show(),
            }
        } else {
            self.0.show()
        }
    }

    pub(crate) fn try_from_bytes(bytes: Bytes) -> json::Result<Self> {
        log::debug!("received body: {}", String::from_utf8_lossy(&bytes));
        json::from_slice(&bytes).map(Output)
    }
}

impl<T> TryFrom<Bytes> for Output<T>
where
    T: de::DeserializeOwned + ser::Serialize,
{
    type Error = anyhow::Error;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        Self::try_from_bytes(bytes).map_err(anyhow::Error::from)
    }
}

impl<T> fmt::Display for Output<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = format!("{:?}", self.0);
        text.fmt(f)
    }
}

impl<T> ops::Deref for Output<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
