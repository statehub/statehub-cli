//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::ops;

use serde::{de::DeserializeOwned, Serialize};
use serde_json as json;

use crate::traits::Show;

pub(crate) struct Quiet<T>(pub(crate) T)
where
    T: DeserializeOwned + Serialize + Show;

impl<T> Quiet<T>
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
            String::new()
        }
    }
}

impl<T> ops::Deref for Quiet<T>
where
    T: ops::Deref + DeserializeOwned + Serialize + Show,
{
    type Target = T::Target;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
