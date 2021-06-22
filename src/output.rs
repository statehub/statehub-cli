//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::convert::TryFrom;
use std::{fmt, ops};

use bytes::Bytes;
use serde::{de, Deserialize, Serialize};
use serde_json as json;

use crate::traits::Show;

/// `Output` wraps different form of data returned by the API
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Output<T>(T);

impl<T> Output<T>
where
    T: de::DeserializeOwned + Serialize,
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

    pub(crate) fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> Show for Output<T>
where
    T: Show,
{
    fn show(&self) -> String {
        self.0.show()
    }

    fn detailed_show(&self) -> String {
        self.0.detailed_show()
    }
}

impl<T> TryFrom<Bytes> for Output<T>
where
    T: de::DeserializeOwned + Serialize,
{
    type Error = anyhow::Error;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        let bytes = if bytes.is_empty() {
            b"null"
        } else {
            bytes.as_ref()
        };
        let inner = json::from_slice(bytes)?;
        Ok(Self(inner))
    }
}

impl<T> fmt::Display for Output<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> ops::Deref for Output<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> IntoIterator for Output<T>
where
    T: IntoIterator,
{
    type Item = <T as IntoIterator>::Item;
    type IntoIter = <T as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
