use super::*;
mod impls;

#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, SerializeDisplay, DeserializeFromStr,
)]
pub enum GcpRegion {
    Antarctica,
}
