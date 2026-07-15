//! HBV-EK-001 candidate Ekadasi.
//!
//! Implements only the frozen Knowledge Base rule:
//!
//! ```text
//! A civil day enters the initial Ekadasi candidate set when the tithi present
//! at local sunrise is Ekadasi.
//! ```

use crate::astronomy::{AstronomicalTithi, NakshatraIndex, Paksha};
use crate::calendar::CivilDayTithiPresence;
use crate::core::{DurationDays, JulianDate};

const DASAMI_TRADITIONAL_NUMBER: u8 = 10;
const EKADASI_TRADITIONAL_NUMBER: u8 = 11;
const DVADASI_TRADITIONAL_NUMBER: u8 = 12;
const MINUTES_PER_DAY: f64 = 1_440.0;

/// HBV-EK-002 arunodaya interval before sunrise, in minutes.
pub const ARUNODAYA_BEFORE_SUNRISE_MINUTES: f64 = 96.0;

/// HBV-EK-001 candidate classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EkadasiCandidate {
    /// Ekadasi is present at sunrise.
    CandidateEkadasi,

    /// Ekadasi is not present at sunrise.
    NotCandidate,
}

/// HBV-EK-002 Viddha classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViddhaCandidateStatus {
    /// The input was not an Ekadasi candidate, so HBV-EK-002 has no effect.
    NotCandidate,

    /// The candidate remains valid under HBV-EK-002.
    ValidCandidate,

    /// The candidate is invalidated because Dasami is present at arunodaya.
    InvalidViddha,
}

/// HBV-EK-003 observance disposition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EkadasiObservanceDisposition {
    /// The valid candidate is observed on Ekadasi.
    ObserveOnEkadasi,

    /// The Viddha candidate is displaced to Dvadasi.
    ObserveOnDvadasi,

    /// There is no Ekadasi observance candidate.
    NoObservanceCandidate,
}

/// HBV-EK-004 Mahadvadasi type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MahadvadasiType {
    /// Ekadasi continues beyond the following sunrise.
    Unmilani,

    /// Pure Ekadasi and Dvadasi present at sunrise, with Dvadasi continuing
    /// toward Trayodasi.
    ///
    /// ISSUE-VAI-001 resolved the documented Vyanjuli formulation divergence;
    /// this variant follows the frozen Knowledge Base rule.
    Vyanjuli,

    /// Ekadasi, Dvadasi, and Trayodasi touch in a valid sequence without
    /// Dasami involvement.
    Trisprsa,

    /// The preceding Dvadasi is classified because Amavasya or Purnima grows.
    Paksavardhini,

    /// Sukla Dvadasi combined with Punarvasu.
    Jaya,

    /// Sukla Dvadasi combined with Shravana.
    Vijaya,

    /// Sukla Dvadasi combined with Rohini.
    Jayanti,

    /// Sukla Dvadasi combined with Pushya.
    Papanasini,
}

/// HBV-EK-005 Parana mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParanaMode {
    /// Ordinary parana window after Hari-vasara and before Dvadasi ends.
    Standard,

    /// Dvadasi is too short for an ordinary window after all limits are applied.
    ShortDvadasi,
}

/// HBV-EK-005 Parana window.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParanaWindow {
    /// Earliest allowed parana instant.
    pub starts_at: JulianDate,

    /// Latest allowed parana instant before Dvadasi is transgressed.
    pub ends_at: JulianDate,

    /// Standard window or short-Dvadasi contingency.
    pub mode: ParanaMode,
}

/// Explicit errors for invalid HBV-EK-005 interval inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParanaWindowError {
    /// Dvadasi end must be later than Dvadasi start.
    InvalidDvadasiInterval,
}

/// Errors returned by the integrated Vaishnava Engine facade.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaishnavaEngineError {
    /// An observance exists, but the Parana interval inputs were not supplied.
    MissingParanaInput,

    /// The Parana interval inputs were invalid.
    ParanaWindow(ParanaWindowError),
}

/// Tithi-based HBV-EK-004 condition already determined by calendar facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TithiMahadvadasiCondition {
    /// Ekadasi is present across two consecutive sunrises.
    Unmilani,

    /// Pure Ekadasi and Dvadasi present at sunrise, with Dvadasi continuing
    /// toward Trayodasi.
    Vyanjuli,

    /// Ekadasi, Dvadasi, and Trayodasi touch in a valid sequence.
    Trisprsa,

    /// Dvadasi preceding a growing Amavasya or Purnima.
    Paksavardhini,
}

