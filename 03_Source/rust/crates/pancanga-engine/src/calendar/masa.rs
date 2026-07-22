//! Provisional Vaiṣṇava Māsa calculation.
//!
//! This module implements Campaign 51.0 from the provisional KB-CAL-001 /
//! KB-CAL-002 rule:
//!
//! ```text
//! pūrṇimānta lunar month
//! māsa by saṅkrānti inside the lunar month
//! Adhika Māsa = 0 saṅkrāntis
//! Kṣaya Māsa = 2 saṅkrāntis
//! ```
//!
//! The ayanāṁśa model remains pending certification and is isolated in
//! [`VaishnavaMasaConfig`].

use crate::astronomy::{lunar_solar_elongation, moon, solar};
use crate::core::math::normalize_360;
use crate::core::{Degrees, DurationDays, JulianDate};

const HALF_TURN_DEGREES: f64 = 180.0;
const SIGN_DEGREES: f64 = 30.0;
const SEARCH_STEP_DAYS: f64 = 0.25;
const BOUNDARY_REFINEMENT_STEPS: usize = 48;
const MAX_LUNAR_MONTH_SEARCH_DAYS: f64 = 35.0;
const MAX_SANKRANTI_SEARCH_DAYS: f64 = 35.0;
const PROVISIONAL_LAHIRI_J2000_DEGREES: f64 = 23.856_75;
const PROVISIONAL_AYANAMSA_ARCSECONDS_PER_CENTURY: f64 = 5_029.096_6;
const ARCSECONDS_PER_DEGREE: f64 = 3_600.0;
const J2000: f64 = 2_451_545.0;
const DAYS_PER_JULIAN_CENTURY: f64 = 36_525.0;

/// Lunar month identifier used by the Vaiṣṇava calendar and observance catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaishnavaMasa {
    Chaitra,
    Vaisakha,
    Jyestha,
    Asadha,
    Sravana,
    Bhadrapada,
    Asvina,
    Kartika,
    Margasirsa,
    Pausa,
    Magha,
    Phalguna,
}

impl VaishnavaMasa {
    /// Display label used by APIs and RC1.
    pub fn label(self) -> &'static str {
        match self {
            Self::Chaitra => "Chaitra",
            Self::Vaisakha => "Vaiśākha",
            Self::Jyestha => "Jyeṣṭha",
            Self::Asadha => "Āṣāḍha",
            Self::Sravana => "Śrāvaṇa",
            Self::Bhadrapada => "Bhādrapada",
            Self::Asvina => "Āśvina",
            Self::Kartika => "Kārtika",
            Self::Margasirsa => "Mārgaśīrṣa",
            Self::Pausa => "Pauṣa",
            Self::Magha => "Māgha",
            Self::Phalguna => "Phālguna",
        }
    }
}

/// Provisional month classification under KB-CAL-001 / KB-CAL-002.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaishnavaMasaType {
    Normal,
    Adhika,
    Ksaya,
}

impl VaishnavaMasaType {
    /// Stable diagnostic label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Normal => "normal_masa",
            Self::Adhika => "adhika_masa",
            Self::Ksaya => "ksaya_masa",
        }
    }
}

/// Diagnostic status for the ayanāṁśa used by native māsa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AyanamsaStatus {
    /// The value is the provisional Lahiri-style default from Campaign 51.0.
    Provisional,

    /// The caller supplied the value explicitly.
    Explicit,
}

impl AyanamsaStatus {
    /// Stable diagnostic label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Provisional => "provisional",
            Self::Explicit => "explicit",
        }
    }
}

/// Configuration for provisional Vaiṣṇava Māsa calculation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VaishnavaMasaConfig {
    ayanamsa_at_j2000: Degrees,
    ayanamsa_status: AyanamsaStatus,
}

impl VaishnavaMasaConfig {
    /// Provisional Lahiri-style default.
    ///
    /// The exact reference ayanāṁśa remains pending certification in
    /// Campaign 50.4. The value is isolated here so it can be replaced without
    /// changing the māsa algorithm.
    pub fn provisional_lahiri() -> Self {
        Self {
            ayanamsa_at_j2000: Degrees::new(PROVISIONAL_LAHIRI_J2000_DEGREES),
            ayanamsa_status: AyanamsaStatus::Provisional,
        }
    }

