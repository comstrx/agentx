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

    pub fn clamp ( value: i64, low: i64, high: i64 ) -> i64 {

        value.clamp(low.min(high), low.max(high))

    }

    pub fn clamp_f ( value: f64, low: f64, high: f64 ) -> f64 {

        value.clamp(low.min(high), low.max(high))

    }

    pub fn abs ( value: i64 ) -> i64 {

        value.saturating_abs()

    }

    pub fn sign ( value: i64 ) -> i64 {

        value.signum()

    }

    pub fn pow ( base: i64, exp: u32 ) -> i64 {

        base.saturating_pow(exp)

    }

    pub fn lerp ( from: f64, to: f64, t: f64 ) -> f64 {

        from + ( to - from ) * t

    }

    pub fn is_even ( value: i64 ) -> bool {

        value % 2 == 0

    }

    pub fn is_odd ( value: i64 ) -> bool {

        value % 2 != 0

    }

    pub fn percent ( part: f64, whole: f64 ) -> f64 {

        if whole == 0.0 { 0.0 } else { part / whole * 100.0 }

    }

    pub fn round ( value: f64, places: u32 ) -> f64 {

        let factor = 10f64.powi(places as i32);
        ( value * factor ).round() / factor

    }

    pub fn gcd ( mut a: u64, mut b: u64 ) -> u64 {

        while b != 0 {

            let rest = a % b;

            a = b;
            b = rest;

        }

        a

    }

    pub fn lcm ( a: u64, b: u64 ) -> u64 {

        if a == 0 || b == 0 { 0 } else { a / Self::gcd(a, b) * b }

    }

    pub fn sum ( values: &[i64] ) -> i64 {

        values.iter().sum()

    }

    pub fn mean ( values: &[f64] ) -> f64 {

        if values.is_empty() { 0.0 } else { values.iter().sum::<f64>() / values.len() as f64 }

    }

    pub fn human_bytes ( bytes: u64 ) -> String {

        const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];

        let mut value = bytes as f64;
        let mut unit = 0;

        while value >= 1024.0 && unit < UNITS.len() - 1 {

            value /= 1024.0;
            unit += 1;

        }

        if unit == 0 { format!("{bytes} B") } else { format!("{value:.1} {}", UNITS[unit]) }

    }

    pub fn product ( values: &[i64] ) -> i64 {

        values.iter().product()

    }

    pub fn min_f ( values: &[f64] ) -> Option<f64> {

        values.iter().copied().reduce(f64::min)

    }

    pub fn max_f ( values: &[f64] ) -> Option<f64> {

        values.iter().copied().reduce(f64::max)

    }

    pub fn median ( values: &[f64] ) -> f64 {

        if values.is_empty() { return 0.0; }

        let mut sorted = values.to_vec();
        sorted.sort_by(f64::total_cmp);

        let mid = sorted.len() / 2;

        if sorted.len() % 2 == 1 { sorted[mid] }
        else { ( sorted[mid - 1] + sorted[mid] ) / 2.0 }

    }

    pub fn factorial ( n: u64 ) -> u64 {

        ( 1..=n ).fold(1u64, |acc, value| acc.saturating_mul(value))

    }

    pub fn is_prime ( n: u64 ) -> bool {

        if n < 2 { return false; }
        if n.is_multiple_of(2) { return n == 2; }

        let mut divisor = 3;

        while divisor <= n / divisor {

            if n.is_multiple_of(divisor) { return false; }
            divisor += 2;

        }

        true

    }

    pub fn is_power_of_two ( n: u64 ) -> bool {

        n.is_power_of_two()

    }

    pub fn map_range ( value: f64, from: ( f64, f64 ), to: ( f64, f64 ) ) -> f64 {

        let ( in_min, in_max ) = from;
        let ( out_min, out_max ) = to;

        if in_max == in_min { return out_min; }

        out_min + ( value - in_min ) * ( out_max - out_min ) / ( in_max - in_min )

    }

}