/// Input facts for HBV-EK-004 Mahadvadasi classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MahadvadasiClassification {
    observance: EkadasiObservanceDisposition,
    tithi_condition: Option<TithiMahadvadasiCondition>,
    tithi_at_observance_sunrise: Option<AstronomicalTithi>,
    nakshatra_at_observance: Option<NakshatraIndex>,
}

impl MahadvadasiClassification {
    /// Creates input facts for HBV-EK-004 classification.
    pub fn new(observance: EkadasiObservanceDisposition) -> Self {
        Self {
            observance,
            tithi_condition: None,
            tithi_at_observance_sunrise: None,
            nakshatra_at_observance: None,
        }
    }

    /// Adds a tithi-based Mahadvadasi condition.
    pub fn with_tithi_condition(mut self, condition: TithiMahadvadasiCondition) -> Self {
        self.tithi_condition = Some(condition);
        self
    }

    /// Adds the tithi present at the observance sunrise.
    pub fn with_tithi_at_observance_sunrise(mut self, tithi: AstronomicalTithi) -> Self {
        self.tithi_at_observance_sunrise = Some(tithi);
        self
    }

    /// Adds the nakshatra associated with the observance.
    pub fn with_nakshatra_at_observance(mut self, nakshatra: NakshatraIndex) -> Self {
        self.nakshatra_at_observance = Some(nakshatra);
        self
    }
}

/// Optional HBV-EK-004 facts supplied to the integrated facade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MahadvadasiFacts {
    tithi_condition: Option<TithiMahadvadasiCondition>,
    tithi_at_observance_sunrise: Option<AstronomicalTithi>,
    nakshatra_at_observance: Option<NakshatraIndex>,
}

impl MahadvadasiFacts {
    /// Creates empty Mahadvadasi facts.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a tithi-based Mahadvadasi condition.
    pub fn with_tithi_condition(mut self, condition: TithiMahadvadasiCondition) -> Self {
        self.tithi_condition = Some(condition);
        self
    }

    /// Adds the tithi present at the observance sunrise.
    pub fn with_tithi_at_observance_sunrise(mut self, tithi: AstronomicalTithi) -> Self {
        self.tithi_at_observance_sunrise = Some(tithi);
        self
    }

    /// Adds the nakshatra associated with the observance.
    pub fn with_nakshatra_at_observance(mut self, nakshatra: NakshatraIndex) -> Self {
        self.nakshatra_at_observance = Some(nakshatra);
        self
    }

    fn classification(self, observance: EkadasiObservanceDisposition) -> MahadvadasiClassification {
        let mut classification = MahadvadasiClassification::new(observance);

        if let Some(condition) = self.tithi_condition {
            classification = classification.with_tithi_condition(condition);
        }
        if let Some(tithi) = self.tithi_at_observance_sunrise {
            classification = classification.with_tithi_at_observance_sunrise(tithi);
        }
        if let Some(nakshatra) = self.nakshatra_at_observance {
            classification = classification.with_nakshatra_at_observance(nakshatra);
        }

        classification
    }
}

/// HBV-EK-005 inputs supplied to the integrated facade.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParanaInput {
    sunrise: JulianDate,
    dvadasi_start: JulianDate,
    dvadasi_end: JulianDate,
}

impl ParanaInput {
    /// Creates Parana inputs from already calculated calendar facts.
    pub fn new(sunrise: JulianDate, dvadasi_start: JulianDate, dvadasi_end: JulianDate) -> Self {
        Self {
            sunrise,
            dvadasi_start,
            dvadasi_end,
        }
    }
}

/// Inputs for the integrated Vaishnava Engine facade.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VaishnavaDayInput {
    tithi_at_sunrise: AstronomicalTithi,
    tithi_at_arunodaya: AstronomicalTithi,
    mahadvadasi_facts: Option<MahadvadasiFacts>,
    parana: Option<ParanaInput>,
}

impl VaishnavaDayInput {
    /// Creates an input from already calculated calendar facts.
    pub fn new(tithi_at_sunrise: AstronomicalTithi, tithi_at_arunodaya: AstronomicalTithi) -> Self {
        Self {
            tithi_at_sunrise,
            tithi_at_arunodaya,
            mahadvadasi_facts: None,
            parana: None,
        }
    }

