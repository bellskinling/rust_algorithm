pub trait GetMax {
    fn max_value() -> Self;
}

impl GetMax for i8 {
    fn max_value() -> i8 {
        i8::max_value()
    }
}

impl GetMax for  i16{
    fn max_value() -> i16 {
        i16::max_value()
    }
}

impl GetMax for i32 {
    fn max_value() -> i32 {
        i32::max_value()
    }
}

impl GetMax for i64 {
    fn max_value() -> i64 {
        i64::max_value()
    }
}

impl GetMax for u8 {
    fn max_value() -> u8 {
        u8::max_value()
    }
}

impl GetMax for u16 {
    fn max_value() -> u16 {
        u16::max_value()
    }
}

impl GetMax for u32 {
    fn max_value() -> u32 {
        u32::max_value()
    }
}

impl GetMax for u64 {
    fn max_value() -> u64 {
        u64::max_value()
    }
}

impl GetMax for f32 {
    fn max_value() -> f32 {
        f32::max_value()
    }
}

impl GetMax for f64 {
    fn max_value() -> f64 {
        f64::max_value()
    }
}
