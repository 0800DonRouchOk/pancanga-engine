//! Vaishnava Engine.
//!
//! This module interprets calendar facts according to the frozen Knowledge
//! Base. It does not calculate astronomy, sunrise, tithi transitions,
//! Mahadvadasi, parana, nakshatra, or festivals.

mod ekadasi;

pub use ekadasi::{
    arunodaya_start, classify_mahadvadasi, classify_vaishnava_day, ekadasi_candidate_at_sunrise,
    ekadasi_candidate_for_civil_day, hari_vasara_end, invalidate_viddha_candidate,
    observance_displacement, parana_window, EkadasiCandidate, EkadasiObservanceDisposition,
    MahadvadasiClassification, MahadvadasiFacts, MahadvadasiType, ParanaInput, ParanaMode,
    ParanaWindow, ParanaWindowError, TithiMahadvadasiCondition, VaishnavaDayClassification,
    VaishnavaDayInput, VaishnavaEngineError, ViddhaCandidateStatus,
    ARUNODAYA_BEFORE_SUNRISE_MINUTES,
};
