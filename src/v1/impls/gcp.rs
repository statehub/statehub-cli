//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::fmt;
use std::str;

use thiserror::Error;

use super::*;

impl GcpRegion {
    pub(crate) const PREFIX: &'static str = "gcp:";

    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::Antarctica => "antarctica",
        }
    }
}

impl str::FromStr for GcpRegion {
    type Err = InvalidGcpRegion;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let text = s.strip_prefix(Self::PREFIX).unwrap_or(s);
        match text {
            "antarctica" => Ok(Self::Antarctica),
            other => Err(other.into()),
        }
    }
}

impl fmt::Display for GcpRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            format!("{}{}", Self::PREFIX, self.as_str()).fmt(f)
        } else {
            self.as_str().fmt(f)
        }
    }
}

#[derive(Debug, Error)]
#[error(r#"Invalid GCP Region "{0}""#)]
pub struct InvalidGcpRegion(String);

impl InvalidGcpRegion {
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<&str> for InvalidGcpRegion {
    fn from(region: &str) -> Self {
        let region = region.to_string();
        Self(region)
    }
}
