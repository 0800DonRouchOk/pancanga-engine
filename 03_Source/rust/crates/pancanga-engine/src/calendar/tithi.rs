//! Tithi at sunrise.
//!
//! This module determines the instantaneous astronomical tithi present at a
//! sunrise instant. It does not calculate tithi duration, tithi changes,
//! Ekadasi, Mahadvadasi, parana, or Vaishnava rules.

use crate::astronomy::{
    astronomical_tithi, lunar_solar_elongation, moon, solar, AstronomicalTithi,
};
use crate::core::JulianDate;

const SEARCH_STEPS: usize = 96;
const REFINEMENT_STEPS: usize = 48;

/// A tithi transition inside a sunrise-to-sunrise interval.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TithiTransition {
    instant: JulianDate,
    before: AstronomicalTithi,
    after: AstronomicalTithi,
}

impl TithiTransition {
    /// Creates a tithi transition from its instant and adjacent tithis.
    pub fn new(instant: JulianDate, before: AstronomicalTithi, after: AstronomicalTithi) -> Self {
        Self {
            instant,
            before,
            after,
        }
    }

    /// Returns the Julian Date of the transition.
    pub fn instant(self) -> JulianDate {
        self.instant
    }

    /// Returns the tithi before the transition.
    pub fn before(self) -> AstronomicalTithi {
        self.before
    }

    /// Returns the tithi after the transition.
    pub fn after(self) -> AstronomicalTithi {
        self.after
    }
}

/// Objective tithi presence between one sunrise and the next.
///
/// This type describes calendar facts only. It does not decide which tithi
/// "counts" for the day and does not apply Vaishnava rules.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CivilDayTithiPresence {
    tithi_at_sunrise: AstronomicalTithi,
    transition: Option<TithiTransition>,
}

impl CivilDayTithiPresence {
    /// Creates a civil-day tithi presence record.
    pub fn new(tithi_at_sunrise: AstronomicalTithi, transition: Option<TithiTransition>) -> Self {
        Self {
            tithi_at_sunrise,
            transition,
        }
    }

    /// Returns the tithi present at sunrise.
    pub fn tithi_at_sunrise(self) -> AstronomicalTithi {
        self.tithi_at_sunrise
    }

    /// Returns the first tithi transition inside the civil day, if one exists.
    pub fn transition(self) -> Option<TithiTransition> {
        self.transition
    }

    /// Returns the tithi after the transition, if one exists.
    pub fn tithi_after_transition(self) -> Option<AstronomicalTithi> {
        self.transition.map(TithiTransition::after)
    }
}

/// Returns the astronomical tithi present at sunrise.
///
/// The input is the [`JulianDate`] of sunrise. This function consumes only the
/// public astronomy API:
///
/// ```text
/// sunrise JulianDate
///     -> solar apparent longitude
///     -> lunar apparent longitude
///     -> lunar-solar elongation
///     -> astronomical tithi
/// ```
///
/// It does not apply calendar or Vaishnava rule interpretation.
pub fn tithi_at_sunrise(sunrise: JulianDate) -> AstronomicalTithi {
    astronomical_tithi_at(sunrise)
}

/// Returns the objective tithi presence between one sunrise and the next.
///
/// This function describes what happened in the civil day:
///
/// - which tithi was present at sunrise;
/// - whether a tithi transition occurred before the next sunrise;
/// - which tithi was present after that transition.
///
/// It does not decide which tithi belongs to the day and does not apply
/// Vaishnava rules.
pub fn tithi_presence_between_sunrises(
    sunrise_today: JulianDate,
    sunrise_tomorrow: JulianDate,
) -> Option<CivilDayTithiPresence> {
    if sunrise_tomorrow.value() <= sunrise_today.value() {
        return None;
    }

    Some(CivilDayTithiPresence::new(
        tithi_at_sunrise(sunrise_today),
        tithi_transition_between_sunrises(sunrise_today, sunrise_tomorrow),
    ))
}

