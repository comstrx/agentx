use super::arch::Num;

impl Num {

    pub fn parse_int ( text: &str ) -> Option<i64> {

        text.trim().parse().ok()

    }

    pub fn parse_uint ( text: &str ) -> Option<u64> {

        text.trim().parse().ok()

    }

    pub fn parse_float ( text: &str ) -> Option<f64> {

        text.trim().parse().ok()

    }

    /// Clamp `value` into `[low, high]`, tolerating a swapped range.
    pub fn clamp ( value: i64, low: i64, high: i64 ) -> i64 {

        value.clamp(low.min(high), low.max(high))

    }

}
