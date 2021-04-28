//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::fmt;
use std::str;

use super::*;

impl CloudRegion for GcpRegion {
    const VENDOR: &'static str = "GCP";
    const VENDOR_PREFIX: &'static str = "gcp/";

    fn as_str(&self) -> &'static str {
        match self {
            Self::Antarctica => "antarctica",
        }
    }
}

impl str::FromStr for GcpRegion {
    type Err = InvalidRegion;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let text = s.strip_prefix(Self::VENDOR_PREFIX).unwrap_or(s);
        match text {
            "antarctica" => Ok(Self::Antarctica),
            other => Err(InvalidRegion::new(Self::VENDOR, other)),
        }
    }
}

impl fmt::Display for GcpRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = self.as_str();
        if f.alternate() {
            format!("{}{}", Self::VENDOR_PREFIX, text).fmt(f)
        } else {
            text.fmt(f)
        }
    }
}
