//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::{fmt, ops};

use bytes::Bytes;
use serde::{de, ser};
use serde_json as json;

use crate::show::Show;

/// `Output` wraps different form of data returned by the API
#[derive(Debug)]
pub(crate) enum Output<T> {
    Raw(Bytes),
    Typed(T),
    Value(json::Value),
}

impl<T> Output<T>
where
    T: de::DeserializeOwned + ser::Serialize,
{
    pub(crate) fn todo() -> Self {
        let text = Bytes::from("NOT IMPLEMENTED YET");
        Self::Raw(text)
    }

    pub(crate) fn into_typed(self) -> Self {
        if let Self::Raw(raw) = self {
            match json::from_slice(&raw) {
                Ok(typed) => Self::Typed(typed),
                Err(_) => Self::Raw(raw),
            }
        } else {
            self
        }
    }

    pub(crate) fn into_value(self) -> Self {
        match self {
            Self::Raw(raw) => match json::from_slice(&raw) {
                Ok(value) => Self::Value(value),
                Err(_) => Self::Raw(raw),
            },
            Self::Typed(typed) => match json::to_value(&typed) {
                Ok(value) => Self::Value(value),
                Err(_) => Self::Typed(typed),
            },
            Self::Value(value) => Self::Value(value),
        }
    }
}

impl<T> From<Bytes> for Output<T> {
    fn from(bytes: Bytes) -> Self {
        Self::Raw(bytes)
    }
}

impl<T> Show for Output<T>
where
    T: Show,
{
    fn show(self) -> String {
        match self {
            Self::Raw(text) => String::from_utf8_lossy(&text).into_owned(),
            Self::Typed(data) => data.show(),
            Self::Value(value) => value.to_string(),
        }
    }
}

pub(crate) trait IntoOutput {
    type Err;
    fn into_output<T>(self, raw: bool, json: bool) -> Result<Output<T>, Self::Err>
    where
        T: de::DeserializeOwned + fmt::Debug;
}

impl<T> ops::Deref for Output<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Raw(_) => panic!("Deref is not implemented for Body::Raw"),
            Self::Typed(data) => data,
            Self::Value(_) => panic!("Deref is not implemented for Body::Value"),
        }
    }
}

impl<T> IntoIterator for Output<T>
where
    T: IntoIterator,
{
    type Item = <T as IntoIterator>::Item;
    type IntoIter = <T as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Raw(_) => panic!("IntoIterator is not implemented for Output::Raw"),
            Self::Typed(data) => data.into_iter(),
            Self::Value(_) => panic!("IntoIterator is not implemented for Output::Value"),
        }
    }
}
