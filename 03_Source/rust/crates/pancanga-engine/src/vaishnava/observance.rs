//! Vaishnava observance identity resolution.
//!
//! This module identifies the observance attached to facts already calculated
//! by the astronomy, calendar, and Vaishnava rule layers. It does not calculate
//! tithi, masa, Viddha, Mahadvadasi, Hari-vasara, or Parana.

use crate::astronomy::Paksha;

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

impl ObservanceSource {
    /// Stable API label.
    pub fn label(self) -> &'static str {
        match self {
            Self::MasaPaksha => "masa_paksha",
            Self::MahadvadasiRule => "mahadvadasi_rule",
        }
    }
}

/// Lunar month identifier used by the observance catalog.
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
    /// Display label used by the RC1 API.
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

/// Resolved observance identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Observance {
    /// Stable catalog identifier.
    pub id: &'static str,

    /// Technical content key used to load festival-library files.
    pub slug: &'static str,

    /// Human-readable display name.
    pub display_name: &'static str,

    /// Ordinary Ekadasi or Mahadvadasi.
    pub observance_type: ObservanceType,

    /// Why this identity was selected.
    pub source: ObservanceSource,
}

/// Resolves the ordinary Ekadasi identity from masa and paksha.
pub fn resolve_ordinary_ekadasi_observance(masa: VaishnavaMasa, paksha: Paksha) -> Observance {
    let (id, slug, display_name) = match (masa, paksha) {
        (VaishnavaMasa::Chaitra, Paksha::Sukla) => ("EK-001", "kamada", "Kāmadā Ekādaśī"),
        (VaishnavaMasa::Chaitra, Paksha::Krsna) => ("EK-002", "papamocani", "Pāpamocanī Ekādaśī"),
        (VaishnavaMasa::Vaisakha, Paksha::Sukla) => ("EK-003", "mohini", "Mohinī Ekādaśī"),
        (VaishnavaMasa::Vaisakha, Paksha::Krsna) => ("EK-004", "varuthini", "Varūthinī Ekādaśī"),
        (VaishnavaMasa::Jyestha, Paksha::Sukla) => ("EK-005", "nirjala", "Nirjalā Ekādaśī"),
        (VaishnavaMasa::Jyestha, Paksha::Krsna) => ("EK-006", "apara", "Aparā Ekādaśī"),
        (VaishnavaMasa::Asadha, Paksha::Sukla) => ("EK-007", "sayana", "Śayanā Ekādaśī"),
        (VaishnavaMasa::Asadha, Paksha::Krsna) => ("EK-008", "yogini", "Yoginī Ekādaśī"),
        (VaishnavaMasa::Sravana, Paksha::Sukla) => {
            ("EK-009", "pavitropana", "Putradā - Pavitraropani Ekādaśī")
        }
        (VaishnavaMasa::Sravana, Paksha::Krsna) => ("EK-010", "kamika", "Kāmikā Ekādaśī"),
        (VaishnavaMasa::Bhadrapada, Paksha::Sukla) => ("EK-011", "parsva", "Pārśva Ekādaśī"),
        (VaishnavaMasa::Bhadrapada, Paksha::Krsna) => ("EK-012", "aja", "Ajā Ekādaśī"),
        (VaishnavaMasa::Asvina, Paksha::Sukla) => ("EK-013", "pasankusa", "Pāśāṅkuśā Ekādaśī"),
        (VaishnavaMasa::Asvina, Paksha::Krsna) => ("EK-014", "indira", "Indirā Ekādaśī"),
        (VaishnavaMasa::Kartika, Paksha::Sukla) => ("EK-015", "utthana", "Utthāna Ekādaśī"),
        (VaishnavaMasa::Kartika, Paksha::Krsna) => ("EK-016", "rama", "Rāmā Ekādaśī"),
        (VaishnavaMasa::Margasirsa, Paksha::Sukla) => ("EK-017", "mokshada", "Mokṣadā Ekādaśī"),
        (VaishnavaMasa::Margasirsa, Paksha::Krsna) => ("EK-018", "utpanna", "Utpannā Ekādaśī"),
        (VaishnavaMasa::Pausa, Paksha::Sukla) => ("EK-019", "putrada_pausa", "Putradā Ekādaśī"),
        (VaishnavaMasa::Pausa, Paksha::Krsna) => ("EK-020", "saphala", "Saphalā Ekādaśī"),
        (VaishnavaMasa::Magha, Paksha::Sukla) => ("EK-021", "jaya", "Jayā Ekādaśī"),
        (VaishnavaMasa::Magha, Paksha::Krsna) => ("EK-022", "sattila", "Ṣaṭ-tilā Ekādaśī"),
        (VaishnavaMasa::Phalguna, Paksha::Sukla) => ("EK-023", "amalaki", "Āmalakī Ekādaśī"),
        (VaishnavaMasa::Phalguna, Paksha::Krsna) => ("EK-024", "vijaya", "Vijayā Ekādaśī"),
    };

    Observance {
        id,
        slug,
        display_name,
        observance_type: ObservanceType::Ekadasi,
        source: ObservanceSource::MasaPaksha,
    }
}

