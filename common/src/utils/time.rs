use chrono::{TimeZone, Utc};

pub fn get_month_range(year: i32, month: u32) -> (i64, i64) {
    // その月の1日(0時0分0秒)を取得する
    let start = Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0);
    // 次の月の1日(0時0分0秒)を取得する
    let next_month = match month == 12 {
        true => Utc.with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0),
        false => Utc.with_ymd_and_hms(year, month + 1, 1, 0, 0, 0),
    };
    // unix timeを取得する
    let start_time = start.unwrap().timestamp_millis();
    let end_time = next_month.unwrap().timestamp_millis();
    (start_time, end_time)
}
