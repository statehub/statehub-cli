//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::ops;

use serde::{de::DeserializeOwned, Serialize};
use serde_json as json;

// use crate::output::Output;
use crate::traits::Show;

// use super::*;

pub(crate) struct Detailed<T>(pub(crate) T)
where
    T: DeserializeOwned + Serialize + Show;

impl<T> Detailed<T>
where
    T: DeserializeOwned + Serialize + Show,
{
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
            self.0.detailed_show()
        }
    }
}

impl<T> Show for Detailed<T>
where
    T: DeserializeOwned + Serialize + Show,
{
    fn show(&self) -> String {
        self.0.detailed_show()
    }

    fn detailed_show(&self) -> String {
        self.0.detailed_show()
    }
}

impl<T> ops::Deref for Detailed<T>
where
    T: ops::Deref + DeserializeOwned + Serialize + Show,
{
    type Target = T::Target;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