    /// Creates a configuration with an explicit J2000 ayanāṁśa value.
    pub fn explicit(ayanamsa_at_j2000: Degrees) -> Self {
        Self {
            ayanamsa_at_j2000,
            ayanamsa_status: AyanamsaStatus::Explicit,
        }
    }

    /// Returns the diagnostic ayanāṁśa status.
    pub fn ayanamsa_status(self) -> AyanamsaStatus {
        self.ayanamsa_status
    }
}

impl Default for VaishnavaMasaConfig {
    fn default() -> Self {
        Self::provisional_lahiri()
    }
}

/// Native Vaiṣṇava Māsa result exposed by the Calendar Engine.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VaishnavaMasaInfo {
    masa: VaishnavaMasa,
    masa_type: VaishnavaMasaType,
    sankranti_count: u8,
    month_start: JulianDate,
    month_end: JulianDate,
    ayanamsa_status: AyanamsaStatus,
}

impl VaishnavaMasaInfo {
    /// Returns the named Vaiṣṇava Māsa.
    pub fn masa(self) -> VaishnavaMasa {
        self.masa
    }

    /// Returns whether this is normal, adhika, or kṣaya.
    pub fn masa_type(self) -> VaishnavaMasaType {
        self.masa_type
    }

    /// Returns true for Adhika Māsa.
    pub fn is_adhika(self) -> bool {
        self.masa_type == VaishnavaMasaType::Adhika
    }

    /// Returns true for Kṣaya Māsa.
    pub fn is_ksaya(self) -> bool {
        self.masa_type == VaishnavaMasaType::Ksaya
    }

    /// Returns the number of saṅkrāntis found in the lunar month.
    pub fn sankranti_count(self) -> u8 {
        self.sankranti_count
    }

    /// Returns the pūrṇimānta lunar month start.
    pub fn month_start(self) -> JulianDate {
        self.month_start
    }

    /// Returns the pūrṇimānta lunar month end.
    pub fn month_end(self) -> JulianDate {
        self.month_end
    }

    /// Returns whether the ayanāṁśa was provisional or explicit.
    pub fn ayanamsa_status(self) -> AyanamsaStatus {
        self.ayanamsa_status
    }
}

/// Errors from provisional native Vaiṣṇava Māsa calculation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaishnavaMasaError {
    /// Could not bracket the preceding or following pūrṇimā.
    LunarMonthBoundaryNotFound,

    /// The saṅkrānti count is outside the provisional normal/adhika/kṣaya set.
    UnsupportedSankrantiCount,

    /// Kṣaya Māsa was detected, but Ekādaśī identity behavior remains pending
    /// certification.
    KsayaMasaIdentityPending,
}

/// Calculates native Vaiṣṇava Māsa at an instant using the provisional default
/// ayanāṁśa configuration.
pub fn vaishnava_masa_at(jd: JulianDate) -> Result<VaishnavaMasaInfo, VaishnavaMasaError> {
    vaishnava_masa_at_with_config(jd, VaishnavaMasaConfig::default())
}

/// Calculates native Vaiṣṇava Māsa at an instant using an explicit config.
pub fn vaishnava_masa_at_with_config(
    jd: JulianDate,
    config: VaishnavaMasaConfig,
) -> Result<VaishnavaMasaInfo, VaishnavaMasaError> {
    let month_start = previous_purnima(jd).ok_or(VaishnavaMasaError::LunarMonthBoundaryNotFound)?;
    let month_end = next_purnima(jd).ok_or(VaishnavaMasaError::LunarMonthBoundaryNotFound)?;
    let sankrantis = sankrantis_between(month_start, month_end, config);
    let sankranti_count = sankrantis.len() as u8;

    match sankranti_count {
        0 => {
            let next_sankranti = next_sankranti_after(month_end, config)
                .ok_or(VaishnavaMasaError::UnsupportedSankrantiCount)?;
            Ok(info(
                masa_from_sign(next_sankranti.after_sign),
                VaishnavaMasaType::Adhika,
                sankranti_count,
                month_start,
                month_end,
                config,
            ))
        }
        1 => Ok(info(
            masa_from_sign(sankrantis[0].after_sign),
            VaishnavaMasaType::Normal,
            sankranti_count,
            month_start,
            month_end,
            config,
        )),
        2 => Err(VaishnavaMasaError::KsayaMasaIdentityPending),
        _ => Err(VaishnavaMasaError::UnsupportedSankrantiCount),
    }
}

