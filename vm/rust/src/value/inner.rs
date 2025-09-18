use cfg_elif::item_feature;

item_feature!(if ("i32") {
    mod i32;
} else {
    mod i64;
});

item_feature!(if ("i32") {
    pub use self::i32::*;
} else {
    pub use self::i64::*;
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_unbox_cons() {
        assert_eq!(unbox_cons(box_cons(0)), 0);
        assert_eq!(unbox_cons(box_cons(1)), 1);
        assert_eq!(unbox_cons(box_cons(42)), 42);
    }

    #[test]
    fn check_cons() {
        assert!(is_cons(box_cons(0)));
        assert!(!is_cons(to_raw(from_number(0))));
    }

    #[test]
    fn convert_number() {
        assert_eq!(to_number(from_number(-42)), -42);
        assert_eq!(to_number(from_number(-1)), -1);
        assert_eq!(to_number(from_number(0)), 0);
        assert_eq!(to_number(from_number(1)), 1);
        assert_eq!(to_number(from_number(42)), 42);
    }

    #[test]
    fn convert_i64() {
        assert_eq!(to_i64(from_i64(-42)), -42);
        assert_eq!(to_i64(from_i64(-1)), -1);
        assert_eq!(to_i64(from_i64(0)), 0);
        assert_eq!(to_i64(from_i64(1)), 1);
        assert_eq!(to_i64(from_i64(42)), 42);
    }
}
