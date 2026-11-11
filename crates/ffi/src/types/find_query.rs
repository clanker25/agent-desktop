use crate::types::{AdFindControl, AdFindFilter};

#[repr(C)]
pub struct AdFindQuery {
    pub control: AdFindControl,
    pub filter: AdFindFilter,
}

pub const AD_FIND_QUERY_VERSION: u32 = 0;
pub const AD_FIND_QUERY_SIZE: usize = 111;

const _: () = assert!(std::mem::size_of::<AdFindQuery>() == AD_FIND_QUERY_SIZE);
