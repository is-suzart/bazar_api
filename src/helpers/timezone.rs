use chrono::{DateTime, Utc};
use chrono_tz::Tz;

pub fn get_current_timezone () -> DateTime<Tz> {
    let timestamp_utc: DateTime<Utc> = Utc::now();
    let timezone_brazil: Tz = chrono_tz::America::Sao_Paulo;
    return timestamp_utc.with_timezone(&timezone_brazil);
}