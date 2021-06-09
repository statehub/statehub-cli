pub(crate) trait CloudRegion {
    const VENDOR: &'static str;
    const VENDOR_PREFIX: &'static str;
    fn as_str(&self) -> &'static str;
}

pub(crate) trait Show {
    fn show(&self) -> String;
}
