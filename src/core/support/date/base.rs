use jiff::{Timestamp, Zoned};

use super::arch::Date;

impl Date {

    #[inline]
    pub fn stamp () -> String {

        Zoned::now().strftime("%Y-%m-%d").to_string()

    }

    #[inline]
    pub fn compact () -> String {

        Zoned::now().strftime("%Y%m%d").to_string()

    }

    #[inline]
    pub fn datetime () -> String {

        Zoned::now().strftime("%Y-%m-%d %H:%M:%S").to_string()

    }

    #[inline]
    pub fn time () -> String {

        Zoned::now().strftime("%H:%M:%S").to_string()

    }

    #[inline]
    pub fn rfc3339 () -> String {

        Timestamp::now().to_string()

    }

    #[inline]
    pub fn format ( pattern: impl AsRef<str> ) -> String {

        Zoned::now().strftime(pattern.as_ref()).to_string()

    }

    #[inline]
    pub fn unix () -> i64 {

        Timestamp::now().as_second()

    }

    #[inline]
    pub fn unix_millis () -> i64 {

        Timestamp::now().as_millisecond()

    }

    pub fn year () -> i64 {

        Zoned::now().year() as i64

    }

    pub fn month () -> i64 {

        Zoned::now().month() as i64

    }

    pub fn day () -> i64 {

        Zoned::now().day() as i64

    }

    pub fn hour () -> i64 {

        Zoned::now().hour() as i64

    }

    pub fn minute () -> i64 {

        Zoned::now().minute() as i64

    }

    pub fn second () -> i64 {

        Zoned::now().second() as i64

    }

    pub fn weekday () -> String {

        Zoned::now().strftime("%A").to_string()

    }

    #[inline]
    pub fn unix_micros () -> i64 {

        Timestamp::now().as_microsecond()

    }

    pub fn day_of_year () -> i64 {

        Zoned::now().day_of_year() as i64

    }

    pub fn is_leap_year ( year: i64 ) -> bool {

        ( year % 4 == 0 && year % 100 != 0 ) || year % 400 == 0

    }

}