    /// Adds optional Mahadvadasi facts.
    pub fn with_mahadvadasi_facts(mut self, facts: MahadvadasiFacts) -> Self {
        self.mahadvadasi_facts = Some(facts);
        self
    }

    /// Adds Parana interval facts.
    pub fn with_parana(mut self, parana: ParanaInput) -> Self {
        self.parana = Some(parana);
        self
    }
}

/// Integrated Vaishnava day classification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VaishnavaDayClassification {
    /// HBV-EK-001 result.
    pub candidate: EkadasiCandidate,

    /// HBV-EK-002 result.
    pub viddha_status: ViddhaCandidateStatus,

    /// HBV-EK-003 result.
    pub observance: EkadasiObservanceDisposition,

    /// HBV-EK-004 result.
    pub mahadvadasi: Option<MahadvadasiType>,

    /// HBV-EK-005 result.
    pub parana: Option<ParanaWindow>,
}

/// Returns whether the tithi present at sunrise makes the civil day an initial
/// Ekadasi candidate.
///
/// This implements HBV-EK-001 only. It does not evaluate Viddha,
/// Mahadvadasi, Hari-vasara, parana, nakshatra, or festivals.
pub fn ekadasi_candidate_at_sunrise(tithi_at_sunrise: AstronomicalTithi) -> EkadasiCandidate {
    if tithi_at_sunrise.traditional_number() == EKADASI_TRADITIONAL_NUMBER {
        EkadasiCandidate::CandidateEkadasi
    } else {
        EkadasiCandidate::NotCandidate
    }
}

/// Returns whether a civil-day tithi presence record is an initial Ekadasi
/// candidate.
///
/// This is a convenience wrapper over [`ekadasi_candidate_at_sunrise`].
pub fn ekadasi_candidate_for_civil_day(presence: CivilDayTithiPresence) -> EkadasiCandidate {
    ekadasi_candidate_at_sunrise(presence.tithi_at_sunrise())
}

/// Returns the start of arunodaya for a given sunrise.
///
/// This uses the frozen HBV-EK-002 operational definition: arunodaya begins
/// 96 minutes before local sunrise.
pub fn arunodaya_start(sunrise: JulianDate) -> JulianDate {
    let days = ARUNODAYA_BEFORE_SUNRISE_MINUTES / MINUTES_PER_DAY;

    sunrise.add_days(DurationDays::new(-days))
}

/// Applies HBV-EK-002 to an initial Ekadasi candidate.
///
/// The rule only invalidates a candidate when Dasami is present at arunodaya.
/// It does not move the observance, resolve Mahadvadasi, calculate
/// Hari-vasara, parana, nakshatra, or festivals.
pub fn invalidate_viddha_candidate(
    candidate: EkadasiCandidate,
    tithi_at_arunodaya: AstronomicalTithi,
) -> ViddhaCandidateStatus {
    match candidate {
        EkadasiCandidate::NotCandidate => ViddhaCandidateStatus::NotCandidate,
        EkadasiCandidate::CandidateEkadasi
            if tithi_at_arunodaya.traditional_number() == DASAMI_TRADITIONAL_NUMBER =>
        {
            ViddhaCandidateStatus::InvalidViddha
        }
        EkadasiCandidate::CandidateEkadasi => ViddhaCandidateStatus::ValidCandidate,
    }
}

/// Applies HBV-EK-003 to the result of the Viddha rule.
///
/// This function only determines whether the observance remains on Ekadasi or
/// is displaced to Dvadasi. It does not classify Mahadvadasi, calculate
/// nakshatra, Hari-vasara, parana, or festivals.
pub fn observance_displacement(status: ViddhaCandidateStatus) -> EkadasiObservanceDisposition {
    match status {
        ViddhaCandidateStatus::NotCandidate => EkadasiObservanceDisposition::NoObservanceCandidate,
        ViddhaCandidateStatus::ValidCandidate => EkadasiObservanceDisposition::ObserveOnEkadasi,
        ViddhaCandidateStatus::InvalidViddha => EkadasiObservanceDisposition::ObserveOnDvadasi,
    }
}

