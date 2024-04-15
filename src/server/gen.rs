    use chrono::{Duration, Local, NaiveDateTime};
    use rand::distributions::Alphanumeric;
    use rand::distributions::Distribution;
    use rand::thread_rng;
    use std::io::Error;
    /// generate generates random string pwd of length 8
    pub fn generate() -> Result<String, Error> {
        let mut ring = thread_rng();
        let s: String = Alphanumeric
            .sample_iter(&mut ring)
            .take(8)
            .map(char::from)
            .collect();
        Ok(s)
    }
    /// utc_expiry_time gets system time in utc and
    /// add 20 minutes to current time
    pub fn utc_expiry_time() -> Option<NaiveDateTime> {
        Local::now()
            .naive_utc()
            .checked_add_signed(Duration::minutes(20))
    }

