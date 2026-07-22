//! Vaishnava observance identity resolution.
//!
//! This module identifies the observance attached to facts already calculated
//! by the astronomy, calendar, and Vaishnava rule layers. It does not calculate
//! tithi, masa, Viddha, Mahadvadasi, Hari-vasara, or Parana.

use crate::astronomy::Paksha;
pub use crate::calendar::VaishnavaMasa;

use super::ekadasi::MahadvadasiType;

/// Catalog-backed observance type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservanceType {
    /// Ordinary Ekadasi identified by masa and paksha.
    Ekadasi,

    /// Mahadvadasi identified by HBV-EK-004 classification.
    Mahadvadasi,
}

impl ObservanceType {
    /// Stable API label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Ekadasi => "ekadasi",
            Self::Mahadvadasi => "mahadvadasi",
        }
    }
}

/// Diagnostic source used to identify the observance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservanceSource {
    /// Ordinary Ekadasi resolved by lunar month and paksha.
    MasaPaksha,

    /// Mahadvadasi resolved by HBV-EK-004 rule output.
    MahadvadasiRule,
}

/// Errors returned by observance identity resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservanceResolutionError {
    /// Ordinary Ekadasi identity requires a formal Vaishnava masa.
    MissingMasa,
}

/// Stable observance identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservanceId {
    Ek001,
    Ek002,
    Ek003,
    Ek004,
    Ek005,
    Ek006,
    Ek007,
    Ek008,
    Ek009,
    Ek010,
    Ek011,
    Ek012,
    Ek013,
    Ek014,
    Ek015,
    Ek016,
    Ek017,
    Ek018,
    Ek019,
    Ek020,
    Ek021,
    Ek022,
    Ek023,
    Ek024,
    Md001,
    Md002,
    Md003,
    Md004,
    Md005,
    Md006,
    Md007,
    Md008,
}

impl ObservanceId {
    /// Stable external label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Ek001 => "EK-001",
            Self::Ek002 => "EK-002",
            Self::Ek003 => "EK-003",
            Self::Ek004 => "EK-004",
            Self::Ek005 => "EK-005",
            Self::Ek006 => "EK-006",
            Self::Ek007 => "EK-007",
            Self::Ek008 => "EK-008",
            Self::Ek009 => "EK-009",
            Self::Ek010 => "EK-010",
            Self::Ek011 => "EK-011",
            Self::Ek012 => "EK-012",
            Self::Ek013 => "EK-013",
            Self::Ek014 => "EK-014",
            Self::Ek015 => "EK-015",
            Self::Ek016 => "EK-016",
            Self::Ek017 => "EK-017",
            Self::Ek018 => "EK-018",
            Self::Ek019 => "EK-019",
            Self::Ek020 => "EK-020",
            Self::Ek021 => "EK-021",
            Self::Ek022 => "EK-022",
            Self::Ek023 => "EK-023",
            Self::Ek024 => "EK-024",
            Self::Md001 => "MD-001",
            Self::Md002 => "MD-002",
            Self::Md003 => "MD-003",
            Self::Md004 => "MD-004",
            Self::Md005 => "MD-005",
            Self::Md006 => "MD-006",
            Self::Md007 => "MD-007",
            Self::Md008 => "MD-008",
        }
    }
}

impl ObservanceSource {
    /// Stable API label.
    pub fn label(self) -> &'static str {
        match self {
            Self::MasaPaksha => "masa_paksha",
            Self::MahadvadasiRule => "mahadvadasi_rule",
        }
    }
}

/// Resolved observance identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObservanceContent {
    /// Stable catalog identifier.
    pub id: ObservanceId,

    /// Technical content key used to load festival-library files.
    pub slug: &'static str,

    /// Human-readable display name.
    pub display_name: &'static str,

    /// Ordinary Ekadasi or Mahadvadasi.
    pub observance_type: ObservanceType,

    /// Lunar month used for ordinary Ekadasi identity, when applicable.
    pub masa: Option<VaishnavaMasa>,

    /// Paksha used for ordinary Ekadasi identity, when applicable.
    pub paksha: Option<Paksha>,

    /// Related ordinary Ekadasi identifier for Mahadvadasi observances, when
    /// it can be derived from already supplied calendar facts.
    pub fasting_for: Option<ObservanceId>,

    /// Why this identity was selected.
    pub source: ObservanceSource,
}