/// Classifies an observance as one of the eight HBV-EK-004 Mahadvadasi types.
///
/// This function implements only the frozen Mahadvadasi classification table.
/// It does not calculate parana, Hari-vasara, ritual procedure, or festivals.
pub fn classify_mahadvadasi(input: MahadvadasiClassification) -> Option<MahadvadasiType> {
    if input.observance != EkadasiObservanceDisposition::ObserveOnDvadasi {
        return None;
    }

    if let Some(condition) = input.tithi_condition {
        return Some(match condition {
            TithiMahadvadasiCondition::Unmilani => MahadvadasiType::Unmilani,
            TithiMahadvadasiCondition::Vyanjuli => MahadvadasiType::Vyanjuli,
            TithiMahadvadasiCondition::Trisprsa => MahadvadasiType::Trisprsa,
            TithiMahadvadasiCondition::Paksavardhini => MahadvadasiType::Paksavardhini,
        });
    }

    let tithi = input.tithi_at_observance_sunrise?;
    if tithi.traditional_number() != DVADASI_TRADITIONAL_NUMBER || tithi.paksha() != Paksha::Sukla {
        return None;
    }

    match input.nakshatra_at_observance? {
        NakshatraIndex::PUNARVASU => Some(MahadvadasiType::Jaya),
        NakshatraIndex::SHRAVANA => Some(MahadvadasiType::Vijaya),
        NakshatraIndex::ROHINI => Some(MahadvadasiType::Jayanti),
        NakshatraIndex::PUSHYA => Some(MahadvadasiType::Papanasini),
        _ => None,
    }
}

/// Returns the end of Hari-vasara for HBV-EK-005.
///
/// Operationally, Hari-vasara ends after the first quarter of Dvadasi.
pub fn hari_vasara_end(
    dvadasi_start: JulianDate,
    dvadasi_end: JulianDate,
) -> Result<JulianDate, ParanaWindowError> {
    if dvadasi_end <= dvadasi_start {
        return Err(ParanaWindowError::InvalidDvadasiInterval);
    }

    let dvadasi_duration = dvadasi_end.days_since(dvadasi_start).value();
    let first_quarter = DurationDays::new(dvadasi_duration * 0.25);

    Ok(dvadasi_start.add_days(first_quarter))
}

/// Returns the HBV-EK-005 Parana window.
///
/// The ordinary window begins after both local sunrise and Hari-vasara have
/// passed, and ends before Dvadasi is transgressed. If those limits leave no
/// ordinary interval, the result preserves the short-Dvadasi contingency without
/// inventing ritual details.
pub fn parana_window(
    sunrise: JulianDate,
    dvadasi_start: JulianDate,
    dvadasi_end: JulianDate,
) -> Result<ParanaWindow, ParanaWindowError> {
    let harivasara_end = hari_vasara_end(dvadasi_start, dvadasi_end)?;
    let starts_at = if sunrise > harivasara_end {
        sunrise
    } else {
        harivasara_end
    };

    let mode = if starts_at >= dvadasi_end {
        ParanaMode::ShortDvadasi
    } else {
        ParanaMode::Standard
    };

    Ok(ParanaWindow {
        starts_at,
        ends_at: dvadasi_end,
        mode,
    })
}

/// Integrates HBV-EK-001 through HBV-EK-005 for one Vaishnava day.
///
/// This facade consumes facts already produced by the astronomy and calendar
/// layers. It does not calculate solar longitude, lunar longitude, sunrise,
/// tithi transitions, or nakshatra internally.
pub fn classify_vaishnava_day(
    input: VaishnavaDayInput,
) -> Result<VaishnavaDayClassification, VaishnavaEngineError> {
    let candidate = ekadasi_candidate_at_sunrise(input.tithi_at_sunrise);

    if candidate == EkadasiCandidate::NotCandidate {
        return Ok(VaishnavaDayClassification {
            candidate,
            viddha_status: ViddhaCandidateStatus::NotCandidate,
            observance: EkadasiObservanceDisposition::NoObservanceCandidate,
            mahadvadasi: None,
            parana: None,
        });
    }

    let viddha_status = invalidate_viddha_candidate(candidate, input.tithi_at_arunodaya);
    let observance = observance_displacement(viddha_status);
    let mahadvadasi = input
        .mahadvadasi_facts
        .and_then(|facts| classify_mahadvadasi(facts.classification(observance)));

    let parana_input = input
        .parana
        .ok_or(VaishnavaEngineError::MissingParanaInput)?;
    let parana = parana_window(
        parana_input.sunrise,
        parana_input.dvadasi_start,
        parana_input.dvadasi_end,
    )
    .map_err(VaishnavaEngineError::ParanaWindow)?;

    Ok(VaishnavaDayClassification {
        candidate,
        viddha_status,
        observance,
        mahadvadasi,
        parana: Some(parana),
    })
}

