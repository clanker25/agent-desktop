#[repr(C)]
pub struct AdOptionalU64 {
    pub value: u64,
    pub present: bool,
}

pub const AD_OPTIONAL_U64_SIZE: usize = 32;

const _: () = assert!(std::mem::size_of::<AdOptionalU64>() == AD_OPTIONAL_U64_SIZE);