/// Backwards-compatible public name for the unified observance domain output.
pub type Observance = ObservanceContent;

/// Resolves an observance identity from calculated calendar and rule facts.
///
/// Mahadvadasi classification takes precedence over ordinary Ekadasi naming
/// because HBV-EK-004 transforms the observance identity from the ordinary
/// Ekadasi name into a specific Mahadvadasi type.
pub fn resolve_observance(
    mahadvadasi: Option<MahadvadasiType>,
    masa: Option<VaishnavaMasa>,
    paksha: Paksha,
) -> Result<Observance, ObservanceResolutionError> {
    if let Some(mahadvadasi) = mahadvadasi {
        let fasting_for = masa.map(|masa| resolve_ordinary_ekadasi_observance(masa, paksha).id);
        return Ok(resolve_mahadvadasi_observance_with_fasting_for(
            mahadvadasi,
            fasting_for,
        ));
    }

    let masa = masa.ok_or(ObservanceResolutionError::MissingMasa)?;

    Ok(resolve_ordinary_ekadasi_observance(masa, paksha))
}

/// Resolves the ordinary Ekadasi identity from masa and paksha.
pub fn resolve_ordinary_ekadasi_observance(masa: VaishnavaMasa, paksha: Paksha) -> Observance {
    let (id, slug, display_name) = match (masa, paksha) {
        (VaishnavaMasa::Chaitra, Paksha::Sukla) => {
            (ObservanceId::Ek001, "kamada", "Kāmadā Ekādaśī")
        }
        (VaishnavaMasa::Chaitra, Paksha::Krsna) => {
            (ObservanceId::Ek002, "papamocani", "Pāpamocanī Ekādaśī")
        }
        (VaishnavaMasa::Vaisakha, Paksha::Sukla) => {
            (ObservanceId::Ek003, "mohini", "Mohinī Ekādaśī")
        }
        (VaishnavaMasa::Vaisakha, Paksha::Krsna) => {
            (ObservanceId::Ek004, "varuthini", "Varūthinī Ekādaśī")
        }
        (VaishnavaMasa::Jyestha, Paksha::Sukla) => {
            (ObservanceId::Ek005, "nirjala", "Nirjalā Ekādaśī")
        }
        (VaishnavaMasa::Jyestha, Paksha::Krsna) => (ObservanceId::Ek006, "apara", "Aparā Ekādaśī"),
        (VaishnavaMasa::Asadha, Paksha::Sukla) => (ObservanceId::Ek007, "sayana", "Śayanā Ekādaśī"),
        (VaishnavaMasa::Asadha, Paksha::Krsna) => (ObservanceId::Ek008, "yogini", "Yoginī Ekādaśī"),
        (VaishnavaMasa::Sravana, Paksha::Sukla) => (
            ObservanceId::Ek009,
            "pavitropana",
            "Putradā - Pavitraropani Ekādaśī",
        ),
        (VaishnavaMasa::Sravana, Paksha::Krsna) => {
            (ObservanceId::Ek010, "kamika", "Kāmikā Ekādaśī")
        }
        (VaishnavaMasa::Bhadrapada, Paksha::Sukla) => {
            (ObservanceId::Ek011, "parsva", "Pārśva Ekādaśī")
        }
        (VaishnavaMasa::Bhadrapada, Paksha::Krsna) => (ObservanceId::Ek012, "aja", "Ajā Ekādaśī"),
        (VaishnavaMasa::Asvina, Paksha::Sukla) => {
            (ObservanceId::Ek013, "pasankusa", "Pāśāṅkuśā Ekādaśī")
        }
        (VaishnavaMasa::Asvina, Paksha::Krsna) => (ObservanceId::Ek014, "indira", "Indirā Ekādaśī"),
        (VaishnavaMasa::Kartika, Paksha::Sukla) => {
            (ObservanceId::Ek015, "utthana", "Utthāna Ekādaśī")
        }
        (VaishnavaMasa::Kartika, Paksha::Krsna) => (ObservanceId::Ek016, "rama", "Rāmā Ekādaśī"),
        (VaishnavaMasa::Margasirsa, Paksha::Sukla) => {
            (ObservanceId::Ek017, "mokshada", "Mokṣadā Ekādaśī")
        }
        (VaishnavaMasa::Margasirsa, Paksha::Krsna) => {
            (ObservanceId::Ek018, "utpanna", "Utpannā Ekādaśī")
        }
        (VaishnavaMasa::Pausa, Paksha::Sukla) => {
            (ObservanceId::Ek019, "putrada_pausa", "Putradā Ekādaśī")
        }
        (VaishnavaMasa::Pausa, Paksha::Krsna) => {
            (ObservanceId::Ek020, "saphala", "Saphalā Ekādaśī")
        }
        (VaishnavaMasa::Magha, Paksha::Sukla) => (ObservanceId::Ek021, "jaya", "Jayā Ekādaśī"),
        (VaishnavaMasa::Magha, Paksha::Krsna) => {
            (ObservanceId::Ek022, "sattila", "Ṣaṭ-tilā Ekādaśī")
        }
        (VaishnavaMasa::Phalguna, Paksha::Sukla) => {
            (ObservanceId::Ek023, "amalaki", "Āmalakī Ekādaśī")
        }
        (VaishnavaMasa::Phalguna, Paksha::Krsna) => {
            (ObservanceId::Ek024, "vijaya", "Vijayā Ekādaśī")
        }
    };

    Observance {
        id,
        slug,
        display_name,
        observance_type: ObservanceType::Ekadasi,
        masa: Some(masa),
        paksha: Some(paksha),
        fasting_for: None,
        source: ObservanceSource::MasaPaksha,
    }
}

