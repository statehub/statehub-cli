//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Helm {
    pub repo: String,
    pub chart: String,
    pub version: String,
    pub parameters: HashMap<String, String>,
}
