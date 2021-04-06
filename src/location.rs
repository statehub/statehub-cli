//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::str::FromStr;

use crate::v1;

#[derive(Debug)]
pub(crate) enum Location {
    Aws(v1::AwsRegion),
    Azure(v1::AzureRegion),
    // Gcp(v1::GcpRegion),
}

impl FromStr for Location {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let aws = text.parse::<v1::AwsRegion>();
        let azure = text.parse::<v1::AzureRegion>();
        // let gcp = text.parse::<v1::GcpRegion>();

        match (aws, azure) {
            (Ok(aws), Err(_)) => Ok(Self::Aws(aws)),
            (Err(_), Ok(azure)) => Ok(Self::Azure(azure)),
            (Ok(aws), Ok(azure)) => Err(format!(
                "Ambiguous region, use either {:#} or {:#}",
                aws, azure
            )),
            (Err(aws), Err(azure)) => {
                let aws = aws.into_inner();
                let _azure = azure.into_inner();
                Err(format!("I cannot parse location '{}'", aws))
            }
        }
    }
}

pub(crate) trait CloudLocation {
    const PREFIX: &'static str;

    fn as_str(&self) -> &'static str;
}