fn info(
    masa: VaishnavaMasa,
    masa_type: VaishnavaMasaType,
    sankranti_count: u8,
    month_start: JulianDate,
    month_end: JulianDate,
    config: VaishnavaMasaConfig,
) -> VaishnavaMasaInfo {
    VaishnavaMasaInfo {
        masa,
        masa_type,
        sankranti_count,
        month_start,
        month_end,
        ayanamsa_status: config.ayanamsa_status(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Sankranti {
    after_sign: u8,
}

fn previous_purnima(anchor: JulianDate) -> Option<JulianDate> {
    let mut high = anchor;
    let mut low = high.add_days(DurationDays::new(-SEARCH_STEP_DAYS));
    let limit = anchor.add_days(DurationDays::new(-MAX_LUNAR_MONTH_SEARCH_DAYS));

    while low >= limit {
        if !is_after_purnima(low) && is_after_purnima(high) {
            return Some(refine_purnima_boundary(low, high));
        }
        high = low;
        low = low.add_days(DurationDays::new(-SEARCH_STEP_DAYS));
    }

    None
}

fn next_purnima(anchor: JulianDate) -> Option<JulianDate> {
    let mut low = anchor;
    let mut high = low.add_days(DurationDays::new(SEARCH_STEP_DAYS));
    let limit = anchor.add_days(DurationDays::new(MAX_LUNAR_MONTH_SEARCH_DAYS));

    while high <= limit {
        if !is_after_purnima(low) && is_after_purnima(high) {
            return Some(refine_purnima_boundary(low, high));
        }
        low = high;
        high = high.add_days(DurationDays::new(SEARCH_STEP_DAYS));
    }

    None
}

fn refine_purnima_boundary(mut low: JulianDate, mut high: JulianDate) -> JulianDate {
    for _ in 0..BOUNDARY_REFINEMENT_STEPS {
        let midpoint = JulianDate::new((low.value() + high.value()) / 2.0);
        if is_after_purnima(midpoint) {
            high = midpoint;
        } else {
            low = midpoint;
        }
    }

    high
}

fn is_after_purnima(jd: JulianDate) -> bool {
    lunar_solar_elongation_at(jd).value() >= HALF_TURN_DEGREES
}

fn sankrantis_between(
    start: JulianDate,
    end: JulianDate,
    config: VaishnavaMasaConfig,
) -> Vec<Sankranti> {
    let mut sankrantis = Vec::new();
    let mut previous = start;
    let mut previous_sign = sidereal_solar_sign(previous, config);
    let mut current = start.add_days(DurationDays::new(SEARCH_STEP_DAYS));

    while current <= end {
        let current_sign = sidereal_solar_sign(current, config);
        if current_sign != previous_sign {
            let instant = refine_sankranti(previous, current, previous_sign, config);
            if instant > start && instant <= end {
                sankrantis.push(Sankranti {
                    after_sign: current_sign,
                });
            }
        }

        previous = current;
        previous_sign = current_sign;
        current = current.add_days(DurationDays::new(SEARCH_STEP_DAYS));
    }

    sankrantis
}

fn next_sankranti_after(anchor: JulianDate, config: VaishnavaMasaConfig) -> Option<Sankranti> {
    let mut previous_sign = sidereal_solar_sign(anchor, config);
    let mut current = anchor.add_days(DurationDays::new(SEARCH_STEP_DAYS));
    let limit = anchor.add_days(DurationDays::new(MAX_SANKRANTI_SEARCH_DAYS));

    while current <= limit {
        let current_sign = sidereal_solar_sign(current, config);
        if current_sign != previous_sign {
            return Some(Sankranti {
                after_sign: current_sign,
            });
        }

        previous_sign = current_sign;
        current = current.add_days(DurationDays::new(SEARCH_STEP_DAYS));
    }

    None
}

fn refine_sankranti(
    mut low: JulianDate,
    mut high: JulianDate,
    before_sign: u8,
    config: VaishnavaMasaConfig,
) -> JulianDate {
    for _ in 0..BOUNDARY_REFINEMENT_STEPS {
        let midpoint = JulianDate::new((low.value() + high.value()) / 2.0);
        if sidereal_solar_sign(midpoint, config) == before_sign {
            low = midpoint;
        } else {
            high = midpoint;
        }
    }

    high
}

fn sidereal_solar_sign(jd: JulianDate, config: VaishnavaMasaConfig) -> u8 {
    (sidereal_solar_longitude(jd, config).value() / SIGN_DEGREES).floor() as u8
}

fn sidereal_solar_longitude(jd: JulianDate, config: VaishnavaMasaConfig) -> Degrees {
    normalize_360(Degrees::new(
        solar::apparent_longitude(jd).value() - provisional_ayanamsa(jd, config).value(),
    ))
}

fn provisional_ayanamsa(jd: JulianDate, config: VaishnavaMasaConfig) -> Degrees {
    let centuries = (jd.value() - J2000) / DAYS_PER_JULIAN_CENTURY;
    Degrees::new(
        config.ayanamsa_at_j2000.value()
            + (PROVISIONAL_AYANAMSA_ARCSECONDS_PER_CENTURY / ARCSECONDS_PER_DEGREE * centuries),
    )
}

fn lunar_solar_elongation_at(jd: JulianDate) -> Degrees {
    lunar_solar_elongation(moon::apparent_longitude(jd), solar::apparent_longitude(jd))
}

fn masa_from_sign(sign: u8) -> VaishnavaMasa {
    match sign {
        0 => VaishnavaMasa::Chaitra,
        1 => VaishnavaMasa::Vaisakha,
        2 => VaishnavaMasa::Jyestha,
        3 => VaishnavaMasa::Asadha,
        4 => VaishnavaMasa::Sravana,
        5 => VaishnavaMasa::Bhadrapada,
        6 => VaishnavaMasa::Asvina,
        7 => VaishnavaMasa::Kartika,
        8 => VaishnavaMasa::Margasirsa,
        9 => VaishnavaMasa::Pausa,
        10 => VaishnavaMasa::Magha,
        _ => VaishnavaMasa::Phalguna,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        vaishnava_masa_at, vaishnava_masa_at_with_config, AyanamsaStatus, VaishnavaMasa,
        VaishnavaMasaConfig, VaishnavaMasaType,
    };
    use crate::core::time::{gregorian_to_jd, CivilDate, CivilDateTime, CivilTime, TimeScale};
    use crate::core::{Degrees, JulianDate};

    #[test]
    fn identifies_asadha_for_2026_sayana_period_with_provisional_lahiri() {
        let info = vaishnava_masa_at(civil_midday(2026, 7, 24)).expect("masa should resolve");

        assert_eq!(info.masa(), VaishnavaMasa::Asadha);
        assert_eq!(info.masa_type(), VaishnavaMasaType::Normal);
        assert_eq!(info.sankranti_count(), 1);
        assert_eq!(info.ayanamsa_status(), AyanamsaStatus::Provisional);
    }

    #[test]
    fn identifies_sravana_for_2026_pavitropana_period_with_provisional_lahiri() {
        let info = vaishnava_masa_at(civil_midday(2026, 8, 8)).expect("masa should resolve");

        assert_eq!(info.masa(), VaishnavaMasa::Sravana);
        assert_eq!(info.masa_type(), VaishnavaMasaType::Normal);
    }

    #[test]
    fn explicit_config_marks_ayanamsa_as_explicit() {
        let config = VaishnavaMasaConfig::explicit(Degrees::new(23.856_75));
        let info = vaishnava_masa_at_with_config(civil_midday(2026, 7, 24), config).expect("masa");

        assert_eq!(info.ayanamsa_status(), AyanamsaStatus::Explicit);
    }

    #[test]
    fn exposes_lunar_month_boundaries() {
        let instant = civil_midday(2026, 7, 24);
        let info = vaishnava_masa_at(instant).expect("masa should resolve");

        assert!(info.month_start() < instant);
        assert!(info.month_end() > instant);
    }

    fn civil_midday(year: i32, month: u8, day: u8) -> JulianDate {
        gregorian_to_jd(CivilDateTime::new(
            CivilDate::new(year, month, day).expect("valid date"),
            CivilTime::new(12, 0, 0.0).expect("valid time"),
            TimeScale::Utc,
        ))
    }
}
