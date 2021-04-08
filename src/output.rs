//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::convert::TryFrom;
use std::{fmt, ops};

use attohttpc::{header::HeaderMap, StatusCode};
use serde::{de, ser};
use serde_json as json;

use crate::show::Show;

#[derive(Debug)]
pub(crate) struct Output<T> {
    status: StatusCode,
    headers: HeaderMap,
    body: Body<T>,
}

#[derive(Debug)]
enum Body<T> {
    Raw(Vec<u8>),
    Typed(T),
    Value(json::Value),
}

impl<T> Output<T>
where
    T: de::DeserializeOwned + ser::Serialize,
{
    pub(crate) fn todo() -> Self {
        let status = StatusCode::OK;
        let headers = HeaderMap::new();
        let raw = String::from("NOT IMPLEMENTED YET").into_bytes();
        let body = Body::<T>::Raw(raw);
        Self {
            status,
            headers,
            body,
        }
    }

    pub(crate) fn _is_typed(&self) -> bool {
        matches!(self.body, Body::Typed(_))
    }

    pub(crate) fn into_typed(self) -> Self {
        let body = self.body.into_typed();
        Self { body, ..self }
    }

    pub(crate) fn into_value(self) -> Self {
        let body = self.body.into_value();
        Self { body, ..self }
    }
}

impl<T> Body<T>
where
    T: de::DeserializeOwned + ser::Serialize,
{
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
        // if let Self::Raw(raw) = self {
        //     match json::from_slice(&raw) {
        //         Ok(value) => Self::Value(value),
        //         Err(_) => Self::Raw(raw),
        //     }
        // } else {
        //     self
        // }
    }
}

impl<T> TryFrom<attohttpc::Response> for Output<T> {
    type Error = attohttpc::Error;

    fn try_from(response: attohttpc::Response) -> Result<Self, Self::Error> {
        let (status, headers, reader) = response.split();
        let bytes = reader.bytes()?;
        let body = Body::Raw(bytes);
        Ok(Self {
            status,
            headers,
            body,
        })
    }
}

impl<T> Show for Output<T>
where
    T: Show,
{
    fn show(self) -> String {
        self.body.show()
    }
}

impl<T> Show for Body<T>
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

// impl IntoOutput for attohttpc::Response {
//     type Err = attohttpc::Error;
//     fn into_output<T>(self, raw: bool, json: bool) -> Result<Output<T>, <Self as IntoOutput>::Err>
//     where
//         T: de::DeserializeOwned + fmt::Debug,
//     {
//         if raw {
//             self.text().map(Output::Raw)
//         } else if json {
//             self.json().map(Output::Value)
//         } else {
//             self.json().map(Output::Typed)
//         }
//     }
// }

// impl<T> Into<T> for Output<T>
// where
//     T: de::DeserializeOwned,
// {
//     fn into(self) -> T {
//         match self {
//             Output::Raw(text) => todo!(),
//             Output::Typed(data) => data,
//         }
//     }
// }

impl<T> ops::Deref for Output<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.body.deref()
    }
}

impl<T> ops::Deref for Body<T> {
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
        match self.body {
            Body::Raw(_) => panic!("IntoIterator is not implemented for Output::Raw"),
            Body::Typed(data) => data.into_iter(),
            Body::Value(_) => panic!("IntoIterator is not implemented for Output::Value"),
        }
    }
}