/// Returns the first tithi transition between two consecutive sunrises.
///
/// The interval is treated as `[sunrise_today, sunrise_tomorrow)`.
///
/// This function only detects the astronomical event. It does not decide which
/// tithi belongs to the civil day and does not apply Vaishnava rules.
pub fn tithi_transition_between_sunrises(
    sunrise_today: JulianDate,
    sunrise_tomorrow: JulianDate,
) -> Option<TithiTransition> {
    if sunrise_tomorrow.value() <= sunrise_today.value() {
        return None;
    }

    let start_tithi = astronomical_tithi_at(sunrise_today);
    let interval = sunrise_tomorrow.value() - sunrise_today.value();
    let step = interval / SEARCH_STEPS as f64;
    let mut previous_instant = sunrise_today;
    let mut previous_tithi = start_tithi;

    for step_index in 1..=SEARCH_STEPS {
        let current_instant = JulianDate::new(sunrise_today.value() + (step * step_index as f64));
        let current_tithi = astronomical_tithi_at(current_instant);

        if current_tithi.index() != previous_tithi.index() {
            return Some(refine_transition(
                previous_instant,
                current_instant,
                previous_tithi,
                current_tithi,
            ));
        }

        previous_instant = current_instant;
        previous_tithi = current_tithi;
    }

    None
}

fn astronomical_tithi_at(jd: JulianDate) -> AstronomicalTithi {
    let sun_longitude = solar::apparent_longitude(jd);
    let moon_longitude = moon::apparent_longitude(jd);
    let elongation = lunar_solar_elongation(moon_longitude, sun_longitude);

    astronomical_tithi(elongation)
}

fn refine_transition(
    low: JulianDate,
    high: JulianDate,
    before: AstronomicalTithi,
    after: AstronomicalTithi,
) -> TithiTransition {
    let mut low_value = low.value();
    let mut high_value = high.value();

    for _ in 0..REFINEMENT_STEPS {
        let midpoint = JulianDate::new((low_value + high_value) / 2.0);
        let midpoint_tithi = astronomical_tithi_at(midpoint);

        if midpoint_tithi.index() == before.index() {
            low_value = midpoint.value();
        } else {
            high_value = midpoint.value();
        }
    }

    TithiTransition::new(JulianDate::new(high_value), before, after)
}

#[cfg(test)]
mod tests {
    use crate::calendar::{
        sunrise, tithi_at_sunrise, tithi_presence_between_sunrises,
        tithi_transition_between_sunrises,
    };
    use crate::core::{CivilDate, GeoLocation, JulianDate, Latitude, Longitude};

    const TOLERANCE_DAYS: f64 = 1.0e-8;

    #[test]
    fn calculates_tithi_at_equatorial_equinox_sunrise() {
        let tithi = tithi_at_sunrise(JulianDate::new(2_461_119.753_038_483));

        assert_eq!(tithi.index(), 1);
    }

    #[test]
    fn calculates_tithi_at_new_york_summer_sunrise() {
        let tithi = tithi_at_sunrise(JulianDate::new(2_461_212.892_268_492));

        assert_eq!(tithi.index(), 6);
    }

    #[test]
    fn calculates_tithi_at_buenos_aires_winter_sunrise() {
        let tithi = tithi_at_sunrise(JulianDate::new(2_461_212.958_473_451_4));

        assert_eq!(tithi.index(), 7);
    }

    #[test]
    fn calculates_tithi_at_sydney_summer_sunrise() {
        let tithi = tithi_at_sunrise(JulianDate::new(2_461_396.278_178_977));

        assert_eq!(tithi.index(), 12);
    }

    #[test]
    fn finds_transition_after_equatorial_equinox_sunrise() {
        let transition = transition_for(date(2026, 3, 20), date(2026, 3, 21), location(0.0, 0.0))
            .expect("transition should exist");

        assert_transition(transition.instant().value(), 2_461_120.376_724_702, 1, 2);
    }

    #[test]
    fn finds_transition_after_new_york_summer_sunrise() {
        let transition = transition_for(
            date(2026, 6, 21),
            date(2026, 6, 22),
            location(40.7128, -74.0060),
        )
        .expect("transition should exist");

        assert_transition(transition.instant().value(), 2_461_212.911_309_726, 6, 7);
    }

    #[test]
    fn finds_transition_after_buenos_aires_winter_sunrise() {
        let transition = transition_for(
            date(2026, 6, 21),
            date(2026, 6, 22),
            location(-34.6037, -58.3816),
        )
        .expect("transition should exist");

        assert_transition(transition.instant().value(), 2_461_213.924_739_889_3, 7, 8);
    }

