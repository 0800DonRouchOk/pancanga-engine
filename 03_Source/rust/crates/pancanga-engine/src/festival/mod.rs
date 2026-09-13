//! Vaishnava Festival Engine foundation.
//!
//! This module is the public domain layer for festival-like observances. In
//! Campaign 61.0 it deliberately covers only the observances already resolved
//! by the certified Ekadasi/Mahadvadasi pipeline. It does not add festival
//! rules, dates, fixtures, or hardcoded civil-day associations.

use crate::vaishnava::{Observance, ObservanceId, ObservanceSource, ObservanceType, VaishnavaMasa};

/// Broad festival domain supported by Pancanga Engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FestivalDomain {
    /// Ekadasi and Mahadvadasi observances.
    Ekadasi,
}

impl FestivalDomain {
    /// Stable API label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Ekadasi => "ekadasi",
        }
    }
}

/// Public festival kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FestivalKind {
    /// Ordinary Ekadasi.
    Ekadasi,

    /// Mahadvadasi.
    Mahadvadasi,
}

impl FestivalKind {
    /// Stable API label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Ekadasi => "ekadasi",
            Self::Mahadvadasi => "mahadvadasi",
        }
    }
}

impl From<ObservanceType> for FestivalKind {
    fn from(value: ObservanceType) -> Self {
        match value {
            ObservanceType::Ekadasi => Self::Ekadasi,
            ObservanceType::Mahadvadasi => Self::Mahadvadasi,
        }
    }
}

/// Stable festival identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FestivalId(pub ObservanceId);

impl FestivalId {
    /// Stable external label.
    pub fn label(self) -> &'static str {
        self.0.label()
    }
}

/// Unified festival output for interfaces, APIs, and future catalogs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Festival {
    /// Stable catalog identifier.
    pub id: FestivalId,

    /// Technical content key used to load festival-library files.
    pub slug: &'static str,

    /// Human-readable display name.
    pub display_name: &'static str,

    /// Current festival domain.
    pub domain: FestivalDomain,

    /// Specific festival kind inside the domain.
    pub kind: FestivalKind,

    /// Lunar month used for identity, when applicable.
    pub masa: Option<VaishnavaMasa>,

    /// Related observance for Mahadvadasi fasting relationships, when known.
    pub fasting_for: Option<FestivalId>,

    /// Why this festival identity was selected.
    pub source: FestivalSource,
}

/// Diagnostic source for festival identity resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FestivalSource {
    /// Festival identity came from the certified observance resolver.
    ObservanceResolver(ObservanceSource),
}

impl FestivalSource {
    /// Stable API label.
    pub fn label(self) -> &'static str {
        match self {
            Self::ObservanceResolver(source) => source.label(),
        }
    }
}

/// Converts a certified observance into the unified festival domain output.
pub fn festival_from_observance(observance: Observance) -> Festival {
    Festival {
        id: FestivalId(observance.id),
        slug: observance.slug,
        display_name: observance.display_name,
        domain: FestivalDomain::Ekadasi,
        kind: FestivalKind::from(observance.observance_type),
        masa: observance.masa,
        fasting_for: observance.fasting_for.map(FestivalId),
        source: FestivalSource::ObservanceResolver(observance.source),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        festival_from_observance, FestivalDomain, FestivalId, FestivalKind, FestivalSource,
    };
    use crate::astronomy::Paksha;
    use crate::vaishnava::{
        resolve_observance, resolve_ordinary_ekadasi_observance, MahadvadasiType, ObservanceId,
        ObservanceSource, VaishnavaMasa,
    };

    #[test]
    fn wraps_ordinary_ekadasi_as_festival_without_changing_identity() {
        let observance = resolve_ordinary_ekadasi_observance(VaishnavaMasa::Sravana, Paksha::Krsna);
        let festival = festival_from_observance(observance);

        assert_eq!(festival.id, FestivalId(ObservanceId::Ek010));
        assert_eq!(festival.slug, "kamika");
        assert_eq!(festival.display_name, "Kāmikā Ekādaśī");
        assert_eq!(festival.domain, FestivalDomain::Ekadasi);
        assert_eq!(festival.kind, FestivalKind::Ekadasi);
        assert_eq!(festival.masa, Some(VaishnavaMasa::Sravana));
        assert_eq!(festival.fasting_for, None);
        assert_eq!(
            festival.source,
            FestivalSource::ObservanceResolver(ObservanceSource::MasaPaksha)
        );
    }

    #[test]
    fn preserves_mahadvadasi_fasting_relationship() {
        let observance = resolve_observance(
            Some(MahadvadasiType::Trisprsa),
            Some(VaishnavaMasa::Sravana),
            Paksha::Krsna,
        )
        .expect("mahadvadasi observance resolves");
        let festival = festival_from_observance(observance);

        assert_eq!(festival.id, FestivalId(ObservanceId::Md003));
        assert_eq!(festival.slug, "trisprsa");
        assert_eq!(festival.kind, FestivalKind::Mahadvadasi);
        assert_eq!(festival.fasting_for, Some(FestivalId(ObservanceId::Ek010)));
        assert_eq!(
            festival.source,
            FestivalSource::ObservanceResolver(ObservanceSource::MahadvadasiRule)
        );
    }
}
