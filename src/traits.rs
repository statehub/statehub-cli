//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

pub(crate) trait CloudRegion {
    const VENDOR: &'static str;
    const VENDOR_PREFIX: &'static str;
    fn as_str(&self) -> &'static str;
}

pub(crate) trait Show {
    fn show(&self) -> String;
    fn detailed_show(&self) -> String {
        self.show()
    }
}