#[cfg(test)]
mod tests {
    use super::{
        arunodaya_start, classify_mahadvadasi, classify_vaishnava_day,
        ekadasi_candidate_at_sunrise, ekadasi_candidate_for_civil_day, hari_vasara_end,
        invalidate_viddha_candidate, observance_displacement, parana_window, EkadasiCandidate,
        EkadasiObservanceDisposition, MahadvadasiClassification, MahadvadasiFacts, MahadvadasiType,
        ParanaInput, ParanaMode, ParanaWindowError, TithiMahadvadasiCondition, VaishnavaDayInput,
        ViddhaCandidateStatus, ARUNODAYA_BEFORE_SUNRISE_MINUTES,
    };
    use crate::astronomy::{AstronomicalTithi, NakshatraIndex};
    use crate::calendar::CivilDayTithiPresence;
    use crate::core::JulianDate;

    #[test]
    fn dasami_at_sunrise_is_not_candidate() {
        assert_eq!(candidate_for_index(9), EkadasiCandidate::NotCandidate);
        assert_eq!(candidate_for_index(24), EkadasiCandidate::NotCandidate);
    }

    #[test]
    fn ekadasi_at_sunrise_is_candidate() {
        assert_eq!(candidate_for_index(10), EkadasiCandidate::CandidateEkadasi);
        assert_eq!(candidate_for_index(25), EkadasiCandidate::CandidateEkadasi);
    }

    #[test]
    fn dvadasi_at_sunrise_is_not_candidate() {
        assert_eq!(candidate_for_index(11), EkadasiCandidate::NotCandidate);
        assert_eq!(candidate_for_index(26), EkadasiCandidate::NotCandidate);
    }

    #[test]
    fn all_thirty_tithis_follow_hbv_ek_001() {
        for index in 0..30 {
            let expected = if index == 10 || index == 25 {
                EkadasiCandidate::CandidateEkadasi
            } else {
                EkadasiCandidate::NotCandidate
            };

            assert_eq!(candidate_for_index(index), expected, "index {index}");
        }
    }

    #[test]
    fn civil_day_wrapper_uses_only_tithi_at_sunrise() {
        let presence = CivilDayTithiPresence::new(tithi(10), None);

        assert_eq!(
            ekadasi_candidate_for_civil_day(presence),
            EkadasiCandidate::CandidateEkadasi
        );
    }

    #[test]
    fn candidate_with_dasami_at_arunodaya_is_invalid_viddha() {
        assert_eq!(
            invalidate_viddha_candidate(EkadasiCandidate::CandidateEkadasi, tithi(9)),
            ViddhaCandidateStatus::InvalidViddha
        );
        assert_eq!(
            invalidate_viddha_candidate(EkadasiCandidate::CandidateEkadasi, tithi(24)),
            ViddhaCandidateStatus::InvalidViddha
        );
    }

    #[test]
    fn candidate_without_dasami_at_arunodaya_remains_valid() {
        assert_eq!(
            invalidate_viddha_candidate(EkadasiCandidate::CandidateEkadasi, tithi(10)),
            ViddhaCandidateStatus::ValidCandidate
        );
        assert_eq!(
            invalidate_viddha_candidate(EkadasiCandidate::CandidateEkadasi, tithi(25)),
            ViddhaCandidateStatus::ValidCandidate
        );
    }

    #[test]
    fn not_candidate_is_unchanged_by_viddha_rule() {
        assert_eq!(
            invalidate_viddha_candidate(EkadasiCandidate::NotCandidate, tithi(9)),
            ViddhaCandidateStatus::NotCandidate
        );
    }

    #[test]
    fn arunodaya_starts_exactly_ninety_six_minutes_before_sunrise() {
        let sunrise = JulianDate::new(2_461_120.25);

        let arunodaya = arunodaya_start(sunrise);

        let expected_days = ARUNODAYA_BEFORE_SUNRISE_MINUTES / 1_440.0;
        let actual_days = sunrise.days_since(arunodaya).value();
        assert!((actual_days - expected_days).abs() < 1.0e-9);
    }