    #[test]
    fn finds_transition_after_sydney_summer_sunrise() {
        let transition = transition_for(
            date(2026, 12, 21),
            date(2026, 12, 22),
            location(-33.8688, 151.2093),
        )
        .expect("transition should exist");

        assert_transition(transition.instant().value(), 2_461_396.871_763_413, 12, 13);
    }

    #[test]
    fn returns_none_for_invalid_interval() {
        assert_eq!(
            tithi_transition_between_sunrises(
                JulianDate::new(2_461_120.0),
                JulianDate::new(2_461_119.0),
            ),
            None,
        );
    }

    #[test]
    fn describes_presence_with_no_transition() {
        let start = JulianDate::new(2_461_120.404_176_93);
        let end = JulianDate::new(2_461_120.5);

        let presence =
            tithi_presence_between_sunrises(start, end).expect("interval should be valid");

        assert_eq!(presence.tithi_at_sunrise().index(), 2);
        assert_eq!(presence.transition(), None);
        assert_eq!(presence.tithi_after_transition(), None);
    }

    #[test]
    fn describes_presence_with_transition() {
        let presence = presence_for(date(2026, 3, 20), date(2026, 3, 21), location(0.0, 0.0))
            .expect("presence should exist");
        let transition = presence.transition().expect("transition should exist");

        assert_eq!(presence.tithi_at_sunrise().index(), 1);
        assert_eq!(transition.before().index(), 1);
        assert_eq!(transition.after().index(), 2);
        assert_eq!(
            presence.tithi_after_transition().map(|tithi| tithi.index()),
            Some(2)
        );
        assert!((transition.instant().value() - 2_461_120.376_724_702).abs() <= TOLERANCE_DAYS);
    }

    #[test]
    fn describes_presence_for_representative_locations() {
        let examples = [
            (
                date(2026, 6, 21),
                date(2026, 6, 22),
                location(40.7128, -74.0060),
                6,
                7,
            ),
            (
                date(2026, 6, 21),
                date(2026, 6, 22),
                location(-34.6037, -58.3816),
                7,
                8,
            ),
            (
                date(2026, 12, 21),
                date(2026, 12, 22),
                location(-33.8688, 151.2093),
                12,
                13,
            ),
        ];

        for (start_date, next_date, location, before, after) in examples {
            let presence =
                presence_for(start_date, next_date, location).expect("presence should exist");
            let transition = presence.transition().expect("transition should exist");

            assert_eq!(presence.tithi_at_sunrise().index(), before);
            assert_eq!(transition.before().index(), before);
            assert_eq!(transition.after().index(), after);
        }
    }

    #[test]
    fn returns_none_for_invalid_presence_interval() {
        assert_eq!(
            tithi_presence_between_sunrises(
                JulianDate::new(2_461_120.0),
                JulianDate::new(2_461_119.0),
            ),
            None,
        );
    }

    fn assert_transition(actual_instant: f64, expected_instant: f64, before: u8, after: u8) {
        assert!(
            (actual_instant - expected_instant).abs() <= TOLERANCE_DAYS,
            "expected transition JD {}, got JD {}",
            expected_instant,
            actual_instant,
        );

        let transition = tithi_transition_between_sunrises(
            JulianDate::new(actual_instant - 0.01),
            JulianDate::new(actual_instant + 0.01),
        )
        .expect("transition should exist");

        assert_eq!(transition.before().index(), before);
        assert_eq!(transition.after().index(), after);
    }

    fn transition_for(
        start_date: CivilDate,
        next_date: CivilDate,
        location: GeoLocation,
    ) -> Option<crate::calendar::TithiTransition> {
        let start = sunrise(start_date, location)?;
        let next = sunrise(next_date, location)?;

        tithi_transition_between_sunrises(start, next)
    }

    fn presence_for(
        start_date: CivilDate,
        next_date: CivilDate,
        location: GeoLocation,
    ) -> Option<crate::calendar::CivilDayTithiPresence> {
        let start = sunrise(start_date, location)?;
        let next = sunrise(next_date, location)?;

        tithi_presence_between_sunrises(start, next)
    }

    fn date(year: i32, month: u8, day: u8) -> CivilDate {
        CivilDate::new(year, month, day).expect("date should be valid")
    }

    fn location(latitude: f64, longitude: f64) -> GeoLocation {
        let latitude = Latitude::from_degrees(latitude).expect("latitude should be valid");
        let longitude = Longitude::from_degrees(longitude).expect("longitude should be valid");

        GeoLocation::new(latitude, longitude)
    }
}