/// Resolves Mahadvadasi identity from HBV-EK-004 classification output.
pub fn resolve_mahadvadasi_observance(mahadvadasi: MahadvadasiType) -> Observance {
    let (id, slug, display_name) = match mahadvadasi {
        MahadvadasiType::Unmilani => ("MD-001", "unmilani", "Unmīlanī Mahādvādaśī"),
        MahadvadasiType::Vyanjuli => ("MD-002", "vyanjuli", "Vyañjulī Mahādvādaśī"),
        MahadvadasiType::Trisprsa => ("MD-003", "trisprsa", "Triṣpṛṣā Mahādvādaśī"),
        MahadvadasiType::Paksavardhini => ("MD-004", "paksavardhini", "Pakṣavardhinī Mahādvādaśī"),
        MahadvadasiType::Jaya => ("MD-005", "jaya_mahadvadasi", "Jayā Mahādvādaśī"),
        MahadvadasiType::Vijaya => ("MD-006", "vijaya_mahadvadasi", "Vijayā Mahādvādaśī"),
        MahadvadasiType::Jayanti => ("MD-007", "jayanti", "Jayantī Mahādvādaśī"),
        MahadvadasiType::Papanasini => ("MD-008", "papanasini", "Pāpanāśinī Mahādvādaśī"),
    };

    Observance {
        id,
        slug,
        display_name,
        observance_type: ObservanceType::Mahadvadasi,
        source: ObservanceSource::MahadvadasiRule,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        resolve_mahadvadasi_observance, resolve_ordinary_ekadasi_observance, ObservanceSource,
        ObservanceType, VaishnavaMasa,
    };
    use crate::astronomy::Paksha;
    use crate::vaishnava::MahadvadasiType;

    #[test]
    fn resolves_ordinary_ekadasi_from_masa_and_paksha() {
        let observance = resolve_ordinary_ekadasi_observance(VaishnavaMasa::Sravana, Paksha::Krsna);

        assert_eq!(observance.id, "EK-010");
        assert_eq!(observance.slug, "kamika");
        assert_eq!(observance.display_name, "Kāmikā Ekādaśī");
        assert_eq!(observance.observance_type, ObservanceType::Ekadasi);
        assert_eq!(observance.source, ObservanceSource::MasaPaksha);
    }

    #[test]
    fn resolves_mahadvadasi_from_rule_output() {
        let observance = resolve_mahadvadasi_observance(MahadvadasiType::Trisprsa);

        assert_eq!(observance.id, "MD-003");
        assert_eq!(observance.slug, "trisprsa");
        assert_eq!(observance.display_name, "Triṣpṛṣā Mahādvādaśī");
        assert_eq!(observance.observance_type, ObservanceType::Mahadvadasi);
        assert_eq!(observance.source, ObservanceSource::MahadvadasiRule);
    }
}
