use super::Time;

impl Time {

    /// Today's civil date as `YYYY-MM-DD` in the system time zone.
    pub fn stamp () -> String {

        jiff::Zoned::now().strftime("%Y-%m-%d").to_string()

    }

    /// Full local timestamp, `YYYY-MM-DD HH:MM:SS`.
    pub fn datetime () -> String {

        jiff::Zoned::now().strftime("%Y-%m-%d %H:%M:%S").to_string()

    }

    /// Seconds since the Unix epoch.
    pub fn unix () -> i64 {

        jiff::Timestamp::now().as_second()

    }

}