/// Resolves Mahadvadasi identity from HBV-EK-004 classification output.
pub fn resolve_mahadvadasi_observance(mahadvadasi: MahadvadasiType) -> Observance {
    resolve_mahadvadasi_observance_with_fasting_for(mahadvadasi, None)
}

fn resolve_mahadvadasi_observance_with_fasting_for(
    mahadvadasi: MahadvadasiType,
    fasting_for: Option<ObservanceId>,
) -> Observance {
    let (id, slug, display_name) = match mahadvadasi {
        MahadvadasiType::Unmilani => (ObservanceId::Md001, "unmilani", "Unmīlanī Mahādvādaśī"),
        MahadvadasiType::Vyanjuli => (ObservanceId::Md002, "vyanjuli", "Vyañjulī Mahādvādaśī"),
        MahadvadasiType::Trisprsa => (ObservanceId::Md003, "trisprsa", "Triṣpṛṣā Mahādvādaśī"),
        MahadvadasiType::Paksavardhini => (
            ObservanceId::Md004,
            "paksavardhini",
            "Pakṣavardhinī Mahādvādaśī",
        ),
        MahadvadasiType::Jaya => (ObservanceId::Md005, "jaya_mahadvadasi", "Jayā Mahādvādaśī"),
        MahadvadasiType::Vijaya => (
            ObservanceId::Md006,
            "vijaya_mahadvadasi",
            "Vijayā Mahādvādaśī",
        ),
        MahadvadasiType::Jayanti => (ObservanceId::Md007, "jayanti", "Jayantī Mahādvādaśī"),
        MahadvadasiType::Papanasini => {
            (ObservanceId::Md008, "papanasini", "Pāpanāśinī Mahādvādaśī")
        }
    };

    Observance {
        id,
        slug,
        display_name,
        observance_type: ObservanceType::Mahadvadasi,
        masa: None,
        paksha: None,
        fasting_for,
        source: ObservanceSource::MahadvadasiRule,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        resolve_mahadvadasi_observance, resolve_observance, resolve_ordinary_ekadasi_observance,
        ObservanceId, ObservanceResolutionError, ObservanceSource, ObservanceType, VaishnavaMasa,
    };
    use crate::astronomy::Paksha;
    use crate::vaishnava::MahadvadasiType;

    #[test]
    fn resolves_ordinary_ekadasi_from_masa_and_paksha() {
        let observance = resolve_ordinary_ekadasi_observance(VaishnavaMasa::Sravana, Paksha::Krsna);

        assert_eq!(observance.id, ObservanceId::Ek010);
        assert_eq!(observance.slug, "kamika");
        assert_eq!(observance.display_name, "Kāmikā Ekādaśī");
        assert_eq!(observance.observance_type, ObservanceType::Ekadasi);
        assert_eq!(observance.masa, Some(VaishnavaMasa::Sravana));
        assert_eq!(observance.paksha, Some(Paksha::Krsna));
        assert_eq!(observance.fasting_for, None);
        assert_eq!(observance.source, ObservanceSource::MasaPaksha);
    }

    #[test]
    fn resolves_all_ordinary_ekadasis_from_masa_and_paksha() {
        let cases = [
            (
                VaishnavaMasa::Chaitra,
                Paksha::Sukla,
                ObservanceId::Ek001,
                "kamada",
            ),
            (
                VaishnavaMasa::Chaitra,
                Paksha::Krsna,
                ObservanceId::Ek002,
                "papamocani",
            ),
            (
                VaishnavaMasa::Vaisakha,
                Paksha::Sukla,
                ObservanceId::Ek003,
                "mohini",
            ),
            (
                VaishnavaMasa::Vaisakha,
                Paksha::Krsna,
                ObservanceId::Ek004,
                "varuthini",
            ),
            (
                VaishnavaMasa::Jyestha,
                Paksha::Sukla,
                ObservanceId::Ek005,
                "nirjala",
            ),
            (
                VaishnavaMasa::Jyestha,
                Paksha::Krsna,
                ObservanceId::Ek006,
                "apara",
            ),
            (
                VaishnavaMasa::Asadha,
                Paksha::Sukla,
                ObservanceId::Ek007,
                "sayana",
            ),
            (
                VaishnavaMasa::Asadha,
                Paksha::Krsna,
                ObservanceId::Ek008,
                "yogini",
            ),
            (
                VaishnavaMasa::Sravana,
                Paksha::Sukla,
                ObservanceId::Ek009,
                "pavitropana",
            ),
            (
                VaishnavaMasa::Sravana,
                Paksha::Krsna,
                ObservanceId::Ek010,
                "kamika",
            ),
            (
                VaishnavaMasa::Bhadrapada,
                Paksha::Sukla,
                ObservanceId::Ek011,
                "parsva",
            ),
            (
                VaishnavaMasa::Bhadrapada,
                Paksha::Krsna,
                ObservanceId::Ek012,
                "aja",
            ),
            (
                VaishnavaMasa::Asvina,
                Paksha::Sukla,
                ObservanceId::Ek013,
                "pasankusa",
            ),
            (
                VaishnavaMasa::Asvina,
                Paksha::Krsna,
                ObservanceId::Ek014,
                "indira",
            ),
            (
                VaishnavaMasa::Kartika,
                Paksha::Sukla,
                ObservanceId::Ek015,
                "utthana",
            ),
            (
                VaishnavaMasa::Kartika,
                Paksha::Krsna,
                ObservanceId::Ek016,
                "rama",
            ),
            (
                VaishnavaMasa::Margasirsa,
                Paksha::Sukla,
                ObservanceId::Ek017,
                "mokshada",
            ),
            (
                VaishnavaMasa::Margasirsa,
                Paksha::Krsna,
                ObservanceId::Ek018,
                "utpanna",
            ),
            (
                VaishnavaMasa::Pausa,
                Paksha::Sukla,
                ObservanceId::Ek019,
                "putrada_pausa",
            ),
            (
                VaishnavaMasa::Pausa,
                Paksha::Krsna,
                ObservanceId::Ek020,
                "saphala",
            ),
            (
                VaishnavaMasa::Magha,
                Paksha::Sukla,
                ObservanceId::Ek021,
                "jaya",
            ),
            (
                VaishnavaMasa::Magha,
                Paksha::Krsna,
                ObservanceId::Ek022,
                "sattila",
            ),
            (
                VaishnavaMasa::Phalguna,
                Paksha::Sukla,
                ObservanceId::Ek023,
                "amalaki",
            ),
            (
                VaishnavaMasa::Phalguna,
                Paksha::Krsna,
                ObservanceId::Ek024,
                "vijaya",
            ),
        ];

        for (masa, paksha, id, slug) in cases {
            let observance = resolve_ordinary_ekadasi_observance(masa, paksha);

            assert_eq!(observance.id, id);
            assert_eq!(observance.slug, slug);
            assert_eq!(observance.observance_type, ObservanceType::Ekadasi);
            assert_eq!(observance.source, ObservanceSource::MasaPaksha);
        }
    }

    #[test]
    fn resolves_mahadvadasi_from_rule_output() {
        let observance = resolve_mahadvadasi_observance(MahadvadasiType::Trisprsa);

        assert_eq!(observance.id, ObservanceId::Md003);
        assert_eq!(observance.slug, "trisprsa");
        assert_eq!(observance.display_name, "Triṣpṛṣā Mahādvādaśī");
        assert_eq!(observance.observance_type, ObservanceType::Mahadvadasi);
        assert_eq!(observance.masa, None);
        assert_eq!(observance.paksha, None);
        assert_eq!(observance.fasting_for, None);
        assert_eq!(observance.source, ObservanceSource::MahadvadasiRule);
    }

    #[test]
    fn resolves_all_supported_mahadvadasis_from_rule_output() {
        let cases = [
            (MahadvadasiType::Unmilani, ObservanceId::Md001, "unmilani"),
            (MahadvadasiType::Vyanjuli, ObservanceId::Md002, "vyanjuli"),
            (MahadvadasiType::Trisprsa, ObservanceId::Md003, "trisprsa"),
            (
                MahadvadasiType::Paksavardhini,
                ObservanceId::Md004,
                "paksavardhini",
            ),
            (
                MahadvadasiType::Jaya,
                ObservanceId::Md005,
                "jaya_mahadvadasi",
            ),
            (
                MahadvadasiType::Vijaya,
                ObservanceId::Md006,
                "vijaya_mahadvadasi",
            ),
            (MahadvadasiType::Jayanti, ObservanceId::Md007, "jayanti"),
            (
                MahadvadasiType::Papanasini,
                ObservanceId::Md008,
                "papanasini",
            ),
        ];

        for (mahadvadasi, id, slug) in cases {
            let observance = resolve_mahadvadasi_observance(mahadvadasi);

            assert_eq!(observance.id, id);
            assert_eq!(observance.slug, slug);
            assert_eq!(observance.observance_type, ObservanceType::Mahadvadasi);
            assert_eq!(observance.source, ObservanceSource::MahadvadasiRule);
        }
    }

    #[test]
    fn mahadvadasi_resolution_takes_precedence_over_masa_paksha() {
        let observance = resolve_observance(
            Some(MahadvadasiType::Trisprsa),
            Some(VaishnavaMasa::Sravana),
            Paksha::Krsna,
        )
        .expect("mahadvadasi resolves without using ordinary name");

        assert_eq!(observance.id, ObservanceId::Md003);
        assert_eq!(observance.slug, "trisprsa");
        assert_eq!(observance.fasting_for, Some(ObservanceId::Ek010));
        assert_eq!(observance.source, ObservanceSource::MahadvadasiRule);
    }

    #[test]
    fn ordinary_ekadasi_resolution_requires_masa() {
        assert_eq!(
            resolve_observance(None, None, Paksha::Krsna),
            Err(ObservanceResolutionError::MissingMasa)
        );
    }
}
