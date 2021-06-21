//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub hostname: String,
}