    #[test]
    fn valid_candidate_is_observed_on_ekadasi() {
        assert_eq!(
            observance_displacement(ViddhaCandidateStatus::ValidCandidate),
            EkadasiObservanceDisposition::ObserveOnEkadasi
        );
    }

    #[test]
    fn invalid_viddha_candidate_is_observed_on_dvadasi() {
        assert_eq!(
            observance_displacement(ViddhaCandidateStatus::InvalidViddha),
            EkadasiObservanceDisposition::ObserveOnDvadasi
        );
    }

    #[test]
    fn not_candidate_has_no_observance_candidate() {
        assert_eq!(
            observance_displacement(ViddhaCandidateStatus::NotCandidate),
            EkadasiObservanceDisposition::NoObservanceCandidate
        );
    }

    #[test]
    fn composition_displaces_viddha_ekadasi_to_dvadasi() {
        let candidate = ekadasi_candidate_at_sunrise(tithi(10));
        let viddha_status = invalidate_viddha_candidate(candidate, tithi(9));

        assert_eq!(
            observance_displacement(viddha_status),
            EkadasiObservanceDisposition::ObserveOnDvadasi
        );
    }

    #[test]
    fn classifies_tithi_based_mahadvadasis() {
        assert_eq!(
            classify_tithi_condition(TithiMahadvadasiCondition::Unmilani),
            Some(MahadvadasiType::Unmilani)
        );
        assert_eq!(
            classify_tithi_condition(TithiMahadvadasiCondition::Vyanjuli),
            Some(MahadvadasiType::Vyanjuli)
        );
        assert_eq!(
            classify_tithi_condition(TithiMahadvadasiCondition::Trisprsa),
            Some(MahadvadasiType::Trisprsa)
        );
        assert_eq!(
            classify_tithi_condition(TithiMahadvadasiCondition::Paksavardhini),
            Some(MahadvadasiType::Paksavardhini)
        );
    }

    #[test]
    fn does_not_classify_tithi_based_condition_without_dvadasi_observance() {
        let input = MahadvadasiClassification::new(EkadasiObservanceDisposition::ObserveOnEkadasi)
            .with_tithi_condition(TithiMahadvadasiCondition::Unmilani);

        assert_eq!(classify_mahadvadasi(input), None);
    }

    #[test]
    fn classifies_nakshatra_based_mahadvadasis() {
        assert_eq!(
            classify_nakshatra_condition(NakshatraIndex::PUNARVASU),
            Some(MahadvadasiType::Jaya)
        );
        assert_eq!(
            classify_nakshatra_condition(NakshatraIndex::SHRAVANA),
            Some(MahadvadasiType::Vijaya)
        );
        assert_eq!(
            classify_nakshatra_condition(NakshatraIndex::ROHINI),
            Some(MahadvadasiType::Jayanti)
        );
        assert_eq!(
            classify_nakshatra_condition(NakshatraIndex::PUSHYA),
            Some(MahadvadasiType::Papanasini)
        );
    }

    #[test]
    fn nakshatra_based_mahadvadasi_requires_sukla_dvadasi() {
        let krsna_dvadasi =
            MahadvadasiClassification::new(EkadasiObservanceDisposition::ObserveOnDvadasi)
                .with_tithi_at_observance_sunrise(tithi(26))
                .with_nakshatra_at_observance(NakshatraIndex::PUNARVASU);

        let sukla_ekadasi =
            MahadvadasiClassification::new(EkadasiObservanceDisposition::ObserveOnDvadasi)
                .with_tithi_at_observance_sunrise(tithi(10))
                .with_nakshatra_at_observance(NakshatraIndex::PUNARVASU);

        assert_eq!(classify_mahadvadasi(krsna_dvadasi), None);
        assert_eq!(classify_mahadvadasi(sukla_ekadasi), None);
    }

    #[test]
    fn nakshatra_based_mahadvadasi_requires_matching_nakshatra() {
        let input = MahadvadasiClassification::new(EkadasiObservanceDisposition::ObserveOnDvadasi)
            .with_tithi_at_observance_sunrise(tithi(11))
            .with_nakshatra_at_observance(NakshatraIndex::from_index(0).expect("valid nakshatra"));

        assert_eq!(classify_mahadvadasi(input), None);
    }

