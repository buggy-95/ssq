use chrono::{DateTime, Datelike, Duration, Local, NaiveTime, Weekday};

pub fn is_outdated(date_str: &str) -> bool {
    let now = Local::now();
    return check_outdated(date_str, now);
}

fn check_outdated(date_str: &str, now: DateTime<Local>) -> bool {
    let date_str = &date_str[0..10];
    let today = now.date_naive();

    if date_str == today.to_string() { return false }

    let mut target_date = now.date_naive();
    let update_time = NaiveTime::from_hms_opt(21, 16, 0).unwrap();

    match now.weekday() {
        Weekday::Tue | Weekday::Thu | Weekday::Sun => {
            if now.time() > update_time { return true }
            else { target_date -= Duration::days(1) }
        }
        _ => {}
    }

    match target_date.weekday() {
        Weekday::Mon | Weekday::Wed | Weekday::Fri => target_date -= Duration::days(1),
        Weekday::Sat => target_date -= Duration::days(2),
        _ => {}
    }

    return target_date.to_string() != date_str;
}

#[cfg(test)]
mod test {
    use chrono::{NaiveDateTime, TimeZone};
    use super::*;

    #[test]
    fn outdated_test_should_success() {
        let test_cases = [
            ("2024-02-27", "2024-02-28 12:00:00", false),
            ("2024-02-27", "2024-02-29 12:00:00", false),
            ("2024-02-27", "2024-02-29 22:00:00", true),
            ("2024-02-29", "2024-03-01 12:00:00", false),
            ("2024-02-29", "2024-03-02 12:00:00", false),
            ("2024-02-29", "2024-03-03 12:00:00", false),
            ("2024-02-29", "2024-03-03 22:00:00", true),
            ("2024-02-27", "2024-03-01 12:00:00", true),
            ("2024-02-27", "2024-03-02 12:00:00", true),
            ("2024-02-27", "2024-03-03 12:00:00", true),
        ];
        for (last_day, now, result) in test_cases {
            let naive_datetime =NaiveDateTime::parse_from_str(
                now,
                "%Y-%m-%d %H:%M:%S",
            ).unwrap();
            let time = Local.from_local_datetime(&naive_datetime).unwrap();
            assert_eq!(result, check_outdated(last_day, time));
        }
    }
}
