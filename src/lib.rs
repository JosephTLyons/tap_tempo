use chrono::{offset::Utc, DateTime};

pub struct TapTempo {
    start_datetime: Option<DateTime<Utc>>,
    tap_count: u128,
}

// Maybe use different types to enforce the different states of tapping (new, able to calc)
impl TapTempo {
    pub fn new() -> Self {
        Self {
            start_datetime: None,
            tap_count: 0,
        }
    }

    pub fn tap(&mut self) -> Option<f64> {
        self.tap_count += 1;

        if let Some(start_datetime) = self.start_datetime {
            return get_tempo(start_datetime, Utc::now(), self.tap_count);
        }

        self.start_datetime = Some(Utc::now());

        None
    }
}

impl Default for TapTempo {
    fn default() -> Self {
        Self::new()
    }
}

fn get_tempo(
    start_datetime: DateTime<Utc>,
    end_datetime: DateTime<Utc>,
    tap_count: u128,
) -> Option<f64> {
    if tap_count < 2 {
        return None;
    }

    let interval_count = tap_count - 1;

    let duration = end_datetime - start_datetime;
    let duration_in_minutes = duration.num_milliseconds() as f64 / 60_000.0;

    Some(interval_count as f64 / duration_in_minutes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, TimeZone};

    #[test]
    fn test_tap_tempo() {
        let mut tap_tempo = TapTempo::new();
        assert!(tap_tempo.start_datetime.is_none());
        assert_eq!(tap_tempo.tap_count, 0);

        let tempo = tap_tempo.tap();
        assert!(tap_tempo.start_datetime.is_some());
        assert_eq!(tap_tempo.tap_count, 1);
        assert!(tempo.is_none());

        let tempo = tap_tempo.tap();
        assert!(tap_tempo.start_datetime.is_some());
        assert_eq!(tap_tempo.tap_count, 2);
        assert!(tempo.is_some());
    }

    // Largely the same as above (redundant), but a bit stripped down for the README.md
    #[test]
    fn test_readme_example() {
        let mut tap_tempo = TapTempo::new();
        let tempo = tap_tempo.tap();
        assert!(tempo.is_none());

        // After some time has passed ...

        let tempo = tap_tempo.tap();
        assert!(tempo.is_some());
    }

    #[test]
    fn test_get_tempo_tap_count_zero() {
        let (start_datetime, end_datetime) = get_start_and_end_test_datetimes();
        let tempo = get_tempo(start_datetime, end_datetime, 0);
        assert_eq!(tempo, None)
    }

    #[test]
    fn test_get_tempo_tap_count_one() {
        let (start_datetime, end_datetime) = get_start_and_end_test_datetimes();
        let tempo = get_tempo(start_datetime, end_datetime, 1);
        assert_eq!(tempo, None)
    }

    #[test]
    fn test_get_tempo_tap_count_two() {
        let (start_datetime, end_datetime) = get_start_and_end_test_datetimes();
        let tempo = get_tempo(start_datetime, end_datetime, 2);
        assert_eq!(tempo, Some(60.0))
    }

    fn get_start_and_end_test_datetimes() -> (DateTime<Utc>, DateTime<Utc>) {
        let start_datetime = Utc.ymd(1990, 4, 12).and_hms_milli(0, 0, 0, 0);
        let end_datetime = start_datetime + Duration::seconds(1);
        (start_datetime, end_datetime)
    }
}
