use chrono::{DateTime, Duration, Utc};

const LUNAR_CYCLE_DAYS: f64 = 29.53058867;
const REFERENCE_NEW_MOON_MILLIS: i64 = 947_116_800_000;
const MILLIS_PER_DAY: f64 = 86_400_000.0;

#[derive(Debug, Clone, PartialEq)]
pub struct LunarPhaseInfo {
    pub date: DateTime<Utc>,
    pub phase_name: &'static str,
    pub illumination: u8,
    pub days_in_cycle: f64,
}

pub struct Lunarphase;

impl Lunarphase {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate_phase(&self, date: DateTime<Utc>) -> LunarPhaseInfo {
        let days_since_reference =
            (date.timestamp_millis() - REFERENCE_NEW_MOON_MILLIS) as f64 / MILLIS_PER_DAY;
        let days_in_cycle = days_since_reference % LUNAR_CYCLE_DAYS;
        let cycle_progress = days_in_cycle / LUNAR_CYCLE_DAYS;
        let illumination = (((1.0 - (2.0 * std::f64::consts::PI * cycle_progress).cos()) / 2.0)
            * 100.0)
            .round() as u8;

        let phase_name = if days_in_cycle < 1.84 {
            "New Moon"
        } else if days_in_cycle < 7.38 {
            "Waxing Crescent"
        } else if days_in_cycle < 9.23 {
            "First Quarter"
        } else if days_in_cycle < 14.77 {
            "Waxing Gibbous"
        } else if days_in_cycle < 16.61 {
            "Full Moon"
        } else if days_in_cycle < 22.15 {
            "Waning Gibbous"
        } else if days_in_cycle < 23.99 {
            "Last Quarter"
        } else {
            "Waning Crescent"
        };

        LunarPhaseInfo {
            date,
            phase_name,
            illumination,
            days_in_cycle: (days_in_cycle * 10.0).round() / 10.0,
        }
    }

    pub fn predict_upcoming(&self, days_ahead: u32) -> Vec<LunarPhaseInfo> {
        let today = Utc::now();
        (0..=days_ahead)
            .map(|days| self.calculate_phase(today + Duration::days(days as i64)))
            .collect()
    }

    pub fn find_next_full_moon(&self) -> Option<LunarPhaseInfo> {
        self.predict_upcoming(365)
            .into_iter()
            .find(|prediction| prediction.phase_name == "Full Moon")
    }

    pub fn assess_night_mission(&self) -> &'static str {
        let illumination = self.calculate_phase(Utc::now()).illumination;

        if illumination > 70 {
            "Bright night - High visibility, use caution!"
        } else if illumination > 40 {
            "Moderate visibility - Standard mission conditions"
        } else if illumination > 10 {
            "Dark night - Excellent for stealth operations"
        } else {
            "Very dark night - Perfect for covert dumpster raids!"
        }
    }
}

impl Default for Lunarphase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn reference_date_is_a_new_moon() {
        let phase =
            Lunarphase::new().calculate_phase(Utc.with_ymd_and_hms(2000, 1, 6, 0, 0, 0).unwrap());

        assert_eq!(phase.phase_name, "New Moon");
        assert_eq!(phase.illumination, 0);
        assert_eq!(phase.days_in_cycle, 0.0);
    }

    #[test]
    fn phase_boundaries_match_the_original_calculator() {
        let calculator = Lunarphase::new();
        let reference = Utc.with_ymd_and_hms(2000, 1, 6, 0, 0, 0).unwrap();

        assert_eq!(
            calculator
                .calculate_phase(reference + Duration::days(8))
                .phase_name,
            "First Quarter"
        );
        assert_eq!(
            calculator
                .calculate_phase(reference + Duration::days(15))
                .phase_name,
            "Full Moon"
        );
    }
}