    #[test]
    fn observance_without_mahadvadasi_condition_returns_none() {
        let input = MahadvadasiClassification::new(EkadasiObservanceDisposition::ObserveOnDvadasi);

        assert_eq!(classify_mahadvadasi(input), None);
    }

    #[test]
    fn hari_vasara_ends_after_first_quarter_of_dvadasi() {
        let dvadasi_start = JulianDate::new(2_461_120.0);
        let dvadasi_end = JulianDate::new(2_461_121.0);

        let end = hari_vasara_end(dvadasi_start, dvadasi_end).expect("valid interval");

        assert_eq!(end, JulianDate::new(2_461_120.25));
    }

    #[test]
    fn standard_parana_window_starts_at_sunrise_when_hari_vasara_ended_before_sunrise() {
        let sunrise = JulianDate::new(2_461_120.5);
        let dvadasi_start = JulianDate::new(2_461_120.0);
        let dvadasi_end = JulianDate::new(2_461_121.0);

        let window = parana_window(sunrise, dvadasi_start, dvadasi_end).expect("valid window");

        assert_eq!(window.starts_at, sunrise);
        assert_eq!(window.ends_at, dvadasi_end);
        assert_eq!(window.mode, ParanaMode::Standard);
    }

    #[test]
    fn standard_parana_window_starts_at_hari_vasara_end_when_it_ends_after_sunrise() {
        let sunrise = JulianDate::new(2_461_120.1);
        let dvadasi_start = JulianDate::new(2_461_120.0);
        let dvadasi_end = JulianDate::new(2_461_121.0);
        let expected_start = JulianDate::new(2_461_120.25);

        let window = parana_window(sunrise, dvadasi_start, dvadasi_end).expect("valid window");

        assert_eq!(window.starts_at, expected_start);
        assert_eq!(window.ends_at, dvadasi_end);
        assert_eq!(window.mode, ParanaMode::Standard);
    }

    #[test]
    fn short_dvadasi_is_explicit_when_no_ordinary_window_remains() {
        let sunrise = JulianDate::new(2_461_121.0);
        let dvadasi_start = JulianDate::new(2_461_120.0);
        let dvadasi_end = JulianDate::new(2_461_120.5);

        let window = parana_window(sunrise, dvadasi_start, dvadasi_end).expect("valid interval");

        assert_eq!(window.starts_at, sunrise);
        assert_eq!(window.ends_at, dvadasi_end);
        assert_eq!(window.mode, ParanaMode::ShortDvadasi);
    }

    #[test]
    fn parana_window_never_starts_before_sunrise_hari_vasara_or_after_dvadasi_in_standard_mode() {
        let sunrise = JulianDate::new(2_461_120.2);
        let dvadasi_start = JulianDate::new(2_461_120.0);
        let dvadasi_end = JulianDate::new(2_461_121.0);
        let harivasara_end = hari_vasara_end(dvadasi_start, dvadasi_end).expect("valid interval");

        let window = parana_window(sunrise, dvadasi_start, dvadasi_end).expect("valid window");

        assert!(window.starts_at >= sunrise);
        assert!(window.starts_at >= harivasara_end);
        assert!(window.ends_at <= dvadasi_end);
        assert!(window.starts_at < window.ends_at);
        assert_eq!(window.mode, ParanaMode::Standard);
    }

    #[test]
    fn invalid_dvadasi_interval_returns_explicit_error() {
        let sunrise = JulianDate::new(2_461_120.5);
        let dvadasi_start = JulianDate::new(2_461_121.0);
        let dvadasi_end = JulianDate::new(2_461_121.0);

        assert_eq!(
            parana_window(sunrise, dvadasi_start, dvadasi_end),
            Err(ParanaWindowError::InvalidDvadasiInterval)
        );
    }

    #[test]
    fn integrated_valid_ekadasi_observes_on_ekadasi_with_parana() {
        let result =
            classify_vaishnava_day(base_input(tithi(10), tithi(10)).with_parana(parana_input()))
                .expect("valid classification");

        assert_eq!(result.candidate, EkadasiCandidate::CandidateEkadasi);
        assert_eq!(result.viddha_status, ViddhaCandidateStatus::ValidCandidate);
        assert_eq!(
            result.observance,
            EkadasiObservanceDisposition::ObserveOnEkadasi
        );
        assert_eq!(result.mahadvadasi, None);
        assert_eq!(result.parana.expect("parana").mode, ParanaMode::Standard);
    }

