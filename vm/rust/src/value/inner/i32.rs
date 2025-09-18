pub type InnerNumber = i32;

#[inline]
pub const fn box_cons(cons: u32) -> u32 {
    cons << 1
}

#[inline]
pub const fn unbox_cons(cons: u32) -> u32 {
    cons >> 1
}

#[inline]
pub const fn is_cons(value: u32) -> bool {
    value & 1 == 0
}

#[inline]
pub const fn from_number(number: InnerNumber) -> InnerNumber {
    (number << 1) | 1
}

#[inline]
pub const fn to_number(number: InnerNumber) -> InnerNumber {
    number >> 1
}

#[inline]
pub const fn from_i64(number: i64) -> InnerNumber {
    from_number(number as _)
}

#[inline]
pub const fn to_i64(number: InnerNumber) -> i64 {
    to_number(number) as _
}

#[inline]
pub const fn from_raw(raw: u32) -> InnerNumber {
    raw as _
}

#[inline]
pub const fn to_raw(number: InnerNumber) -> u32 {
    number as _
}
