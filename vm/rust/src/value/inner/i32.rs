pub type InnerNumber = i32;
pub type InnerValue = u32;

#[inline]
pub const fn box_cons(cons: InnerValue) -> InnerValue {
    cons << 1
}

#[inline]
pub const fn unbox_cons(cons: InnerValue) -> InnerValue {
    cons >> 1
}

#[inline]
pub const fn is_cons(value: InnerValue) -> bool {
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
pub const fn from_raw(raw: InnerValue) -> InnerNumber {
    raw as _
}

#[inline]
pub const fn to_raw(number: InnerNumber) -> InnerValue {
    number as _
}