    #[test]
    fn integrated_viddha_ekadasi_observes_on_dvadasi_with_parana() {
        let result =
            classify_vaishnava_day(base_input(tithi(10), tithi(9)).with_parana(parana_input()))
                .expect("valid classification");

        assert_eq!(result.candidate, EkadasiCandidate::CandidateEkadasi);
        assert_eq!(result.viddha_status, ViddhaCandidateStatus::InvalidViddha);
        assert_eq!(
            result.observance,
            EkadasiObservanceDisposition::ObserveOnDvadasi
        );
        assert_eq!(result.mahadvadasi, None);
        assert_eq!(result.parana.expect("parana").mode, ParanaMode::Standard);
    }

    #[test]
    fn integrated_tithi_based_mahadvadasi_is_classified() {
        let facts =
            MahadvadasiFacts::new().with_tithi_condition(TithiMahadvadasiCondition::Unmilani);
        let result = classify_vaishnava_day(
            base_input(tithi(10), tithi(9))
                .with_mahadvadasi_facts(facts)
                .with_parana(parana_input()),
        )
        .expect("valid classification");

        assert_eq!(result.mahadvadasi, Some(MahadvadasiType::Unmilani));
        assert_eq!(result.parana.expect("parana").mode, ParanaMode::Standard);
    }

    #[test]
    fn integrated_nakshatra_based_mahadvadasi_is_classified() {
        let facts = MahadvadasiFacts::new()
            .with_tithi_at_observance_sunrise(tithi(11))
            .with_nakshatra_at_observance(NakshatraIndex::PUNARVASU);
        let result = classify_vaishnava_day(
            base_input(tithi(10), tithi(9))
                .with_mahadvadasi_facts(facts)
                .with_parana(parana_input()),
        )
        .expect("valid classification");

        assert_eq!(result.mahadvadasi, Some(MahadvadasiType::Jaya));
        assert_eq!(result.parana.expect("parana").mode, ParanaMode::Standard);
    }

    #[test]
    fn integrated_non_ekadasi_stops_without_parana() {
        let result = classify_vaishnava_day(base_input(tithi(9), tithi(9)))
            .expect("not candidate does not need parana");

        assert_eq!(result.candidate, EkadasiCandidate::NotCandidate);
        assert_eq!(result.viddha_status, ViddhaCandidateStatus::NotCandidate);
        assert_eq!(
            result.observance,
            EkadasiObservanceDisposition::NoObservanceCandidate
        );
        assert_eq!(result.mahadvadasi, None);
        assert_eq!(result.parana, None);
    }

    fn candidate_for_index(index: u8) -> EkadasiCandidate {
        ekadasi_candidate_at_sunrise(tithi(index))
    }

    fn tithi(index: u8) -> AstronomicalTithi {
        AstronomicalTithi::from_index(index).expect("valid tithi index")
    }

    fn classify_tithi_condition(condition: TithiMahadvadasiCondition) -> Option<MahadvadasiType> {
        let input = MahadvadasiClassification::new(EkadasiObservanceDisposition::ObserveOnDvadasi)
            .with_tithi_condition(condition);

        classify_mahadvadasi(input)
    }

    fn classify_nakshatra_condition(nakshatra: NakshatraIndex) -> Option<MahadvadasiType> {
        let input = MahadvadasiClassification::new(EkadasiObservanceDisposition::ObserveOnDvadasi)
            .with_tithi_at_observance_sunrise(tithi(11))
            .with_nakshatra_at_observance(nakshatra);

        classify_mahadvadasi(input)
    }

    fn base_input(
        tithi_at_sunrise: AstronomicalTithi,
        tithi_at_arunodaya: AstronomicalTithi,
    ) -> VaishnavaDayInput {
        VaishnavaDayInput::new(tithi_at_sunrise, tithi_at_arunodaya)
    }

    fn parana_input() -> ParanaInput {
        ParanaInput::new(
            JulianDate::new(2_461_120.5),
            JulianDate::new(2_461_120.0),
            JulianDate::new(2_461_121.0),
        )
    }
}
