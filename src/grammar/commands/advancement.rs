#[repr(u32)]
pub enum Args {
    Grant,
    Revoke,
}

#[repr(u32)]
pub enum SubArgs {
    Everything,
    From,
    Only,
    Through,
    Until,
}