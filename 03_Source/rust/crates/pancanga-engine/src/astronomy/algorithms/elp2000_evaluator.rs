//! ELP2000 Interpreter.
//!
//! This module will contain the semantic evaluator for ELP2000-82B/85.
//!
//! It is intentionally not a generic trigonometric table evaluator. The
//! imported coefficient files preserve IMCCE data; this interpreter applies the
//! theory-specific rules needed to obtain lunar periodic corrections.

use crate::core::math::constants::PI;
use crate::core::{ArcSeconds, JulianDate, Radians};

#[allow(dead_code, clippy::approx_constant)]
#[path = "../../../../../../../06_Data/Reference/ELP2000/generated/elp2000_coefficients.rs"]
#[rustfmt::skip]
mod coefficients;

/// Interprets ELP2000-82B/85 lunar terms.
///
/// The interpreter is responsible for applying the theory semantics. It does
/// not calculate `W1`, final lunar longitude, apparent longitude, tithi, or any
/// calendar rule.
#[derive(Debug, Clone, Copy, Default)]
pub struct Elp2000Evaluator;

/// Identifies a generated ELP2000 data family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElpFamily {
    /// `ELP1`: main problem longitude periodic terms.
    Elp1,
    /// `ELP4`: Earth figure perturbations, longitude.
    Elp4,
    /// `ELP7`: Earth figure perturbations, longitude terms proportional to `t`.
    Elp7,
    /// `ELP10`: planetary perturbations table 1, longitude.
    Elp10,
    /// `ELP13`: planetary perturbations table 1, longitude terms proportional to `t`.
    Elp13,
    /// `ELP16`: planetary perturbations table 2, longitude.
    Elp16,
    /// `ELP19`: planetary perturbations table 2, longitude terms proportional to `t`.
    Elp19,
    /// `ELP22`: tidal effects, longitude.
    Elp22,
    /// `ELP25`: tidal effects, longitude terms proportional to `t`.
    Elp25,
    /// `ELP28`: Moon figure perturbations, longitude.
    Elp28,
    /// `ELP31`: relativistic perturbations, longitude.
    Elp31,
    /// `ELP34`: solar eccentricity perturbations, longitude terms proportional to `t²`.
    Elp34,
}

/// Lunar magnitude represented by one ELP2000 family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElpMagnitude {
    /// Ecliptic longitude contribution.
    Longitude,
}

/// Semantic evaluator assigned to one ELP2000 family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElpSemanticEvaluator {
    /// Main-problem evaluator for `ELP1`.
    MainProblem,
    /// Short-periodic evaluator for Earth figure, tides, Moon figure,
    /// relativistic, and solar-eccentricity short series.
    ShortPeriodic,
    /// Planetary evaluator for ELP2000 planetary perturbation tables.
    Planetary,
}

/// Temporal order of one ELP2000 family contribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElpTemporalOrder {
    /// Zeroth-order contribution: `t^0`.
    Order0,
    /// First-order contribution: `t^1`.
    Order1,
    /// Second-order contribution: `t^2`.
    Order2,
}

impl ElpTemporalOrder {
    /// Returns the exponent of `t` represented by this temporal order.
    pub fn exponent(self) -> u8 {
        match self {
            Self::Order0 => 0,
            Self::Order1 => 1,
            Self::Order2 => 2,
        }
    }
}

/// Internal semantic definition for one ELP2000 family.
///
/// This is the interpreter map: it connects an official ELP family to the
/// semantic evaluator required by the theory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElpFamilyDefinition {
    family: ElpFamily,
    info: ElpFamilyInfo,
    magnitude: ElpMagnitude,
    semantic_evaluator: ElpSemanticEvaluator,
    temporal_order: ElpTemporalOrder,
}

impl ElpFamilyDefinition {
    /// Returns the official ELP family identifier.
    pub fn family(self) -> ElpFamily {
        self.family
    }

    /// Returns structural metadata for the family.
    pub fn info(self) -> ElpFamilyInfo {
        self.info
    }

    /// Returns the lunar magnitude represented by this family.
    pub fn magnitude(self) -> ElpMagnitude {
        self.magnitude
    }

    /// Returns the semantic evaluator assigned to this family.
    pub fn semantic_evaluator(self) -> ElpSemanticEvaluator {
        self.semantic_evaluator
    }

    /// Returns the temporal order assigned to this family.
    pub fn temporal_order(self) -> ElpTemporalOrder {
        self.temporal_order
    }
}

/// Structural metadata for one generated ELP2000 family.
///
/// This type intentionally exposes only metadata. It does not expose term
/// evaluation and does not imply that the family has been interpreted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElpFamilyInfo {
    name: &'static str,
    term_count: usize,
    fingerprint: u64,
}

impl ElpFamilyInfo {
    /// Creates structural metadata for a generated ELP2000 family.
    pub fn new(name: &'static str, term_count: usize, fingerprint: u64) -> Self {
        Self {
            name,
            term_count,
            fingerprint,
        }
    }

    /// Returns the official ELP file name for this family.
    pub fn name(self) -> &'static str {
        self.name
    }

    /// Returns the number of generated scientific terms in this family.
    pub fn term_count(self) -> usize {
        self.term_count
    }

    /// Returns the deterministic fingerprint of the source family.
    pub fn fingerprint(self) -> u64 {
        self.fingerprint
    }
}

/// Audit result for one interpreted ELP2000 family.
///
/// This type is intentionally separate from the final `AST-L002` return value.
/// It exists to validate one generated family at a time while preserving the
/// theory-specific trace needed for scientific review.
#[derive(Debug, Clone, PartialEq)]
pub enum ElpFamilyAudit {
    /// Main-problem terms evaluated with corrected amplitudes and sine
    /// arguments.
    MainProblem {
        /// Structural metadata for the audited family.
        info: ElpFamilyInfo,
        /// Term-by-term evaluation trace.
        terms: Vec<ElpMainProblemTermEvaluation>,
    },
}

impl ElpFamilyAudit {
    /// Returns structural metadata for the audited family.
    pub fn info(&self) -> ElpFamilyInfo {
        match self {
            Self::MainProblem { info, .. } => *info,
        }
    }

    /// Returns the number of evaluated terms.
    pub fn term_count(&self) -> usize {
        match self {
            Self::MainProblem { terms, .. } => terms.len(),
        }
    }

    /// Returns the main-problem evaluation trace, when this audit represents a
    /// main-problem family.
    pub fn main_problem_terms(&self) -> Option<&[ElpMainProblemTermEvaluation]> {
        match self {
            Self::MainProblem { terms, .. } => Some(terms),
        }
    }
}

/// Step-by-step evaluation of one main-problem ELP term.
///
/// This is an audit structure used while bringing up the ELP2000 interpreter
/// one term at a time.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ElpMainProblemTermEvaluation {
    argument: Radians,
    corrected_amplitude: ArcSeconds,
    contribution: ArcSeconds,
}

impl ElpMainProblemTermEvaluation {
    /// Creates an evaluated main-problem term trace.
    pub fn new(
        argument: Radians,
        corrected_amplitude: ArcSeconds,
        contribution: ArcSeconds,
    ) -> Self {
        Self {
            argument,
            corrected_amplitude,
            contribution,
        }
    }

    /// Returns the trigonometric argument used by the term.
    pub fn argument(self) -> Radians {
        self.argument
    }

    /// Returns the corrected amplitude in arcseconds.
    pub fn corrected_amplitude(self) -> ArcSeconds {
        self.corrected_amplitude
    }

    /// Returns the signed contribution in arcseconds.
    pub fn contribution(self) -> ArcSeconds {
        self.contribution
    }
}

/// Step-by-step evaluation of one short-periodic ELP term.
///
/// This audit structure exists to validate the `ShortPeriodic` semantics using
/// the first official `ELP4` term before expanding to subsets or complete
/// families.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ElpShortPeriodicTermEvaluation {
    phase: Radians,
    zeta: Radians,
    iz: i16,
    argument: Radians,
    xx: ArcSeconds,
    contribution: ArcSeconds,
}

impl ElpShortPeriodicTermEvaluation {
    /// Creates an evaluated short-periodic term trace.
    pub fn new(
        phase: Radians,
        zeta: Radians,
        iz: i16,
        argument: Radians,
        xx: ArcSeconds,
        contribution: ArcSeconds,
    ) -> Self {
        Self {
            phase,
            zeta,
            iz,
            argument,
            xx,
            contribution,
        }
    }

    /// Returns the constant phase in radians.
    pub fn phase(self) -> Radians {
        self.phase
    }

    /// Returns the ELP `zeta(t)` argument in radians.
    pub fn zeta(self) -> Radians {
        self.zeta
    }

    /// Returns `iz`, the integer multiplier applied to `zeta`.
    pub fn iz(self) -> i16 {
        self.iz
    }

    /// Returns the trigonometric argument used by the term.
    pub fn argument(self) -> Radians {
        self.argument
    }

    /// Returns `xx`, the short-periodic amplitude in arcseconds.
    pub fn xx(self) -> ArcSeconds {
        self.xx
    }

    /// Returns the signed contribution in arcseconds.
    pub fn contribution(self) -> ArcSeconds {
        self.contribution
    }
}

/// Step-by-step evaluation of one planetary ELP term.
///
/// This audit structure exists to validate the `Planetary` semantics using the
/// first official `ELP10` term before expanding to subsets or complete
/// families.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ElpPlanetaryTermEvaluation {
    phase: Radians,
    argument: Radians,
    amplitude: ArcSeconds,
    contribution: ArcSeconds,
}

impl ElpPlanetaryTermEvaluation {
    /// Creates an evaluated planetary term trace.
    pub fn new(
        phase: Radians,
        argument: Radians,
        amplitude: ArcSeconds,
        contribution: ArcSeconds,
    ) -> Self {
        Self {
            phase,
            argument,
            amplitude,
            contribution,
        }
    }

    /// Returns the constant phase in radians.
    pub fn phase(self) -> Radians {
        self.phase
    }

    /// Returns the trigonometric argument used by the term.
    pub fn argument(self) -> Radians {
        self.argument
    }

    /// Returns the effective amplitude in arcseconds.
    pub fn amplitude(self) -> ArcSeconds {
        self.amplitude
    }

    /// Returns the signed contribution in arcseconds.
    pub fn contribution(self) -> ArcSeconds {
        self.contribution
    }
}

impl Elp2000Evaluator {
    /// Creates an ELP2000 interpreter.
    pub fn new() -> Self {
        Self
    }

    /// Returns structural metadata for a generated ELP2000 family.
    ///
    /// This is an inspection-only API. It verifies that the interpreter can see
    /// the generated data without evaluating any term.
    pub fn inspect_family(self, family: ElpFamily) -> ElpFamilyInfo {
        let _ = self;

        family_definition(family).info()
    }

    /// Returns the semantic definition assigned to a generated ELP2000 family.
    ///
    /// This exposes the interpreter map without evaluating any term.
    pub fn family_definition(self, family: ElpFamily) -> ElpFamilyDefinition {
        let _ = self;

        family_definition(family)
    }

    /// Audits one generated ELP2000 family.
    ///
    /// This is the general family-audit entry point. It does not imply that the
    /// complete `AST-L002` periodic longitude correction has been implemented.
    pub fn audit_family(self, family: ElpFamily, jd: JulianDate) -> ElpFamilyAudit {
        let info = self.inspect_family(family);
        let t = julian_centuries_tdb(jd);

        match family {
            ElpFamily::Elp1 => ElpFamilyAudit::MainProblem {
                info,
                terms: evaluate_main_problem_longitude_family_for_audit(coefficients::ELP1, t),
            },
            _ => todo!("semantic evaluator for this ELP2000 family is not implemented yet"),
        }
    }

    /// Evaluates only the first official `ELP1` term for audit.
    ///
    /// This method exists to validate the semantics of one main-problem
    /// longitude term before the interpreter expands to multiple terms.
    ///
    /// It does not evaluate the full `ELP1` family and does not implement
    /// `AST-L002`.
    pub fn evaluate_first_elp1_longitude_term_for_audit(
        self,
        jd: JulianDate,
    ) -> ElpMainProblemTermEvaluation {
        let _ = self;

        evaluate_elp1_longitude_terms_for_audit::<1>(jd)[0]
    }

    /// Evaluates the first `N` official `ELP1` terms for audit.
    ///
    /// This uses the same term evaluator as the single-term audit path. It is
    /// the interpreter-subset validation mechanism for `ELP1`.
    ///
    /// It does not evaluate the full `ELP1` family and does not implement
    /// `AST-L002`.
    pub fn evaluate_elp1_longitude_subset_for_audit<const N: usize>(
        self,
        jd: JulianDate,
    ) -> [ElpMainProblemTermEvaluation; N] {
        let _ = self;

        evaluate_elp1_longitude_terms_for_audit::<N>(jd)
    }

    /// Evaluates the first ten official `ELP1` terms for audit.
    ///
    /// This convenience method delegates to the generic subset mechanism.
    pub fn evaluate_first_ten_elp1_longitude_terms_for_audit(
        self,
        jd: JulianDate,
    ) -> [ElpMainProblemTermEvaluation; 10] {
        self.evaluate_elp1_longitude_subset_for_audit::<10>(jd)
    }

    /// Evaluates the complete official `ELP1` longitude family for audit.
    ///
    /// This closes the main-problem longitude family validation path. It still
    /// does not implement `AST-L002`, because the full algorithm also needs the
    /// remaining longitude families and their ELP2000-specific semantics.
    pub fn evaluate_full_elp1_longitude_family_for_audit(
        self,
        jd: JulianDate,
    ) -> [ElpMainProblemTermEvaluation; coefficients::ELP1_RECORDS] {
        self.evaluate_elp1_longitude_subset_for_audit::<{ coefficients::ELP1_RECORDS }>(jd)
    }

    /// Evaluates only the first official `ELP4` term for audit.
    ///
    /// This method validates the `ShortPeriodic` semantic evaluator on one
    /// official Earth-figure longitude term. It does not evaluate the full
    /// `ELP4` family and does not implement `AST-L002`.
    pub fn evaluate_first_elp4_longitude_term_for_audit(
        self,
        jd: JulianDate,
    ) -> ElpShortPeriodicTermEvaluation {
        self.evaluate_elp4_longitude_subset_for_audit::<1>(jd)[0]
    }

    /// Evaluates the first `N` official `ELP4` terms for audit.
    ///
    /// This uses the same short-periodic term evaluator as the single-term
    /// audit path. It does not evaluate the full `ELP4` family and does not
    /// implement `AST-L002`.
    pub fn evaluate_elp4_longitude_subset_for_audit<const N: usize>(
        self,
        jd: JulianDate,
    ) -> [ElpShortPeriodicTermEvaluation; N] {
        let _ = self;

        ShortPeriodicEvaluator::evaluate_elp4_longitude_terms_for_audit::<N>(jd)
    }

    /// Evaluates the first ten official `ELP4` terms for audit.
    ///
    /// This convenience method delegates to the generic subset mechanism.
    pub fn evaluate_first_ten_elp4_longitude_terms_for_audit(
        self,
        jd: JulianDate,
    ) -> [ElpShortPeriodicTermEvaluation; 10] {
        self.evaluate_elp4_longitude_subset_for_audit::<10>(jd)
    }

    /// Evaluates the first one hundred official `ELP4` terms for audit.
    ///
    /// This convenience method delegates to the generic subset mechanism.
    pub fn evaluate_first_hundred_elp4_longitude_terms_for_audit(
        self,
        jd: JulianDate,
    ) -> [ElpShortPeriodicTermEvaluation; 100] {
        self.evaluate_elp4_longitude_subset_for_audit::<100>(jd)
    }

    /// Evaluates only the first official `ELP7` term for audit.
    ///
    /// This method validates the already proven `ShortPeriodic` semantic
    /// evaluator with the first temporal order `t^1`.
    pub fn evaluate_first_elp7_longitude_term_for_audit(
        self,
        jd: JulianDate,
    ) -> ElpShortPeriodicTermEvaluation {
        self.evaluate_elp7_longitude_subset_for_audit::<1>(jd)[0]
    }

    /// Evaluates the first `N` official `ELP7` terms for audit.
    ///
    /// This uses the same short-periodic term evaluator as `ELP4`; the family
    /// definition supplies the temporal order.
    pub fn evaluate_elp7_longitude_subset_for_audit<const N: usize>(
        self,
        jd: JulianDate,
    ) -> [ElpShortPeriodicTermEvaluation; N] {
        let _ = self;

        ShortPeriodicEvaluator::evaluate_elp7_longitude_terms_for_audit::<N>(jd)
    }

    /// Evaluates the first ten official `ELP7` terms for audit.
    ///
    /// This convenience method delegates to the generic subset mechanism.
    pub fn evaluate_first_ten_elp7_longitude_terms_for_audit(
        self,
        jd: JulianDate,
    ) -> [ElpShortPeriodicTermEvaluation; 10] {
        self.evaluate_elp7_longitude_subset_for_audit::<10>(jd)
    }

    /// Evaluates only the first official `ELP22` term for audit.
    ///
    /// This method reuses the already proven `ShortPeriodic` semantic
    /// evaluator with the zeroth temporal order `t^0`.
    pub fn evaluate_first_elp22_longitude_term_for_audit(
        self,
        jd: JulianDate,
    ) -> ElpShortPeriodicTermEvaluation {
        self.evaluate_elp22_longitude_subset_for_audit::<1>(jd)[0]
    }

    /// Evaluates the first `N` official `ELP22` terms for audit.
    ///
    /// This uses the same short-periodic term evaluator as `ELP4` and `ELP7`.
    pub fn evaluate_elp22_longitude_subset_for_audit<const N: usize>(
        self,
        jd: JulianDate,
    ) -> [ElpShortPeriodicTermEvaluation; N] {
        let _ = self;

        ShortPeriodicEvaluator::evaluate_elp22_longitude_terms_for_audit::<N>(jd)
    }

    /// Evaluates the complete official `ELP25` longitude family for audit.
    ///
    /// This reuses the already proven `ShortPeriodic` semantic evaluator with
    /// the first temporal order `t^1`.
    pub fn evaluate_full_elp25_longitude_family_for_audit(
        self,
        jd: JulianDate,
    ) -> [ElpShortPeriodicTermEvaluation; coefficients::ELP25_RECORDS] {
        let _ = self;

        ShortPeriodicEvaluator::evaluate_elp25_longitude_terms_for_audit::<
            { coefficients::ELP25_RECORDS },
        >(jd)
    }

    /// Evaluates the complete official `ELP28` longitude family for audit.
    ///
    /// This reuses the already proven `ShortPeriodic` semantic evaluator with
    /// the zeroth temporal order `t^0`.
    pub fn evaluate_full_elp28_longitude_family_for_audit(
        self,
        jd: JulianDate,
    ) -> [ElpShortPeriodicTermEvaluation; coefficients::ELP28_RECORDS] {
        let _ = self;

        ShortPeriodicEvaluator::evaluate_elp28_longitude_terms_for_audit::<
            { coefficients::ELP28_RECORDS },
        >(jd)
    }

    /// Evaluates the complete official `ELP31` longitude family for audit.
    ///
    /// This reuses the already proven `ShortPeriodic` semantic evaluator with
    /// the zeroth temporal order `t^0`.
    pub fn evaluate_full_elp31_longitude_family_for_audit(
        self,
        jd: JulianDate,
    ) -> [ElpShortPeriodicTermEvaluation; coefficients::ELP31_RECORDS] {
        let _ = self;

        ShortPeriodicEvaluator::evaluate_elp31_longitude_terms_for_audit::<
            { coefficients::ELP31_RECORDS },
        >(jd)
    }

    /// Evaluates the complete official `ELP34` longitude family for audit.
    ///
    /// This reuses the already proven `ShortPeriodic` semantic evaluator with
    /// the second temporal order `t^2`.
    pub fn evaluate_full_elp34_longitude_family_for_audit(
        self,
        jd: JulianDate,
    ) -> [ElpShortPeriodicTermEvaluation; coefficients::ELP34_RECORDS] {
        let _ = self;

        ShortPeriodicEvaluator::evaluate_elp34_longitude_terms_for_audit::<
            { coefficients::ELP34_RECORDS },
        >(jd)
    }

    /// Evaluates only the first official `ELP10` term for audit.
    ///
    /// This method validates the `Planetary` semantic evaluator on one official
    /// planetary longitude term. It does not evaluate the full `ELP10` family
    /// and does not implement `AST-L002`.
    pub fn evaluate_first_elp10_longitude_term_for_audit(
        self,
        jd: JulianDate,
    ) -> ElpPlanetaryTermEvaluation {
        self.evaluate_elp10_longitude_subset_for_audit::<1>(jd)[0]
    }

    /// Evaluates the first `N` official `ELP10` terms for audit.
    ///
    /// This uses the same planetary term evaluator as the single-term audit
    /// path. It does not evaluate the full `ELP10` family and does not
    /// implement `AST-L002`.
    pub fn evaluate_elp10_longitude_subset_for_audit<const N: usize>(
        self,
        jd: JulianDate,
    ) -> [ElpPlanetaryTermEvaluation; N] {
        let _ = self;

        PlanetaryEvaluator::evaluate_elp10_longitude_terms_for_audit::<N>(jd)
    }

    /// Evaluates the complete official `ELP10` longitude family for audit.
    ///
    /// This closes the planetary order-zero family validation path. It still
    /// does not implement `AST-L002`, because the full algorithm also needs the
    /// remaining longitude families.
    pub fn evaluate_full_elp10_longitude_family_for_audit(
        self,
        jd: JulianDate,
    ) -> Vec<ElpPlanetaryTermEvaluation> {
        let _ = self;

        PlanetaryEvaluator::evaluate_full_elp10_longitude_family_for_audit(jd)
    }

    /// Evaluates only the first official `ELP13` term for audit.
    ///
    /// This method validates the already proven `Planetary` semantic evaluator
    /// with the first temporal order `t^1`.
    pub fn evaluate_first_elp13_longitude_term_for_audit(
        self,
        jd: JulianDate,
    ) -> ElpPlanetaryTermEvaluation {
        self.evaluate_elp13_longitude_subset_for_audit::<1>(jd)[0]
    }

    /// Evaluates the first `N` official `ELP13` terms for audit.
    ///
    /// This uses the same planetary term evaluator as `ELP10`; the family
    /// definition supplies the temporal order.
    pub fn evaluate_elp13_longitude_subset_for_audit<const N: usize>(
        self,
        jd: JulianDate,
    ) -> [ElpPlanetaryTermEvaluation; N] {
        let _ = self;

        PlanetaryEvaluator::evaluate_elp13_longitude_terms_for_audit::<N>(jd)
    }

    /// Evaluates the complete official `ELP13` longitude family for audit.
    ///
    /// This closes the planetary order-one family validation path. It still
    /// does not implement `AST-L002`, because the full algorithm also needs the
    /// remaining longitude families.
    pub fn evaluate_full_elp13_longitude_family_for_audit(
        self,
        jd: JulianDate,
    ) -> Vec<ElpPlanetaryTermEvaluation> {
        let _ = self;

        PlanetaryEvaluator::evaluate_full_elp13_longitude_family_for_audit(jd)
    }

    /// Evaluates the complete official `ELP16` longitude family for audit.
    ///
    /// This validates the second planetary table with the zeroth temporal
    /// order. It still does not implement `AST-L002`, because the full
    /// algorithm also needs the remaining longitude families.
    pub fn evaluate_full_elp16_longitude_family_for_audit(
        self,
        jd: JulianDate,
    ) -> Vec<ElpPlanetaryTermEvaluation> {
        let _ = self;

        PlanetaryEvaluator::evaluate_full_elp16_longitude_family_for_audit(jd)
    }

    /// Evaluates the complete official `ELP19` longitude family for audit.
    ///
    /// This validates the second planetary table with the first temporal
    /// order. It still does not implement `AST-L002`, because the full
    /// algorithm also needs the remaining longitude families.
    pub fn evaluate_full_elp19_longitude_family_for_audit(
        self,
        jd: JulianDate,
    ) -> Vec<ElpPlanetaryTermEvaluation> {
        let _ = self;

        PlanetaryEvaluator::evaluate_full_elp19_longitude_family_for_audit(jd)
    }

    /// Evaluates the currently validated partial periodic longitude correction
    /// `Δλ`.
    ///
    /// This audit method integrates only the longitude families whose
    /// semantics have already been validated: `ELP1`, `ELP4`, `ELP7`, `ELP10`,
    /// `ELP13`, `ELP16`, `ELP19`, and `ELP22`.
    ///
    /// It does not add `W1`, does not calculate lunar longitude, and does not
    /// include the remaining `AST-L002` longitude families.
    pub fn evaluate_validated_longitude_correction_partial_for_audit(
        self,
        jd: JulianDate,
    ) -> ArcSeconds {
        let _ = self;

        ArcSeconds::new(validated_longitude_correction_partial(jd))
    }

    /// Evaluates the periodic longitude correction `Δλ`.
    ///
    /// The returned value is measured in signed arcseconds.
    ///
    /// This is the AST-L002 entry point for the ELP2000 interpreter. It sums
    /// only the twelve validated ELP2000 longitude families and does not add
    /// `W1`, nutation, apparent longitude, tithi, or calendar rules.
    pub fn evaluate_longitude_correction(self, jd: JulianDate) -> ArcSeconds {
        let _ = self;

        ArcSeconds::new(longitude_correction(jd))
    }
}

/// Semantic evaluator for ELP2000 short-periodic families.
///
/// It interprets one short-periodic term and returns its individual
/// contribution. It does not calculate `W1`, final lunar longitude, nutation,
/// tithi, calendar events, or Vaishnava rules.
#[derive(Debug, Clone, Copy, Default)]
pub struct ShortPeriodicEvaluator;

impl ShortPeriodicEvaluator {
    fn evaluate_elp4_longitude_terms_for_audit<const N: usize>(
        jd: JulianDate,
    ) -> [ElpShortPeriodicTermEvaluation; N] {
        let definition = family_definition(ElpFamily::Elp4);

        assert_eq!(
            definition.semantic_evaluator(),
            ElpSemanticEvaluator::ShortPeriodic
        );
        assert_eq!(definition.temporal_order(), ElpTemporalOrder::Order0);
        let t = julian_centuries_tdb(jd);

        Self::evaluate_terms::<N>(coefficients::ELP4, definition, t)
    }

    fn evaluate_elp7_longitude_terms_for_audit<const N: usize>(
        jd: JulianDate,
    ) -> [ElpShortPeriodicTermEvaluation; N] {
        let definition = family_definition(ElpFamily::Elp7);

        assert_eq!(
            definition.semantic_evaluator(),
            ElpSemanticEvaluator::ShortPeriodic
        );
        assert_eq!(definition.temporal_order(), ElpTemporalOrder::Order1);
        let t = julian_centuries_tdb(jd);

        Self::evaluate_terms::<N>(coefficients::ELP7, definition, t)
    }

    fn evaluate_elp22_longitude_terms_for_audit<const N: usize>(
        jd: JulianDate,
    ) -> [ElpShortPeriodicTermEvaluation; N] {
        let definition = family_definition(ElpFamily::Elp22);

        assert_eq!(
            definition.semantic_evaluator(),
            ElpSemanticEvaluator::ShortPeriodic
        );
        assert_eq!(definition.temporal_order(), ElpTemporalOrder::Order0);
        let t = julian_centuries_tdb(jd);

        Self::evaluate_terms::<N>(coefficients::ELP22, definition, t)
    }

    fn evaluate_elp25_longitude_terms_for_audit<const N: usize>(
        jd: JulianDate,
    ) -> [ElpShortPeriodicTermEvaluation; N] {
        let definition = family_definition(ElpFamily::Elp25);

        assert_eq!(
            definition.semantic_evaluator(),
            ElpSemanticEvaluator::ShortPeriodic
        );
        assert_eq!(definition.temporal_order(), ElpTemporalOrder::Order1);
        let t = julian_centuries_tdb(jd);

        Self::evaluate_terms::<N>(coefficients::ELP25, definition, t)
    }

    fn evaluate_elp28_longitude_terms_for_audit<const N: usize>(
        jd: JulianDate,
    ) -> [ElpShortPeriodicTermEvaluation; N] {
        let definition = family_definition(ElpFamily::Elp28);

        assert_eq!(
            definition.semantic_evaluator(),
            ElpSemanticEvaluator::ShortPeriodic
        );
        assert_eq!(definition.temporal_order(), ElpTemporalOrder::Order0);
        let t = julian_centuries_tdb(jd);

        Self::evaluate_terms::<N>(coefficients::ELP28, definition, t)
    }

    fn evaluate_elp31_longitude_terms_for_audit<const N: usize>(
        jd: JulianDate,
    ) -> [ElpShortPeriodicTermEvaluation; N] {
        let definition = family_definition(ElpFamily::Elp31);

        assert_eq!(
            definition.semantic_evaluator(),
            ElpSemanticEvaluator::ShortPeriodic
        );
        assert_eq!(definition.temporal_order(), ElpTemporalOrder::Order0);
        let t = julian_centuries_tdb(jd);

        Self::evaluate_terms::<N>(coefficients::ELP31, definition, t)
    }

    fn evaluate_elp34_longitude_terms_for_audit<const N: usize>(
        jd: JulianDate,
    ) -> [ElpShortPeriodicTermEvaluation; N] {
        let definition = family_definition(ElpFamily::Elp34);

        assert_eq!(
            definition.semantic_evaluator(),
            ElpSemanticEvaluator::ShortPeriodic
        );
        assert_eq!(definition.temporal_order(), ElpTemporalOrder::Order2);
        let t = julian_centuries_tdb(jd);

        Self::evaluate_terms::<N>(coefficients::ELP34, definition, t)
    }

    fn evaluate_terms<const N: usize>(
        terms: &[coefficients::ElpShortPeriodicTerm],
        definition: ElpFamilyDefinition,
        t: f64,
    ) -> [ElpShortPeriodicTermEvaluation; N] {
        std::array::from_fn(|index| Self::evaluate_term(terms[index], definition, t))
    }

    fn evaluate_term(
        term: coefficients::ElpShortPeriodicTerm,
        definition: ElpFamilyDefinition,
        t: f64,
    ) -> ElpShortPeriodicTermEvaluation {
        let phase = Radians::new(term.phase_degrees * DEG_TO_RADIAN);
        let zeta = Radians::new(evaluate_zeta(t));
        let argument = short_periodic_argument(term, phase, zeta, t);
        let xx = ArcSeconds::new(effective_short_periodic_amplitude(term, definition, t));
        let contribution = ArcSeconds::new(xx.value() * argument.value().sin());

        ElpShortPeriodicTermEvaluation::new(
            phase,
            zeta,
            term.zeta_multiplier,
            argument,
            xx,
            contribution,
        )
    }
}

/// Semantic evaluator for ELP2000 planetary families.
///
/// It interprets one planetary perturbation term and returns its individual
/// contribution. It does not calculate `W1`, final lunar longitude, nutation,
/// tithi, calendar events, or Vaishnava rules.
#[derive(Debug, Clone, Copy, Default)]
pub struct PlanetaryEvaluator;

impl PlanetaryEvaluator {
    fn evaluate_elp10_longitude_terms_for_audit<const N: usize>(
        jd: JulianDate,
    ) -> [ElpPlanetaryTermEvaluation; N] {
        let definition = family_definition(ElpFamily::Elp10);

        assert_eq!(
            definition.semantic_evaluator(),
            ElpSemanticEvaluator::Planetary
        );
        assert_eq!(definition.temporal_order(), ElpTemporalOrder::Order0);
        let t = julian_centuries_tdb(jd);

        Self::evaluate_terms::<N>(coefficients::ELP10, definition, t)
    }

    fn evaluate_full_elp10_longitude_family_for_audit(
        jd: JulianDate,
    ) -> Vec<ElpPlanetaryTermEvaluation> {
        let definition = family_definition(ElpFamily::Elp10);

        assert_eq!(
            definition.semantic_evaluator(),
            ElpSemanticEvaluator::Planetary
        );
        assert_eq!(definition.temporal_order(), ElpTemporalOrder::Order0);
        let t = julian_centuries_tdb(jd);

        Self::evaluate_all_terms(coefficients::ELP10, definition, t)
    }

    fn evaluate_elp13_longitude_terms_for_audit<const N: usize>(
        jd: JulianDate,
    ) -> [ElpPlanetaryTermEvaluation; N] {
        let definition = family_definition(ElpFamily::Elp13);

        assert_eq!(
            definition.semantic_evaluator(),
            ElpSemanticEvaluator::Planetary
        );
        assert_eq!(definition.temporal_order(), ElpTemporalOrder::Order1);
        let t = julian_centuries_tdb(jd);

        Self::evaluate_terms::<N>(coefficients::ELP13, definition, t)
    }

    fn evaluate_full_elp13_longitude_family_for_audit(
        jd: JulianDate,
    ) -> Vec<ElpPlanetaryTermEvaluation> {
        let definition = family_definition(ElpFamily::Elp13);

        assert_eq!(
            definition.semantic_evaluator(),
            ElpSemanticEvaluator::Planetary
        );
        assert_eq!(definition.temporal_order(), ElpTemporalOrder::Order1);
        let t = julian_centuries_tdb(jd);

        Self::evaluate_all_terms(coefficients::ELP13, definition, t)
    }

    fn evaluate_full_elp16_longitude_family_for_audit(
        jd: JulianDate,
    ) -> Vec<ElpPlanetaryTermEvaluation> {
        let definition = family_definition(ElpFamily::Elp16);

        assert_eq!(
            definition.semantic_evaluator(),
            ElpSemanticEvaluator::Planetary
        );
        assert_eq!(definition.temporal_order(), ElpTemporalOrder::Order0);
        let t = julian_centuries_tdb(jd);

        Self::evaluate_all_table_two_terms(coefficients::ELP16, definition, t)
    }

    fn evaluate_full_elp19_longitude_family_for_audit(
        jd: JulianDate,
    ) -> Vec<ElpPlanetaryTermEvaluation> {
        let definition = family_definition(ElpFamily::Elp19);

        assert_eq!(
            definition.semantic_evaluator(),
            ElpSemanticEvaluator::Planetary
        );
        assert_eq!(definition.temporal_order(), ElpTemporalOrder::Order1);
        let t = julian_centuries_tdb(jd);

        Self::evaluate_all_table_two_terms(coefficients::ELP19, definition, t)
    }

    fn evaluate_terms<const N: usize>(
        terms: &[coefficients::ElpPlanetaryTerm],
        definition: ElpFamilyDefinition,
        t: f64,
    ) -> [ElpPlanetaryTermEvaluation; N] {
        std::array::from_fn(|index| Self::evaluate_term(terms[index], definition, t))
    }

    fn evaluate_all_terms(
        terms: &[coefficients::ElpPlanetaryTerm],
        definition: ElpFamilyDefinition,
        t: f64,
    ) -> Vec<ElpPlanetaryTermEvaluation> {
        terms
            .iter()
            .copied()
            .map(|term| Self::evaluate_term(term, definition, t))
            .collect()
    }

    fn evaluate_all_table_two_terms(
        terms: &[coefficients::ElpPlanetaryTerm],
        definition: ElpFamilyDefinition,
        t: f64,
    ) -> Vec<ElpPlanetaryTermEvaluation> {
        terms
            .iter()
            .copied()
            .map(|term| Self::evaluate_table_two_term(term, definition, t))
            .collect()
    }

    fn evaluate_term(
        term: coefficients::ElpPlanetaryTerm,
        definition: ElpFamilyDefinition,
        t: f64,
    ) -> ElpPlanetaryTermEvaluation {
        let phase = Radians::new(term.phase_degrees * DEG_TO_RADIAN);
        let argument = planetary_table_one_argument(term, phase, t);
        let amplitude = ArcSeconds::new(effective_planetary_amplitude(term, definition, t));
        let contribution = ArcSeconds::new(amplitude.value() * argument.value().sin());

        ElpPlanetaryTermEvaluation::new(phase, argument, amplitude, contribution)
    }

    fn evaluate_table_two_term(
        term: coefficients::ElpPlanetaryTerm,
        definition: ElpFamilyDefinition,
        t: f64,
    ) -> ElpPlanetaryTermEvaluation {
        let phase = Radians::new(term.phase_degrees * DEG_TO_RADIAN);
        let argument = planetary_table_two_argument(term, phase, t);
        let amplitude = ArcSeconds::new(effective_planetary_amplitude(term, definition, t));
        let contribution = ArcSeconds::new(amplitude.value() * argument.value().sin());

        ElpPlanetaryTermEvaluation::new(phase, argument, amplitude, contribution)
    }
}

const J2000_JULIAN_DAY: f64 = 2_451_545.0;
const DAYS_PER_JULIAN_CENTURY: f64 = 36_525.0;
const ARCSECONDS_PER_RADIAN: f64 = 648_000.0 / PI;
const DEG_TO_RADIAN: f64 = PI / 180.0;
const MINUTES_PER_DEGREE: f64 = 60.0;
const SECONDS_PER_DEGREE: f64 = 3_600.0;

const AM: f64 = 0.074_801_329_518;
const ALPHA: f64 = 0.002_571_881_335;
const DTASM: f64 = 2.0 * ALPHA / (3.0 * AM);

fn family_definition(family: ElpFamily) -> ElpFamilyDefinition {
    match family {
        ElpFamily::Elp1 => definition(
            family,
            "ELP1",
            coefficients::ELP1_RECORDS,
            coefficients::ELP1_FINGERPRINT,
            ElpSemanticEvaluator::MainProblem,
            ElpTemporalOrder::Order0,
        ),
        ElpFamily::Elp4 => definition(
            family,
            "ELP4",
            coefficients::ELP4_RECORDS,
            coefficients::ELP4_FINGERPRINT,
            ElpSemanticEvaluator::ShortPeriodic,
            ElpTemporalOrder::Order0,
        ),
        ElpFamily::Elp7 => definition(
            family,
            "ELP7",
            coefficients::ELP7_RECORDS,
            coefficients::ELP7_FINGERPRINT,
            ElpSemanticEvaluator::ShortPeriodic,
            ElpTemporalOrder::Order1,
        ),
        ElpFamily::Elp10 => definition(
            family,
            "ELP10",
            coefficients::ELP10_RECORDS,
            coefficients::ELP10_FINGERPRINT,
            ElpSemanticEvaluator::Planetary,
            ElpTemporalOrder::Order0,
        ),
        ElpFamily::Elp13 => definition(
            family,
            "ELP13",
            coefficients::ELP13_RECORDS,
            coefficients::ELP13_FINGERPRINT,
            ElpSemanticEvaluator::Planetary,
            ElpTemporalOrder::Order1,
        ),
        ElpFamily::Elp16 => definition(
            family,
            "ELP16",
            coefficients::ELP16_RECORDS,
            coefficients::ELP16_FINGERPRINT,
            ElpSemanticEvaluator::Planetary,
            ElpTemporalOrder::Order0,
        ),
        ElpFamily::Elp19 => definition(
            family,
            "ELP19",
            coefficients::ELP19_RECORDS,
            coefficients::ELP19_FINGERPRINT,
            ElpSemanticEvaluator::Planetary,
            ElpTemporalOrder::Order1,
        ),
        ElpFamily::Elp22 => definition(
            family,
            "ELP22",
            coefficients::ELP22_RECORDS,
            coefficients::ELP22_FINGERPRINT,
            ElpSemanticEvaluator::ShortPeriodic,
            ElpTemporalOrder::Order0,
        ),
        ElpFamily::Elp25 => definition(
            family,
            "ELP25",
            coefficients::ELP25_RECORDS,
            coefficients::ELP25_FINGERPRINT,
            ElpSemanticEvaluator::ShortPeriodic,
            ElpTemporalOrder::Order1,
        ),
        ElpFamily::Elp28 => definition(
            family,
            "ELP28",
            coefficients::ELP28_RECORDS,
            coefficients::ELP28_FINGERPRINT,
            ElpSemanticEvaluator::ShortPeriodic,
            ElpTemporalOrder::Order0,
        ),
        ElpFamily::Elp31 => definition(
            family,
            "ELP31",
            coefficients::ELP31_RECORDS,
            coefficients::ELP31_FINGERPRINT,
            ElpSemanticEvaluator::ShortPeriodic,
            ElpTemporalOrder::Order0,
        ),
        ElpFamily::Elp34 => definition(
            family,
            "ELP34",
            coefficients::ELP34_RECORDS,
            coefficients::ELP34_FINGERPRINT,
            ElpSemanticEvaluator::ShortPeriodic,
            ElpTemporalOrder::Order2,
        ),
    }
}

fn definition(
    family: ElpFamily,
    name: &'static str,
    term_count: usize,
    fingerprint: u64,
    semantic_evaluator: ElpSemanticEvaluator,
    temporal_order: ElpTemporalOrder,
) -> ElpFamilyDefinition {
    ElpFamilyDefinition {
        family,
        info: ElpFamilyInfo::new(name, term_count, fingerprint),
        magnitude: ElpMagnitude::Longitude,
        semantic_evaluator,
        temporal_order,
    }
}

fn julian_centuries_tdb(jd: JulianDate) -> f64 {
    (jd.value() - J2000_JULIAN_DAY) / DAYS_PER_JULIAN_CENTURY
}

fn evaluate_elp1_longitude_terms_for_audit<const N: usize>(
    jd: JulianDate,
) -> [ElpMainProblemTermEvaluation; N] {
    let t = julian_centuries_tdb(jd);

    evaluate_main_problem_longitude_terms_subset_for_audit::<N>(coefficients::ELP1, t)
}

fn evaluate_main_problem_longitude_family_for_audit(
    terms: &[coefficients::ElpMainProblemTerm],
    t: f64,
) -> Vec<ElpMainProblemTermEvaluation> {
    terms
        .iter()
        .copied()
        .map(|term| evaluate_main_problem_longitude_term(term, t))
        .collect()
}

fn evaluate_main_problem_longitude_terms_subset_for_audit<const N: usize>(
    terms: &[coefficients::ElpMainProblemTerm],
    t: f64,
) -> [ElpMainProblemTermEvaluation; N] {
    std::array::from_fn(|index| evaluate_main_problem_longitude_term(terms[index], t))
}

fn validated_longitude_correction_partial(jd: JulianDate) -> f64 {
    validated_longitude_correction_partial_family_totals(jd)
        .iter()
        .map(|(_family, contribution)| contribution)
        .sum()
}

fn longitude_correction(jd: JulianDate) -> f64 {
    longitude_correction_family_totals(jd)
        .iter()
        .map(|(_family, contribution)| contribution)
        .sum()
}

fn longitude_correction_family_totals(jd: JulianDate) -> [(&'static str, f64); 12] {
    [
        (
            "ELP1",
            sum_main_problem_terms(
                &Elp2000Evaluator::new().evaluate_full_elp1_longitude_family_for_audit(jd),
            ),
        ),
        (
            "ELP4",
            sum_short_periodic_terms(
                &Elp2000Evaluator::new()
                    .evaluate_elp4_longitude_subset_for_audit::<{ coefficients::ELP4_RECORDS }>(jd),
            ),
        ),
        (
            "ELP7",
            sum_short_periodic_terms(
                &Elp2000Evaluator::new()
                    .evaluate_elp7_longitude_subset_for_audit::<{ coefficients::ELP7_RECORDS }>(jd),
            ),
        ),
        (
            "ELP10",
            sum_planetary_terms(
                &Elp2000Evaluator::new().evaluate_full_elp10_longitude_family_for_audit(jd),
            ),
        ),
        (
            "ELP13",
            sum_planetary_terms(
                &Elp2000Evaluator::new().evaluate_full_elp13_longitude_family_for_audit(jd),
            ),
        ),
        (
            "ELP16",
            sum_planetary_terms(
                &Elp2000Evaluator::new().evaluate_full_elp16_longitude_family_for_audit(jd),
            ),
        ),
        (
            "ELP19",
            sum_planetary_terms(
                &Elp2000Evaluator::new().evaluate_full_elp19_longitude_family_for_audit(jd),
            ),
        ),
        (
            "ELP22",
            sum_short_periodic_terms(
                &Elp2000Evaluator::new()
                    .evaluate_elp22_longitude_subset_for_audit::<{ coefficients::ELP22_RECORDS }>(
                        jd,
                    ),
            ),
        ),
        (
            "ELP25",
            sum_short_periodic_terms(
                &Elp2000Evaluator::new().evaluate_full_elp25_longitude_family_for_audit(jd),
            ),
        ),
        (
            "ELP28",
            sum_short_periodic_terms(
                &Elp2000Evaluator::new().evaluate_full_elp28_longitude_family_for_audit(jd),
            ),
        ),
        (
            "ELP31",
            sum_short_periodic_terms(
                &Elp2000Evaluator::new().evaluate_full_elp31_longitude_family_for_audit(jd),
            ),
        ),
        (
            "ELP34",
            sum_short_periodic_terms(
                &Elp2000Evaluator::new().evaluate_full_elp34_longitude_family_for_audit(jd),
            ),
        ),
    ]
}

fn validated_longitude_correction_partial_family_totals(
    jd: JulianDate,
) -> [(&'static str, f64); 8] {
    [
        (
            "ELP1",
            sum_main_problem_terms(
                &Elp2000Evaluator::new().evaluate_full_elp1_longitude_family_for_audit(jd),
            ),
        ),
        (
            "ELP4",
            sum_short_periodic_terms(
                &Elp2000Evaluator::new()
                    .evaluate_elp4_longitude_subset_for_audit::<{ coefficients::ELP4_RECORDS }>(jd),
            ),
        ),
        (
            "ELP7",
            sum_short_periodic_terms(
                &Elp2000Evaluator::new()
                    .evaluate_elp7_longitude_subset_for_audit::<{ coefficients::ELP7_RECORDS }>(jd),
            ),
        ),
        (
            "ELP10",
            sum_planetary_terms(
                &Elp2000Evaluator::new().evaluate_full_elp10_longitude_family_for_audit(jd),
            ),
        ),
        (
            "ELP13",
            sum_planetary_terms(
                &Elp2000Evaluator::new().evaluate_full_elp13_longitude_family_for_audit(jd),
            ),
        ),
        (
            "ELP16",
            sum_planetary_terms(
                &Elp2000Evaluator::new().evaluate_full_elp16_longitude_family_for_audit(jd),
            ),
        ),
        (
            "ELP19",
            sum_planetary_terms(
                &Elp2000Evaluator::new().evaluate_full_elp19_longitude_family_for_audit(jd),
            ),
        ),
        (
            "ELP22",
            sum_short_periodic_terms(
                &Elp2000Evaluator::new()
                    .evaluate_elp22_longitude_subset_for_audit::<{ coefficients::ELP22_RECORDS }>(
                        jd,
                    ),
            ),
        ),
    ]
}

fn sum_main_problem_terms(terms: &[ElpMainProblemTermEvaluation]) -> f64 {
    terms
        .iter()
        .map(|evaluation| evaluation.contribution().value())
        .sum()
}

fn sum_short_periodic_terms(terms: &[ElpShortPeriodicTermEvaluation]) -> f64 {
    terms
        .iter()
        .map(|evaluation| evaluation.contribution().value())
        .sum()
}

fn sum_planetary_terms(terms: &[ElpPlanetaryTermEvaluation]) -> f64 {
    terms
        .iter()
        .map(|evaluation| evaluation.contribution().value())
        .sum()
}

fn evaluate_main_problem_longitude_term(
    term: coefficients::ElpMainProblemTerm,
    t: f64,
) -> ElpMainProblemTermEvaluation {
    let argument = main_problem_argument(term.multipliers, t);
    let corrected_amplitude = corrected_main_problem_longitude_amplitude(term);
    let contribution = ArcSeconds::new(corrected_amplitude.value() * argument.value().sin());

    ElpMainProblemTermEvaluation::new(argument, corrected_amplitude, contribution)
}

fn main_problem_argument(multipliers: [i16; 4], t: f64) -> Radians {
    let arguments = delaunay_arguments(t);
    let argument = multipliers
        .iter()
        .zip(arguments)
        .map(|(multiplier, argument)| f64::from(*multiplier) * argument)
        .sum();

    Radians::new(argument)
}

fn short_periodic_argument(
    term: coefficients::ElpShortPeriodicTerm,
    phase: Radians,
    zeta: Radians,
    t: f64,
) -> Radians {
    let arguments = short_periodic_delaunay_arguments(t);
    let argument = phase.value()
        + f64::from(term.zeta_multiplier) * zeta.value()
        + term
            .multipliers
            .iter()
            .zip(arguments)
            .map(|(multiplier, argument)| f64::from(*multiplier) * argument)
            .sum::<f64>();

    Radians::new(argument)
}

fn effective_short_periodic_amplitude(
    term: coefficients::ElpShortPeriodicTerm,
    definition: ElpFamilyDefinition,
    t: f64,
) -> f64 {
    term.amplitude * temporal_multiplier(definition.temporal_order(), t)
}

fn planetary_table_one_argument(
    term: coefficients::ElpPlanetaryTerm,
    phase: Radians,
    t: f64,
) -> Radians {
    let delaunay = short_periodic_delaunay_arguments(t);
    let planets = planetary_arguments(t);
    let multipliers = term.multipliers;
    let argument = phase.value()
        + f64::from(multipliers[8]) * delaunay[0]
        + f64::from(multipliers[9]) * delaunay[2]
        + f64::from(multipliers[10]) * delaunay[3]
        + multipliers[..8]
            .iter()
            .zip(planets)
            .map(|(multiplier, argument)| f64::from(*multiplier) * argument)
            .sum::<f64>();

    Radians::new(argument)
}

fn planetary_table_two_argument(
    term: coefficients::ElpPlanetaryTerm,
    phase: Radians,
    t: f64,
) -> Radians {
    let delaunay = short_periodic_delaunay_arguments(t);
    let planets = planetary_arguments(t);
    let multipliers = term.multipliers;
    let argument = phase.value()
        + multipliers[7..]
            .iter()
            .zip(delaunay)
            .map(|(multiplier, argument)| f64::from(*multiplier) * argument)
            .sum::<f64>()
        + multipliers[..7]
            .iter()
            .zip(planets)
            .map(|(multiplier, argument)| f64::from(*multiplier) * argument)
            .sum::<f64>();

    Radians::new(argument)
}

fn effective_planetary_amplitude(
    term: coefficients::ElpPlanetaryTerm,
    definition: ElpFamilyDefinition,
    t: f64,
) -> f64 {
    term.amplitude * temporal_multiplier(definition.temporal_order(), t)
}

fn temporal_multiplier(order: ElpTemporalOrder, t: f64) -> f64 {
    match order {
        ElpTemporalOrder::Order0 => 1.0,
        ElpTemporalOrder::Order1 => t,
        ElpTemporalOrder::Order2 => t * t,
    }
}

fn evaluate_zeta(t: f64) -> f64 {
    let w1 = lunar_argument_w1();

    w1[0] + (w1[1] + precession_constant()) * t
}

fn short_periodic_delaunay_arguments(t: f64) -> [f64; 4] {
    let w1 = lunar_argument_w1();
    let w2 = lunar_argument_w2();
    let w3 = lunar_argument_w3();
    let earth = earth_mean_longitude();
    let perihelion = earth_moon_barycenter_perihelion();

    [
        w1[0] - earth[0] + PI + (w1[1] - earth[1]) * t,
        earth[0] - perihelion[0] + (earth[1] - perihelion[1]) * t,
        w1[0] - w2[0] + (w1[1] - w2[1]) * t,
        w1[0] - w3[0] + (w1[1] - w3[1]) * t,
    ]
}

fn planetary_arguments(t: f64) -> [f64; 8] {
    let earth = earth_mean_longitude();

    [
        dms_to_radians(252.0, 15.0, 3.259_86) + 538_101_628.688_98 / ARCSECONDS_PER_RADIAN * t,
        dms_to_radians(181.0, 58.0, 47.283_05) + 210_664_136.433_55 / ARCSECONDS_PER_RADIAN * t,
        earth[0] + earth[1] * t,
        dms_to_radians(355.0, 25.0, 59.788_66) + 68_905_077.592_84 / ARCSECONDS_PER_RADIAN * t,
        dms_to_radians(34.0, 21.0, 5.342_12) + 10_925_660.428_61 / ARCSECONDS_PER_RADIAN * t,
        dms_to_radians(50.0, 4.0, 38.896_94) + 4_399_609.659_32 / ARCSECONDS_PER_RADIAN * t,
        dms_to_radians(314.0, 3.0, 18.018_41) + 1_542_481.193_93 / ARCSECONDS_PER_RADIAN * t,
        dms_to_radians(304.0, 20.0, 55.195_75) + 786_550.320_74 / ARCSECONDS_PER_RADIAN * t,
    ]
}

fn corrected_main_problem_longitude_amplitude(
    term: coefficients::ElpMainProblemTerm,
) -> ArcSeconds {
    let [coef2, coef3, coef4, coef5, coef6, _coef7] = term.corrections;
    let tgv = coef2 + DTASM * coef6;
    let constants = elp_correction_constants();

    ArcSeconds::new(
        term.amplitude
            + tgv * (constants.delnp - AM * constants.delnu)
            + coef3 * constants.delg
            + coef4 * constants.dele
            + coef5 * constants.delep,
    )
}

#[derive(Debug, Clone, Copy)]
struct ElpCorrectionConstants {
    delnu: f64,
    dele: f64,
    delg: f64,
    delnp: f64,
    delep: f64,
}

fn elp_correction_constants() -> ElpCorrectionConstants {
    let w1 = lunar_argument_w1();

    ElpCorrectionConstants {
        delnu: 0.556_04 / ARCSECONDS_PER_RADIAN / w1[1],
        dele: 0.017_89 / ARCSECONDS_PER_RADIAN,
        delg: -0.080_66 / ARCSECONDS_PER_RADIAN,
        delnp: -0.064_24 / ARCSECONDS_PER_RADIAN / w1[1],
        delep: -0.128_79 / ARCSECONDS_PER_RADIAN,
    }
}

fn delaunay_arguments(t: f64) -> [f64; 4] {
    let w1 = evaluate_polynomial(lunar_argument_w1(), t);
    let w2 = evaluate_polynomial(lunar_argument_w2(), t);
    let w3 = evaluate_polynomial(lunar_argument_w3(), t);
    let earth = evaluate_polynomial(earth_mean_longitude(), t);
    let perihelion = evaluate_polynomial(earth_moon_barycenter_perihelion(), t);

    [w1 - earth + PI, earth - perihelion, w1 - w2, w1 - w3]
}

fn evaluate_polynomial(coefficients: [f64; 5], t: f64) -> f64 {
    let mut value = 0.0;
    let mut t_power = 1.0;

    for coefficient in coefficients {
        value += coefficient * t_power;
        t_power *= t;
    }

    value
}

fn lunar_argument_w1() -> [f64; 5] {
    [
        dms_to_radians(218.0, 18.0, 59.955_71),
        1_732_559_343.736_04 / ARCSECONDS_PER_RADIAN,
        -5.888_3 / ARCSECONDS_PER_RADIAN,
        0.660_4e-2 / ARCSECONDS_PER_RADIAN,
        -0.316_9e-4 / ARCSECONDS_PER_RADIAN,
    ]
}

fn lunar_argument_w2() -> [f64; 5] {
    [
        dms_to_radians(83.0, 21.0, 11.674_75),
        14_643_420.263_2 / ARCSECONDS_PER_RADIAN,
        -38.277_6 / ARCSECONDS_PER_RADIAN,
        -0.450_47e-1 / ARCSECONDS_PER_RADIAN,
        0.213_01e-3 / ARCSECONDS_PER_RADIAN,
    ]
}

fn lunar_argument_w3() -> [f64; 5] {
    [
        dms_to_radians(125.0, 2.0, 40.398_16),
        -6_967_919.362_2 / ARCSECONDS_PER_RADIAN,
        6.362_2 / ARCSECONDS_PER_RADIAN,
        0.762_5e-2 / ARCSECONDS_PER_RADIAN,
        -0.358_6e-4 / ARCSECONDS_PER_RADIAN,
    ]
}

fn earth_mean_longitude() -> [f64; 5] {
    [
        dms_to_radians(100.0, 27.0, 59.220_59),
        129_597_742.275_8 / ARCSECONDS_PER_RADIAN,
        -0.020_2 / ARCSECONDS_PER_RADIAN,
        0.9e-5 / ARCSECONDS_PER_RADIAN,
        0.15e-6 / ARCSECONDS_PER_RADIAN,
    ]
}

fn earth_moon_barycenter_perihelion() -> [f64; 5] {
    [
        dms_to_radians(102.0, 56.0, 14.427_53),
        1_161.228_3 / ARCSECONDS_PER_RADIAN,
        0.532_7 / ARCSECONDS_PER_RADIAN,
        -0.138e-3 / ARCSECONDS_PER_RADIAN,
        0.0,
    ]
}

fn precession_constant() -> f64 {
    5_029.096_6 / ARCSECONDS_PER_RADIAN
}

fn dms_to_radians(degrees: f64, minutes: f64, seconds: f64) -> f64 {
    (degrees + minutes / MINUTES_PER_DEGREE + seconds / SECONDS_PER_DEGREE) * DEG_TO_RADIAN
}

#[cfg(test)]
mod tests {
    use super::{
        coefficients, Elp2000Evaluator, ElpFamily, ElpMagnitude, ElpSemanticEvaluator,
        ElpTemporalOrder,
    };
    use crate::core::JulianDate;

    const TOLERANCE: f64 = 1.0e-12;
    const ORACLE_TOLERANCE: f64 = 1.0e-11;
    const FULL_AST_L002_ORACLE_TOLERANCE: f64 = 1.0e-7;
    const ELP1_J2000_ORACLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../04_Tests/Astronomical/ELP2000-ELP1-J2000-Contributions.csv"
    ));
    const ELP4_J2000_ORACLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../04_Tests/Astronomical/ELP2000-ELP4-J2000-Contributions.csv"
    ));
    const ELP7_T1_ORACLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../04_Tests/Astronomical/ELP2000-ELP7-T1-Contributions.csv"
    ));
    const ELP22_J2000_ORACLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../04_Tests/Astronomical/ELP2000-ELP22-J2000-Contributions.csv"
    ));
    const ELP25_T1_ORACLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../04_Tests/Astronomical/ELP2000-ELP25-T1-Contributions.csv"
    ));
    const ELP28_J2000_ORACLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../04_Tests/Astronomical/ELP2000-ELP28-J2000-Contributions.csv"
    ));
    const ELP31_J2000_ORACLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../04_Tests/Astronomical/ELP2000-ELP31-J2000-Contributions.csv"
    ));
    const ELP34_T1_ORACLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../04_Tests/Astronomical/ELP2000-ELP34-T1-Contributions.csv"
    ));
    const ELP10_J2000_ORACLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../04_Tests/Astronomical/ELP2000-ELP10-J2000-Contributions.csv"
    ));
    const ELP13_T1_ORACLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../04_Tests/Astronomical/ELP2000-ELP13-T1-Contributions.csv"
    ));
    const ELP16_J2000_ORACLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../04_Tests/Astronomical/ELP2000-ELP16-J2000-Contributions.csv"
    ));
    const ELP19_T1_ORACLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../04_Tests/Astronomical/ELP2000-ELP19-T1-Contributions.csv"
    ));
    const AST_L002_PARTIAL_T1_ORACLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../04_Tests/Astronomical/ELP2000-AST-L002-Partial-T1.csv"
    ));
    const AST_L002_FULL_T1_ORACLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../04_Tests/Astronomical/ELP2000-AST-L002-Full-T1.csv"
    ));

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct ValidationMetrics {
        max_abs_error: f64,
        mean_abs_error: f64,
        rms_error: f64,
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct OracleRow {
        index: usize,
        contribution: f64,
        partial_sum: f64,
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct ShortPeriodicOracleRow {
        index: usize,
        argument: f64,
        xx: f64,
        contribution: f64,
        partial_sum: f64,
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct PlanetaryOracleRow {
        index: usize,
        argument: f64,
        amplitude: f64,
        contribution: f64,
        partial_sum: f64,
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct PartialLongitudeOracleRow {
        family: &'static str,
        contribution: f64,
        partial_sum: f64,
    }

    fn assert_close(actual: f64, expected: f64) {
        let difference = (actual - expected).abs();

        assert!(
            difference <= TOLERANCE,
            "expected {actual} to be within {TOLERANCE} of {expected}; difference {difference}",
        );
    }

    fn assert_close_with_tolerance(actual: f64, expected: f64, tolerance: f64) {
        let difference = (actual - expected).abs();

        assert!(
            difference <= tolerance,
            "expected {actual} to be within {tolerance} of {expected}; difference {difference}",
        );
    }

    fn parse_oracle_rows() -> Vec<OracleRow> {
        ELP1_J2000_ORACLE
            .lines()
            .skip(1)
            .map(|line| {
                let mut columns = line.split(',');
                let index = columns
                    .next()
                    .expect("missing oracle index")
                    .parse()
                    .expect("invalid oracle index");
                let contribution = columns
                    .next()
                    .expect("missing oracle contribution")
                    .parse()
                    .expect("invalid oracle contribution");
                let partial_sum = columns
                    .next()
                    .expect("missing oracle partial sum")
                    .parse()
                    .expect("invalid oracle partial sum");

                assert!(
                    columns.next().is_none(),
                    "unexpected extra column in oracle row {index}"
                );

                OracleRow {
                    index,
                    contribution,
                    partial_sum,
                }
            })
            .collect()
    }

    fn parse_short_periodic_oracle_rows() -> Vec<ShortPeriodicOracleRow> {
        parse_short_periodic_oracle_rows_from(ELP4_J2000_ORACLE)
    }

    fn parse_short_periodic_order1_oracle_rows() -> Vec<ShortPeriodicOracleRow> {
        parse_short_periodic_oracle_rows_from(ELP7_T1_ORACLE)
    }

    fn parse_short_periodic_elp22_oracle_rows() -> Vec<ShortPeriodicOracleRow> {
        parse_short_periodic_oracle_rows_from(ELP22_J2000_ORACLE)
    }

    fn parse_short_periodic_elp25_oracle_rows() -> Vec<ShortPeriodicOracleRow> {
        parse_short_periodic_oracle_rows_from(ELP25_T1_ORACLE)
    }

    fn parse_short_periodic_elp28_oracle_rows() -> Vec<ShortPeriodicOracleRow> {
        parse_short_periodic_oracle_rows_from(ELP28_J2000_ORACLE)
    }

    fn parse_short_periodic_elp31_oracle_rows() -> Vec<ShortPeriodicOracleRow> {
        parse_short_periodic_oracle_rows_from(ELP31_J2000_ORACLE)
    }

    fn parse_short_periodic_elp34_oracle_rows() -> Vec<ShortPeriodicOracleRow> {
        parse_short_periodic_oracle_rows_from(ELP34_T1_ORACLE)
    }

    fn parse_planetary_oracle_rows() -> Vec<PlanetaryOracleRow> {
        ELP10_J2000_ORACLE
            .lines()
            .skip(1)
            .map(parse_planetary_oracle_row)
            .collect()
    }

    fn parse_planetary_order1_oracle_rows() -> Vec<PlanetaryOracleRow> {
        ELP13_T1_ORACLE
            .lines()
            .skip(1)
            .map(parse_planetary_oracle_row)
            .collect()
    }

    fn parse_planetary_table_two_oracle_rows() -> Vec<PlanetaryOracleRow> {
        ELP16_J2000_ORACLE
            .lines()
            .skip(1)
            .map(parse_planetary_oracle_row)
            .collect()
    }

    fn parse_planetary_table_two_order1_oracle_rows() -> Vec<PlanetaryOracleRow> {
        ELP19_T1_ORACLE
            .lines()
            .skip(1)
            .map(parse_planetary_oracle_row)
            .collect()
    }

    fn parse_partial_longitude_oracle_rows() -> Vec<PartialLongitudeOracleRow> {
        parse_longitude_oracle_rows_from(AST_L002_PARTIAL_T1_ORACLE)
    }

    fn parse_full_longitude_oracle_rows() -> Vec<PartialLongitudeOracleRow> {
        parse_longitude_oracle_rows_from(AST_L002_FULL_T1_ORACLE)
    }

    fn parse_longitude_oracle_rows_from(oracle: &'static str) -> Vec<PartialLongitudeOracleRow> {
        oracle
            .lines()
            .skip(1)
            .map(|line| {
                let mut columns = line.split(',');
                let family = columns.next().expect("missing family");
                let contribution = columns
                    .next()
                    .expect("missing contribution")
                    .parse()
                    .expect("invalid contribution");
                let partial_sum = columns
                    .next()
                    .expect("missing partial sum")
                    .parse()
                    .expect("invalid partial sum");

                assert!(
                    columns.next().is_none(),
                    "unexpected extra column in partial longitude oracle row {family}"
                );

                PartialLongitudeOracleRow {
                    family,
                    contribution,
                    partial_sum,
                }
            })
            .collect()
    }

    fn parse_planetary_oracle_row(line: &str) -> PlanetaryOracleRow {
        let mut columns = line.split(',');
        let index = columns
            .next()
            .expect("missing oracle index")
            .parse()
            .expect("invalid oracle index");
        let argument = columns
            .next()
            .expect("missing oracle argument")
            .parse()
            .expect("invalid oracle argument");
        let amplitude = columns
            .next()
            .expect("missing oracle amplitude")
            .parse()
            .expect("invalid oracle amplitude");
        let contribution = columns
            .next()
            .expect("missing oracle contribution")
            .parse()
            .expect("invalid oracle contribution");
        let partial_sum = columns
            .next()
            .expect("missing oracle partial sum")
            .parse()
            .expect("invalid oracle partial sum");

        assert!(
            columns.next().is_none(),
            "unexpected extra column in oracle row {index}"
        );

        PlanetaryOracleRow {
            index,
            argument,
            amplitude,
            contribution,
            partial_sum,
        }
    }

    fn parse_short_periodic_oracle_rows_from(oracle: &str) -> Vec<ShortPeriodicOracleRow> {
        oracle
            .lines()
            .skip(1)
            .map(|line| {
                let mut columns = line.split(',');
                let index = columns
                    .next()
                    .expect("missing oracle index")
                    .parse()
                    .expect("invalid oracle index");
                let argument = columns
                    .next()
                    .expect("missing oracle argument")
                    .parse()
                    .expect("invalid oracle argument");
                let xx = columns
                    .next()
                    .expect("missing oracle xx")
                    .parse()
                    .expect("invalid oracle xx");
                let contribution = columns
                    .next()
                    .expect("missing oracle contribution")
                    .parse()
                    .expect("invalid oracle contribution");
                let partial_sum = columns
                    .next()
                    .expect("missing oracle partial sum")
                    .parse()
                    .expect("invalid oracle partial sum");

                assert!(
                    columns.next().is_none(),
                    "unexpected extra column in oracle row {index}"
                );

                ShortPeriodicOracleRow {
                    index,
                    argument,
                    xx,
                    contribution,
                    partial_sum,
                }
            })
            .collect()
    }

    fn validation_metrics(
        evaluations: &[super::ElpMainProblemTermEvaluation],
        oracle: &[OracleRow],
    ) -> ValidationMetrics {
        let mut max_abs_error: f64 = 0.0;
        let mut sum_abs_error = 0.0;
        let mut sum_squared_error = 0.0;

        for (evaluation, row) in evaluations.iter().zip(oracle) {
            let error = evaluation.contribution().value() - row.contribution;
            let abs_error = error.abs();

            max_abs_error = max_abs_error.max(abs_error);
            sum_abs_error += abs_error;
            sum_squared_error += error * error;
        }

        let count = evaluations.len() as f64;

        ValidationMetrics {
            max_abs_error,
            mean_abs_error: sum_abs_error / count,
            rms_error: (sum_squared_error / count).sqrt(),
        }
    }

    fn short_periodic_validation_metrics(
        evaluations: &[super::ElpShortPeriodicTermEvaluation],
        oracle: &[ShortPeriodicOracleRow],
    ) -> ValidationMetrics {
        let mut max_abs_error: f64 = 0.0;
        let mut sum_abs_error = 0.0;
        let mut sum_squared_error = 0.0;

        for (evaluation, row) in evaluations.iter().zip(oracle) {
            let error = evaluation.contribution().value() - row.contribution;
            let abs_error = error.abs();

            max_abs_error = max_abs_error.max(abs_error);
            sum_abs_error += abs_error;
            sum_squared_error += error * error;
        }

        let count = evaluations.len() as f64;

        ValidationMetrics {
            max_abs_error,
            mean_abs_error: sum_abs_error / count,
            rms_error: (sum_squared_error / count).sqrt(),
        }
    }

    fn planetary_validation_metrics(
        evaluations: &[super::ElpPlanetaryTermEvaluation],
        oracle: &[PlanetaryOracleRow],
    ) -> ValidationMetrics {
        let mut max_abs_error: f64 = 0.0;
        let mut sum_abs_error = 0.0;
        let mut sum_squared_error = 0.0;

        for (evaluation, row) in evaluations.iter().zip(oracle) {
            let error = evaluation.contribution().value() - row.contribution;
            let abs_error = error.abs();

            max_abs_error = max_abs_error.max(abs_error);
            sum_abs_error += abs_error;
            sum_squared_error += error * error;
        }

        let count = evaluations.len() as f64;

        ValidationMetrics {
            max_abs_error,
            mean_abs_error: sum_abs_error / count,
            rms_error: (sum_squared_error / count).sqrt(),
        }
    }

    #[test]
    fn creates_elp2000_evaluator() {
        let evaluator = Elp2000Evaluator::new();

        assert_eq!(format!("{evaluator:?}"), "Elp2000Evaluator");
    }

    #[test]
    fn inspects_elp1_family_without_evaluating_terms() {
        let family = Elp2000Evaluator::new().inspect_family(ElpFamily::Elp1);

        assert_eq!(family.name(), "ELP1");
        assert_eq!(family.term_count(), 1023);
        assert_eq!(family.term_count(), coefficients::ELP1_RECORDS);
        assert_eq!(family.fingerprint(), 0xa460_f0ac_886b_f342);
    }

    #[test]
    fn maps_longitude_families_to_semantic_evaluators() {
        let expected = [
            (
                ElpFamily::Elp1,
                "ELP1",
                1023,
                ElpSemanticEvaluator::MainProblem,
                ElpTemporalOrder::Order0,
            ),
            (
                ElpFamily::Elp4,
                "ELP4",
                347,
                ElpSemanticEvaluator::ShortPeriodic,
                ElpTemporalOrder::Order0,
            ),
            (
                ElpFamily::Elp7,
                "ELP7",
                14,
                ElpSemanticEvaluator::ShortPeriodic,
                ElpTemporalOrder::Order1,
            ),
            (
                ElpFamily::Elp10,
                "ELP10",
                14_328,
                ElpSemanticEvaluator::Planetary,
                ElpTemporalOrder::Order0,
            ),
            (
                ElpFamily::Elp13,
                "ELP13",
                4_384,
                ElpSemanticEvaluator::Planetary,
                ElpTemporalOrder::Order1,
            ),
            (
                ElpFamily::Elp16,
                "ELP16",
                170,
                ElpSemanticEvaluator::Planetary,
                ElpTemporalOrder::Order0,
            ),
            (
                ElpFamily::Elp19,
                "ELP19",
                226,
                ElpSemanticEvaluator::Planetary,
                ElpTemporalOrder::Order1,
            ),
            (
                ElpFamily::Elp22,
                "ELP22",
                3,
                ElpSemanticEvaluator::ShortPeriodic,
                ElpTemporalOrder::Order0,
            ),
            (
                ElpFamily::Elp25,
                "ELP25",
                6,
                ElpSemanticEvaluator::ShortPeriodic,
                ElpTemporalOrder::Order1,
            ),
            (
                ElpFamily::Elp28,
                "ELP28",
                20,
                ElpSemanticEvaluator::ShortPeriodic,
                ElpTemporalOrder::Order0,
            ),
            (
                ElpFamily::Elp31,
                "ELP31",
                11,
                ElpSemanticEvaluator::ShortPeriodic,
                ElpTemporalOrder::Order0,
            ),
            (
                ElpFamily::Elp34,
                "ELP34",
                28,
                ElpSemanticEvaluator::ShortPeriodic,
                ElpTemporalOrder::Order2,
            ),
        ];

        for (family, name, term_count, evaluator, temporal_order) in expected {
            let definition = Elp2000Evaluator::new().family_definition(family);

            assert_eq!(definition.family(), family);
            assert_eq!(definition.info().name(), name);
            assert_eq!(definition.info().term_count(), term_count);
            assert_eq!(definition.magnitude(), ElpMagnitude::Longitude);
            assert_eq!(definition.semantic_evaluator(), evaluator);
            assert_eq!(definition.temporal_order(), temporal_order);
        }
    }

    #[test]
    fn exposes_temporal_order_exponents_explicitly() {
        assert_eq!(ElpTemporalOrder::Order0.exponent(), 0);
        assert_eq!(ElpTemporalOrder::Order1.exponent(), 1);
        assert_eq!(ElpTemporalOrder::Order2.exponent(), 2);
    }

    #[test]
    fn maps_elp4_to_short_periodic_without_evaluating_terms() {
        let definition = Elp2000Evaluator::new().family_definition(ElpFamily::Elp4);

        assert_eq!(definition.info().name(), "ELP4");
        assert_eq!(definition.info().term_count(), coefficients::ELP4_RECORDS);
        assert_eq!(definition.info().fingerprint(), 0xd2fd_a15c_8974_356c);
        assert_eq!(
            definition.semantic_evaluator(),
            ElpSemanticEvaluator::ShortPeriodic,
        );
        assert_eq!(definition.temporal_order(), ElpTemporalOrder::Order0);
        assert_eq!(definition.temporal_order().exponent(), 0);
    }

    #[test]
    fn maps_elp7_to_short_periodic_with_first_temporal_order() {
        let definition = Elp2000Evaluator::new().family_definition(ElpFamily::Elp7);

        assert_eq!(definition.info().name(), "ELP7");
        assert_eq!(definition.info().term_count(), coefficients::ELP7_RECORDS);
        assert_eq!(definition.info().fingerprint(), 0x5888_fc21_d23e_a7e3);
        assert_eq!(
            definition.semantic_evaluator(),
            ElpSemanticEvaluator::ShortPeriodic,
        );
        assert_eq!(definition.temporal_order(), ElpTemporalOrder::Order1);
        assert_eq!(definition.temporal_order().exponent(), 1);
    }

    #[test]
    fn first_elp1_term_matches_official_source_record() {
        let term = coefficients::ELP1[0];

        assert_eq!(term.multipliers, [0, 0, 0, 2]);
        assert_eq!(term.amplitude, -411.602_87);
        assert_eq!(
            term.corrections,
            [168.48, -18_433.81, -121.62, 0.40, -0.18, 0.00],
        );
    }

    #[test]
    fn first_elp4_term_matches_official_source_record() {
        let term = coefficients::ELP4[0];

        assert_eq!(term.zeta_multiplier, 0);
        assert_eq!(term.multipliers, [0, 0, 0, 1]);
        assert_eq!(term.phase_degrees, 270.000_00);
        assert_eq!(term.amplitude, 0.000_03);
    }

    #[test]
    fn first_elp7_term_matches_official_source_record() {
        let term = coefficients::ELP7[0];

        assert_eq!(term.zeta_multiplier, 1);
        assert_eq!(term.multipliers, [-2, 0, 0, -1]);
        assert_eq!(term.phase_degrees, 180.000_00);
        assert_eq!(term.amplitude, 0.000_03);
    }

    #[test]
    fn first_elp22_term_matches_official_source_record() {
        let term = coefficients::ELP22[0];

        assert_eq!(term.zeta_multiplier, 0);
        assert_eq!(term.multipliers, [1, 1, -1, -1]);
        assert_eq!(term.phase_degrees, 192.936_65);
        assert_eq!(term.amplitude, 0.000_04);
    }

    #[test]
    fn first_elp10_term_matches_official_source_record() {
        let term = coefficients::ELP10[0];

        assert_eq!(term.multipliers, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);
        assert_eq!(term.phase_degrees, 359.998_31);
        assert_eq!(term.amplitude, 0.000_20);
    }

    #[test]
    fn first_elp13_term_matches_official_source_record() {
        let term = coefficients::ELP13[0];

        assert_eq!(term.multipliers, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(term.phase_degrees, 270.000_00);
        assert_eq!(term.amplitude, 0.000_11);
    }

    #[test]
    fn audits_elp1_family_through_general_api() {
        let audit =
            Elp2000Evaluator::new().audit_family(ElpFamily::Elp1, JulianDate::new(2_451_545.0));
        let terms = audit
            .main_problem_terms()
            .expect("ELP1 audit must produce main-problem terms");

        assert_eq!(audit.info().name(), "ELP1");
        assert_eq!(audit.term_count(), coefficients::ELP1_RECORDS);
        assert_eq!(terms.len(), coefficients::ELP1_RECORDS);
    }

    #[test]
    fn first_elp1_term_builds_expected_argument_at_j2000() {
        let evaluation = Elp2000Evaluator::new()
            .evaluate_first_elp1_longitude_term_for_audit(JulianDate::new(2_451_545.0));

        assert_close(evaluation.argument().value(), 3.255_810_466_742_936);
    }

    #[test]
    fn first_elp1_term_applies_main_problem_amplitude_corrections() {
        let evaluation = Elp2000Evaluator::new()
            .evaluate_first_elp1_longitude_term_for_audit(JulianDate::new(2_451_545.0));

        assert_close(
            evaluation.corrected_amplitude().value(),
            -411.595_672_253_951_7,
        );
    }

    #[test]
    fn first_elp1_term_produces_expected_individual_contribution() {
        let evaluation = Elp2000Evaluator::new()
            .evaluate_first_elp1_longitude_term_for_audit(JulianDate::new(2_451_545.0));

        assert_close(evaluation.contribution().value(), 46.909_407_726_369_665);
    }

    #[test]
    fn first_elp4_term_exposes_short_periodic_audit_fields() {
        let evaluation = Elp2000Evaluator::new()
            .evaluate_first_elp4_longitude_term_for_audit(JulianDate::new(2_451_545.0));

        assert_close(evaluation.phase().value(), 4.712_388_980_384_69);
        assert_close(evaluation.zeta().value(), 3.810_344_430_588_308);
        assert_eq!(evaluation.iz(), 0);
        assert_close(evaluation.argument().value(), 6.340_294_213_756_158);
        assert_close(evaluation.xx().value(), 0.000_03);
        assert_close(evaluation.contribution().value(), 0.000_001_712_336_066_442);
    }

    #[test]
    fn first_elp4_term_matches_short_periodic_oracle() {
        let oracle = parse_short_periodic_oracle_rows();
        let row = oracle
            .first()
            .expect("ELP4 short-periodic oracle must have one row");
        let evaluation = Elp2000Evaluator::new()
            .evaluate_first_elp4_longitude_term_for_audit(JulianDate::new(2_451_545.0));

        assert_eq!(row.index, 1);
        assert_close_with_tolerance(evaluation.argument().value(), row.argument, 1.0e-15);
        assert_close_with_tolerance(evaluation.xx().value(), row.xx, 1.0e-18);
        assert_close_with_tolerance(evaluation.contribution().value(), row.contribution, 1.0e-18);
        assert_close_with_tolerance(row.partial_sum, row.contribution, 1.0e-18);
    }

    #[test]
    fn first_elp7_term_applies_first_temporal_order_to_amplitude() {
        let j2000 = Elp2000Evaluator::new()
            .evaluate_first_elp7_longitude_term_for_audit(JulianDate::new(2_451_545.0));
        let one_century_after_j2000 = Elp2000Evaluator::new()
            .evaluate_first_elp7_longitude_term_for_audit(JulianDate::new(2_488_070.0));

        assert_close_with_tolerance(j2000.xx().value(), 0.0, 1.0e-18);
        assert_close_with_tolerance(j2000.contribution().value(), 0.0, 1.0e-18);
        assert_close_with_tolerance(one_century_after_j2000.xx().value(), 0.000_03, 1.0e-18);
    }

    #[test]
    fn first_elp7_term_matches_short_periodic_order1_oracle() {
        let oracle = parse_short_periodic_order1_oracle_rows();
        let row = oracle
            .first()
            .expect("ELP7 short-periodic order-one oracle must have one row");
        let evaluation = Elp2000Evaluator::new()
            .evaluate_first_elp7_longitude_term_for_audit(JulianDate::new(2_488_070.0));

        assert_eq!(row.index, 1);
        assert_close_with_tolerance(evaluation.argument().value(), row.argument, 1.0e-12);
        assert_close_with_tolerance(evaluation.xx().value(), row.xx, 1.0e-18);
        assert_close_with_tolerance(evaluation.contribution().value(), row.contribution, 1.0e-18);
        assert_close_with_tolerance(row.partial_sum, row.contribution, 1.0e-18);
    }

    #[test]
    fn first_elp22_term_uses_short_periodic_order_zero() {
        let evaluation = Elp2000Evaluator::new()
            .evaluate_first_elp22_longitude_term_for_audit(JulianDate::new(2_451_545.0));

        assert_close_with_tolerance(evaluation.xx().value(), 0.000_04, 1.0e-18);
    }

    #[test]
    fn first_elp22_term_matches_short_periodic_oracle() {
        let oracle = parse_short_periodic_elp22_oracle_rows();
        let row = oracle
            .first()
            .expect("ELP22 short-periodic oracle must have one row");
        let evaluation = Elp2000Evaluator::new()
            .evaluate_first_elp22_longitude_term_for_audit(JulianDate::new(2_451_545.0));

        assert_eq!(row.index, 1);
        assert_close_with_tolerance(evaluation.argument().value(), row.argument, 1.0e-15);
        assert_close_with_tolerance(evaluation.xx().value(), row.xx, 1.0e-18);
        assert_close_with_tolerance(evaluation.contribution().value(), row.contribution, 1.0e-18);
        assert_close_with_tolerance(row.partial_sum, row.contribution, 1.0e-18);
    }

    #[test]
    fn first_elp10_term_exposes_planetary_audit_fields() {
        let evaluation = Elp2000Evaluator::new()
            .evaluate_first_elp10_longitude_term_for_audit(JulianDate::new(2_451_545.0));

        assert_close(evaluation.phase().value(), 6.283_155_811_115_228);
        assert_close(evaluation.argument().value(), 9.538_966_277_858_165);
        assert_close_with_tolerance(evaluation.amplitude().value(), 0.000_20, 1.0e-18);
        assert_close_with_tolerance(
            evaluation.contribution().value(),
            -0.000_022_788_065_822_491,
            1.0e-18,
        );
    }

    #[test]
    fn first_elp10_term_matches_planetary_oracle() {
        let oracle = parse_planetary_oracle_rows();
        let row = oracle
            .first()
            .expect("ELP10 planetary oracle must have one row");
        let evaluation = Elp2000Evaluator::new()
            .evaluate_first_elp10_longitude_term_for_audit(JulianDate::new(2_451_545.0));

        assert_eq!(row.index, 1);
        assert_close_with_tolerance(evaluation.argument().value(), row.argument, 1.0e-15);
        assert_close_with_tolerance(evaluation.amplitude().value(), row.amplitude, 1.0e-18);
        assert_close_with_tolerance(evaluation.contribution().value(), row.contribution, 1.0e-18);
        assert_close_with_tolerance(row.partial_sum, row.contribution, 1.0e-18);
    }

    #[test]
    fn first_elp13_term_applies_first_temporal_order_to_amplitude() {
        let j2000 = Elp2000Evaluator::new()
            .evaluate_first_elp13_longitude_term_for_audit(JulianDate::new(2_451_545.0));
        let one_century_after_j2000 = Elp2000Evaluator::new()
            .evaluate_first_elp13_longitude_term_for_audit(JulianDate::new(2_488_070.0));

        assert_close_with_tolerance(j2000.amplitude().value(), 0.0, 1.0e-18);
        assert_close_with_tolerance(j2000.contribution().value(), 0.0, 1.0e-18);
        assert_close_with_tolerance(
            one_century_after_j2000.amplitude().value(),
            0.000_11,
            1.0e-18,
        );
    }

    #[test]
    fn first_elp13_term_matches_planetary_order1_oracle() {
        let oracle = parse_planetary_order1_oracle_rows();
        let row = oracle
            .first()
            .expect("ELP13 planetary order-one oracle must have one row");
        let evaluation = Elp2000Evaluator::new()
            .evaluate_first_elp13_longitude_term_for_audit(JulianDate::new(2_488_070.0));

        assert_eq!(row.index, 1);
        assert_close_with_tolerance(evaluation.argument().value(), row.argument, 1.0e-15);
        assert_close_with_tolerance(evaluation.amplitude().value(), row.amplitude, 1.0e-18);
        assert_close_with_tolerance(evaluation.contribution().value(), row.contribution, 1.0e-18);
        assert_close_with_tolerance(row.partial_sum, row.contribution, 1.0e-18);
    }

    #[test]
    fn complete_elp13_family_matches_planetary_order1_oracle_contributions() {
        let oracle = parse_planetary_order1_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp13_longitude_family_for_audit(JulianDate::new(2_488_070.0));

        assert_eq!(oracle.len(), coefficients::ELP13_RECORDS);
        assert_eq!(evaluations.len(), coefficients::ELP13_RECORDS);

        for (term_index, (evaluation, row)) in evaluations.iter().zip(&oracle).enumerate() {
            assert_eq!(row.index, term_index + 1);
            assert_close_with_tolerance(
                evaluation.contribution().value(),
                row.contribution,
                ORACLE_TOLERANCE,
            );
        }
    }

    #[test]
    fn complete_elp13_family_matches_planetary_order1_oracle_final_sum() {
        let oracle = parse_planetary_order1_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp13_longitude_family_for_audit(JulianDate::new(2_488_070.0));
        let total: f64 = evaluations
            .iter()
            .map(|evaluation| evaluation.contribution().value())
            .sum();

        assert_close_with_tolerance(
            total,
            oracle
                .last()
                .expect("ELP13 oracle must include complete family")
                .partial_sum,
            ORACLE_TOLERANCE,
        );
    }

    #[test]
    fn complete_elp13_family_validation_metrics_are_stable() {
        let oracle = parse_planetary_order1_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp13_longitude_family_for_audit(JulianDate::new(2_488_070.0));
        let metrics = planetary_validation_metrics(&evaluations, &oracle);

        assert_close_with_tolerance(metrics.max_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.mean_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.rms_error, 0.0, ORACLE_TOLERANCE);
    }

    #[test]
    fn complete_elp16_family_matches_planetary_table_two_oracle_contributions() {
        let oracle = parse_planetary_table_two_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp16_longitude_family_for_audit(JulianDate::new(2_451_545.0));

        assert_eq!(oracle.len(), coefficients::ELP16_RECORDS);
        assert_eq!(evaluations.len(), coefficients::ELP16_RECORDS);

        for (term_index, (evaluation, row)) in evaluations.iter().zip(&oracle).enumerate() {
            assert_eq!(row.index, term_index + 1);
            assert_close_with_tolerance(
                evaluation.contribution().value(),
                row.contribution,
                ORACLE_TOLERANCE,
            );
        }
    }

    #[test]
    fn complete_elp16_family_matches_planetary_table_two_oracle_final_sum() {
        let oracle = parse_planetary_table_two_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp16_longitude_family_for_audit(JulianDate::new(2_451_545.0));
        let total: f64 = evaluations
            .iter()
            .map(|evaluation| evaluation.contribution().value())
            .sum();

        assert_close_with_tolerance(
            total,
            oracle
                .last()
                .expect("ELP16 oracle must include complete family")
                .partial_sum,
            ORACLE_TOLERANCE,
        );
    }

    #[test]
    fn complete_elp16_family_validation_metrics_are_stable() {
        let oracle = parse_planetary_table_two_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp16_longitude_family_for_audit(JulianDate::new(2_451_545.0));
        let metrics = planetary_validation_metrics(&evaluations, &oracle);

        assert_close_with_tolerance(metrics.max_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.mean_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.rms_error, 0.0, ORACLE_TOLERANCE);
    }

    #[test]
    fn complete_elp19_family_matches_planetary_table_two_order1_oracle_contributions() {
        let oracle = parse_planetary_table_two_order1_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp19_longitude_family_for_audit(JulianDate::new(2_488_070.0));

        assert_eq!(oracle.len(), coefficients::ELP19_RECORDS);
        assert_eq!(evaluations.len(), coefficients::ELP19_RECORDS);

        for (term_index, (evaluation, row)) in evaluations.iter().zip(&oracle).enumerate() {
            assert_eq!(row.index, term_index + 1);
            assert_close_with_tolerance(
                evaluation.contribution().value(),
                row.contribution,
                ORACLE_TOLERANCE,
            );
        }
    }

    #[test]
    fn complete_elp19_family_matches_planetary_table_two_order1_oracle_final_sum() {
        let oracle = parse_planetary_table_two_order1_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp19_longitude_family_for_audit(JulianDate::new(2_488_070.0));
        let total: f64 = evaluations
            .iter()
            .map(|evaluation| evaluation.contribution().value())
            .sum();

        assert_close_with_tolerance(
            total,
            oracle
                .last()
                .expect("ELP19 oracle must include complete family")
                .partial_sum,
            ORACLE_TOLERANCE,
        );
    }

    #[test]
    fn complete_elp19_family_validation_metrics_are_stable() {
        let oracle = parse_planetary_table_two_order1_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp19_longitude_family_for_audit(JulianDate::new(2_488_070.0));
        let metrics = planetary_validation_metrics(&evaluations, &oracle);

        assert_close_with_tolerance(metrics.max_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.mean_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.rms_error, 0.0, ORACLE_TOLERANCE);
    }

    #[test]
    fn partial_ast_l002_integration_matches_family_oracle_contributions() {
        let oracle = parse_partial_longitude_oracle_rows();
        let family_totals = super::validated_longitude_correction_partial_family_totals(
            JulianDate::new(2_488_070.0),
        );

        assert_eq!(oracle.len(), family_totals.len());

        for (row, (family, contribution)) in oracle.iter().zip(family_totals) {
            assert_eq!(row.family, family);
            assert_close_with_tolerance(
                contribution,
                row.contribution,
                FULL_AST_L002_ORACLE_TOLERANCE,
            );
        }
    }

    #[test]
    fn partial_ast_l002_integration_matches_oracle_final_sum() {
        let oracle = parse_partial_longitude_oracle_rows();
        let partial = Elp2000Evaluator::new()
            .evaluate_validated_longitude_correction_partial_for_audit(JulianDate::new(
                2_488_070.0,
            ));

        assert_close_with_tolerance(
            partial.value(),
            oracle
                .last()
                .expect("partial AST-L002 oracle must include final sum")
                .partial_sum,
            ORACLE_TOLERANCE,
        );
    }

    #[test]
    fn ast_l002_integration_matches_family_oracle_contributions() {
        let oracle = parse_full_longitude_oracle_rows();
        let family_totals = super::longitude_correction_family_totals(JulianDate::new(2_488_070.0));

        assert_eq!(oracle.len(), family_totals.len());

        for (row, (family, contribution)) in oracle.iter().zip(family_totals) {
            assert_eq!(row.family, family);
            assert_close_with_tolerance(
                contribution,
                row.contribution,
                FULL_AST_L002_ORACLE_TOLERANCE,
            );
        }
    }

    #[test]
    fn ast_l002_public_entry_point_matches_complete_oracle_final_sum() {
        let oracle = parse_full_longitude_oracle_rows();
        let correction =
            Elp2000Evaluator::new().evaluate_longitude_correction(JulianDate::new(2_488_070.0));

        assert_close_with_tolerance(
            correction.value(),
            oracle
                .last()
                .expect("full AST-L002 oracle must include final sum")
                .partial_sum,
            FULL_AST_L002_ORACLE_TOLERANCE,
        );
    }

    #[test]
    fn complete_elp10_family_matches_planetary_oracle_contributions() {
        let oracle = parse_planetary_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp10_longitude_family_for_audit(JulianDate::new(2_451_545.0));

        assert_eq!(oracle.len(), coefficients::ELP10_RECORDS);
        assert_eq!(evaluations.len(), coefficients::ELP10_RECORDS);

        for (term_index, (evaluation, row)) in evaluations.iter().zip(&oracle).enumerate() {
            assert_eq!(row.index, term_index + 1);
            assert_close_with_tolerance(
                evaluation.contribution().value(),
                row.contribution,
                ORACLE_TOLERANCE,
            );
        }
    }

    #[test]
    fn complete_elp10_family_matches_planetary_oracle_final_sum() {
        let oracle = parse_planetary_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp10_longitude_family_for_audit(JulianDate::new(2_451_545.0));
        let total: f64 = evaluations
            .iter()
            .map(|evaluation| evaluation.contribution().value())
            .sum();

        assert_close_with_tolerance(
            total,
            oracle
                .last()
                .expect("ELP10 oracle must include complete family")
                .partial_sum,
            ORACLE_TOLERANCE,
        );
    }

    #[test]
    fn complete_elp10_family_validation_metrics_are_stable() {
        let oracle = parse_planetary_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp10_longitude_family_for_audit(JulianDate::new(2_451_545.0));
        let metrics = planetary_validation_metrics(&evaluations, &oracle);

        assert_close_with_tolerance(metrics.max_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.mean_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.rms_error, 0.0, ORACLE_TOLERANCE);
    }

    #[test]
    fn complete_elp22_family_matches_short_periodic_oracle_contributions() {
        let oracle = parse_short_periodic_elp22_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_elp22_longitude_subset_for_audit::<{ coefficients::ELP22_RECORDS }>(
                JulianDate::new(2_451_545.0),
            );

        assert_eq!(oracle.len(), coefficients::ELP22_RECORDS);
        assert_eq!(evaluations.len(), coefficients::ELP22_RECORDS);

        for (term_index, (evaluation, row)) in evaluations.iter().zip(&oracle).enumerate() {
            assert_eq!(row.index, term_index + 1);
            assert_close_with_tolerance(evaluation.argument().value(), row.argument, 1.0e-15);
            assert_close_with_tolerance(evaluation.xx().value(), row.xx, 1.0e-18);
            assert_close_with_tolerance(
                evaluation.contribution().value(),
                row.contribution,
                1.0e-18,
            );
        }
    }

    #[test]
    fn complete_elp22_family_matches_short_periodic_oracle_final_sum() {
        let oracle = parse_short_periodic_elp22_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_elp22_longitude_subset_for_audit::<{ coefficients::ELP22_RECORDS }>(
                JulianDate::new(2_451_545.0),
            );
        let total: f64 = evaluations
            .iter()
            .map(|evaluation| evaluation.contribution().value())
            .sum();

        assert_close_with_tolerance(
            total,
            oracle
                .last()
                .expect("ELP22 oracle must include complete family")
                .partial_sum,
            1.0e-18,
        );
    }

    #[test]
    fn complete_elp25_family_matches_short_periodic_order1_oracle_contributions() {
        let oracle = parse_short_periodic_elp25_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp25_longitude_family_for_audit(JulianDate::new(2_488_070.0));

        assert_eq!(oracle.len(), coefficients::ELP25_RECORDS);
        assert_eq!(evaluations.len(), coefficients::ELP25_RECORDS);

        for (term_index, (evaluation, row)) in evaluations.iter().zip(&oracle).enumerate() {
            assert_eq!(row.index, term_index + 1);
            assert_close_with_tolerance(evaluation.argument().value(), row.argument, 1.0e-12);
            assert_close_with_tolerance(evaluation.xx().value(), row.xx, 1.0e-18);
            assert_close_with_tolerance(
                evaluation.contribution().value(),
                row.contribution,
                1.0e-18,
            );
        }
    }

    #[test]
    fn complete_elp25_family_matches_short_periodic_order1_oracle_final_sum() {
        let oracle = parse_short_periodic_elp25_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp25_longitude_family_for_audit(JulianDate::new(2_488_070.0));
        let total: f64 = evaluations
            .iter()
            .map(|evaluation| evaluation.contribution().value())
            .sum();

        assert_close_with_tolerance(
            total,
            oracle
                .last()
                .expect("ELP25 oracle must include complete family")
                .partial_sum,
            1.0e-18,
        );
    }

    #[test]
    fn complete_elp25_family_validation_metrics_are_stable() {
        let oracle = parse_short_periodic_elp25_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp25_longitude_family_for_audit(JulianDate::new(2_488_070.0));
        let metrics = short_periodic_validation_metrics(&evaluations, &oracle);

        assert_close_with_tolerance(metrics.max_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.mean_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.rms_error, 0.0, ORACLE_TOLERANCE);
    }

    #[test]
    fn complete_elp28_family_matches_short_periodic_oracle_contributions() {
        let oracle = parse_short_periodic_elp28_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp28_longitude_family_for_audit(JulianDate::new(2_451_545.0));

        assert_eq!(oracle.len(), coefficients::ELP28_RECORDS);
        assert_eq!(evaluations.len(), coefficients::ELP28_RECORDS);

        for (term_index, (evaluation, row)) in evaluations.iter().zip(&oracle).enumerate() {
            assert_eq!(row.index, term_index + 1);
            assert_close_with_tolerance(evaluation.argument().value(), row.argument, 1.0e-15);
            assert_close_with_tolerance(evaluation.xx().value(), row.xx, 1.0e-18);
            assert_close_with_tolerance(
                evaluation.contribution().value(),
                row.contribution,
                1.0e-18,
            );
        }
    }

    #[test]
    fn complete_elp28_family_matches_short_periodic_oracle_final_sum() {
        let oracle = parse_short_periodic_elp28_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp28_longitude_family_for_audit(JulianDate::new(2_451_545.0));
        let total: f64 = evaluations
            .iter()
            .map(|evaluation| evaluation.contribution().value())
            .sum();

        assert_close_with_tolerance(
            total,
            oracle
                .last()
                .expect("ELP28 oracle must include complete family")
                .partial_sum,
            1.0e-18,
        );
    }

    #[test]
    fn complete_elp28_family_validation_metrics_are_stable() {
        let oracle = parse_short_periodic_elp28_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp28_longitude_family_for_audit(JulianDate::new(2_451_545.0));
        let metrics = short_periodic_validation_metrics(&evaluations, &oracle);

        assert_close_with_tolerance(metrics.max_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.mean_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.rms_error, 0.0, ORACLE_TOLERANCE);
    }

    #[test]
    fn complete_elp31_family_matches_short_periodic_oracle_contributions() {
        let oracle = parse_short_periodic_elp31_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp31_longitude_family_for_audit(JulianDate::new(2_451_545.0));

        assert_eq!(oracle.len(), coefficients::ELP31_RECORDS);
        assert_eq!(evaluations.len(), coefficients::ELP31_RECORDS);

        for (term_index, (evaluation, row)) in evaluations.iter().zip(&oracle).enumerate() {
            assert_eq!(row.index, term_index + 1);
            assert_close_with_tolerance(evaluation.argument().value(), row.argument, 1.0e-15);
            assert_close_with_tolerance(evaluation.xx().value(), row.xx, 1.0e-18);
            assert_close_with_tolerance(
                evaluation.contribution().value(),
                row.contribution,
                1.0e-18,
            );
        }
    }

    #[test]
    fn complete_elp31_family_matches_short_periodic_oracle_final_sum() {
        let oracle = parse_short_periodic_elp31_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp31_longitude_family_for_audit(JulianDate::new(2_451_545.0));
        let total: f64 = evaluations
            .iter()
            .map(|evaluation| evaluation.contribution().value())
            .sum();

        assert_close_with_tolerance(
            total,
            oracle
                .last()
                .expect("ELP31 oracle must include complete family")
                .partial_sum,
            1.0e-18,
        );
    }

    #[test]
    fn complete_elp31_family_validation_metrics_are_stable() {
        let oracle = parse_short_periodic_elp31_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp31_longitude_family_for_audit(JulianDate::new(2_451_545.0));
        let metrics = short_periodic_validation_metrics(&evaluations, &oracle);

        assert_close_with_tolerance(metrics.max_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.mean_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.rms_error, 0.0, ORACLE_TOLERANCE);
    }

    #[test]
    fn complete_elp34_family_matches_short_periodic_order2_oracle_contributions() {
        let oracle = parse_short_periodic_elp34_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp34_longitude_family_for_audit(JulianDate::new(2_488_070.0));

        assert_eq!(oracle.len(), coefficients::ELP34_RECORDS);
        assert_eq!(evaluations.len(), coefficients::ELP34_RECORDS);

        for (term_index, (evaluation, row)) in evaluations.iter().zip(&oracle).enumerate() {
            assert_eq!(row.index, term_index + 1);
            assert_close_with_tolerance(evaluation.argument().value(), row.argument, 1.0e-12);
            assert_close_with_tolerance(evaluation.xx().value(), row.xx, 1.0e-18);
            assert_close_with_tolerance(
                evaluation.contribution().value(),
                row.contribution,
                1.0e-18,
            );
        }
    }

    #[test]
    fn complete_elp34_family_matches_short_periodic_order2_oracle_final_sum() {
        let oracle = parse_short_periodic_elp34_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp34_longitude_family_for_audit(JulianDate::new(2_488_070.0));
        let total: f64 = evaluations
            .iter()
            .map(|evaluation| evaluation.contribution().value())
            .sum();

        assert_close_with_tolerance(
            total,
            oracle
                .last()
                .expect("ELP34 oracle must include complete family")
                .partial_sum,
            1.0e-18,
        );
    }

    #[test]
    fn complete_elp34_family_validation_metrics_are_stable() {
        let oracle = parse_short_periodic_elp34_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_full_elp34_longitude_family_for_audit(JulianDate::new(2_488_070.0));
        let metrics = short_periodic_validation_metrics(&evaluations, &oracle);

        assert_close_with_tolerance(metrics.max_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.mean_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.rms_error, 0.0, ORACLE_TOLERANCE);
    }

    #[test]
    fn first_ten_elp7_terms_use_the_same_short_periodic_evaluator() {
        let first = Elp2000Evaluator::new()
            .evaluate_first_elp7_longitude_term_for_audit(JulianDate::new(2_488_070.0));
        let ten = Elp2000Evaluator::new()
            .evaluate_first_ten_elp7_longitude_terms_for_audit(JulianDate::new(2_488_070.0));

        assert_eq!(first, ten[0]);
    }

    #[test]
    fn first_ten_elp7_terms_match_short_periodic_order1_oracle_contributions() {
        let oracle = parse_short_periodic_order1_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_first_ten_elp7_longitude_terms_for_audit(JulianDate::new(2_488_070.0));

        assert!(oracle.len() >= 10);

        for (term_index, (evaluation, row)) in
            evaluations.iter().zip(oracle.iter().take(10)).enumerate()
        {
            assert_eq!(row.index, term_index + 1);
            assert_close_with_tolerance(evaluation.argument().value(), row.argument, 1.0e-12);
            assert_close_with_tolerance(evaluation.xx().value(), row.xx, 1.0e-18);
            assert_close_with_tolerance(
                evaluation.contribution().value(),
                row.contribution,
                1.0e-18,
            );
        }
    }

    #[test]
    fn first_ten_elp7_terms_match_short_periodic_order1_oracle_partial_sum() {
        let oracle = parse_short_periodic_order1_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_first_ten_elp7_longitude_terms_for_audit(JulianDate::new(2_488_070.0));
        let partial_sum: f64 = evaluations
            .iter()
            .map(|evaluation| evaluation.contribution().value())
            .sum();

        assert_close_with_tolerance(
            partial_sum,
            oracle
                .get(9)
                .expect("ELP7 oracle must include tenth row")
                .partial_sum,
            1.0e-18,
        );
    }

    #[test]
    fn complete_elp7_family_matches_short_periodic_order1_oracle_contributions() {
        let oracle = parse_short_periodic_order1_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_elp7_longitude_subset_for_audit::<{ coefficients::ELP7_RECORDS }>(
                JulianDate::new(2_488_070.0),
            );

        assert_eq!(oracle.len(), coefficients::ELP7_RECORDS);
        assert_eq!(evaluations.len(), coefficients::ELP7_RECORDS);

        for (term_index, (evaluation, row)) in evaluations.iter().zip(&oracle).enumerate() {
            assert_eq!(row.index, term_index + 1);
            assert_close_with_tolerance(evaluation.argument().value(), row.argument, 1.0e-12);
            assert_close_with_tolerance(evaluation.xx().value(), row.xx, 1.0e-18);
            assert_close_with_tolerance(
                evaluation.contribution().value(),
                row.contribution,
                1.0e-18,
            );
        }
    }

    #[test]
    fn complete_elp7_family_matches_short_periodic_order1_oracle_final_sum() {
        let oracle = parse_short_periodic_order1_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_elp7_longitude_subset_for_audit::<{ coefficients::ELP7_RECORDS }>(
                JulianDate::new(2_488_070.0),
            );
        let total: f64 = evaluations
            .iter()
            .map(|evaluation| evaluation.contribution().value())
            .sum();

        assert_close_with_tolerance(
            total,
            oracle
                .last()
                .expect("ELP7 oracle must include complete family")
                .partial_sum,
            1.0e-18,
        );
    }

    #[test]
    fn first_ten_elp4_terms_use_the_same_short_periodic_evaluator() {
        let first = Elp2000Evaluator::new()
            .evaluate_first_elp4_longitude_term_for_audit(JulianDate::new(2_451_545.0));
        let ten = Elp2000Evaluator::new()
            .evaluate_first_ten_elp4_longitude_terms_for_audit(JulianDate::new(2_451_545.0));

        assert_eq!(first, ten[0]);
    }

    #[test]
    fn first_ten_elp4_terms_match_short_periodic_oracle_contributions() {
        let oracle = parse_short_periodic_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_first_ten_elp4_longitude_terms_for_audit(JulianDate::new(2_451_545.0));

        assert!(oracle.len() >= 10);

        for (evaluation, row) in evaluations.iter().zip(oracle.iter().take(10)) {
            assert_close_with_tolerance(evaluation.argument().value(), row.argument, 1.0e-15);
            assert_close_with_tolerance(evaluation.xx().value(), row.xx, 1.0e-18);
            assert_close_with_tolerance(
                evaluation.contribution().value(),
                row.contribution,
                1.0e-18,
            );
        }
    }

    #[test]
    fn first_ten_elp4_terms_match_short_periodic_oracle_partial_sum() {
        let oracle = parse_short_periodic_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_first_ten_elp4_longitude_terms_for_audit(JulianDate::new(2_451_545.0));
        let partial_sum: f64 = evaluations
            .iter()
            .map(|evaluation| evaluation.contribution().value())
            .sum();

        assert_close_with_tolerance(
            partial_sum,
            oracle
                .get(9)
                .expect("ELP4 oracle must include tenth row")
                .partial_sum,
            1.0e-18,
        );
    }

    #[test]
    fn first_hundred_elp4_terms_use_the_same_short_periodic_evaluator() {
        let ten = Elp2000Evaluator::new()
            .evaluate_first_ten_elp4_longitude_terms_for_audit(JulianDate::new(2_451_545.0));
        let hundred = Elp2000Evaluator::new()
            .evaluate_first_hundred_elp4_longitude_terms_for_audit(JulianDate::new(2_451_545.0));

        assert_eq!(ten, hundred[..10]);
    }

    #[test]
    fn first_hundred_elp4_terms_match_short_periodic_oracle_contributions() {
        let oracle = parse_short_periodic_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_first_hundred_elp4_longitude_terms_for_audit(JulianDate::new(2_451_545.0));

        assert!(oracle.len() >= 100);

        for (term_index, (evaluation, row)) in
            evaluations.iter().zip(oracle.iter().take(100)).enumerate()
        {
            assert_eq!(row.index, term_index + 1);
            assert_close_with_tolerance(evaluation.argument().value(), row.argument, 1.0e-15);
            assert_close_with_tolerance(evaluation.xx().value(), row.xx, 1.0e-18);
            assert_close_with_tolerance(
                evaluation.contribution().value(),
                row.contribution,
                1.0e-18,
            );
        }
    }

    #[test]
    fn first_hundred_elp4_terms_match_short_periodic_oracle_checkpoints() {
        let oracle = parse_short_periodic_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_first_hundred_elp4_longitude_terms_for_audit(JulianDate::new(2_451_545.0));
        let checkpoints = [1, 10, 25, 50, 75, 100];

        for terms in checkpoints {
            let partial_sum: f64 = evaluations
                .iter()
                .take(terms)
                .map(|evaluation| evaluation.contribution().value())
                .sum();

            assert_close_with_tolerance(partial_sum, oracle[terms - 1].partial_sum, 1.0e-18);
        }
    }

    #[test]
    fn first_hundred_elp4_terms_validation_metrics_are_stable() {
        let oracle = parse_short_periodic_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_first_hundred_elp4_longitude_terms_for_audit(JulianDate::new(2_451_545.0));
        let metrics = short_periodic_validation_metrics(&evaluations, &oracle);

        assert_close_with_tolerance(metrics.max_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.mean_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.rms_error, 0.0, ORACLE_TOLERANCE);
    }

    #[test]
    fn complete_elp4_family_matches_short_periodic_oracle_contributions() {
        let oracle = parse_short_periodic_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_elp4_longitude_subset_for_audit::<{ coefficients::ELP4_RECORDS }>(
                JulianDate::new(2_451_545.0),
            );

        assert_eq!(oracle.len(), coefficients::ELP4_RECORDS);
        assert_eq!(evaluations.len(), coefficients::ELP4_RECORDS);

        for (term_index, (evaluation, row)) in evaluations.iter().zip(&oracle).enumerate() {
            assert_eq!(row.index, term_index + 1);
            assert_close_with_tolerance(evaluation.argument().value(), row.argument, 1.0e-15);
            assert_close_with_tolerance(evaluation.xx().value(), row.xx, 1.0e-18);
            assert_close_with_tolerance(
                evaluation.contribution().value(),
                row.contribution,
                1.0e-18,
            );
        }
    }

    #[test]
    fn complete_elp4_family_matches_short_periodic_oracle_checkpoints() {
        let oracle = parse_short_periodic_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_elp4_longitude_subset_for_audit::<{ coefficients::ELP4_RECORDS }>(
                JulianDate::new(2_451_545.0),
            );
        let checkpoints = [1, 10, 25, 50, 75, 100, 200, 300, coefficients::ELP4_RECORDS];

        for terms in checkpoints {
            let partial_sum: f64 = evaluations
                .iter()
                .take(terms)
                .map(|evaluation| evaluation.contribution().value())
                .sum();

            assert_close_with_tolerance(partial_sum, oracle[terms - 1].partial_sum, 1.0e-18);
        }
    }

    #[test]
    fn complete_elp4_family_validation_metrics_are_stable() {
        let oracle = parse_short_periodic_oracle_rows();
        let evaluations = Elp2000Evaluator::new()
            .evaluate_elp4_longitude_subset_for_audit::<{ coefficients::ELP4_RECORDS }>(
                JulianDate::new(2_451_545.0),
            );
        let metrics = short_periodic_validation_metrics(&evaluations, &oracle);

        assert_close_with_tolerance(metrics.max_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.mean_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.rms_error, 0.0, ORACLE_TOLERANCE);
    }

    #[test]
    fn first_ten_elp1_terms_use_the_same_audit_evaluator() {
        let first = Elp2000Evaluator::new()
            .evaluate_first_elp1_longitude_term_for_audit(JulianDate::new(2_451_545.0));
        let ten = Elp2000Evaluator::new()
            .evaluate_first_ten_elp1_longitude_terms_for_audit(JulianDate::new(2_451_545.0));

        assert_eq!(ten[0], first);
    }

    #[test]
    fn first_ten_elp1_terms_match_independent_oracle_contributions() {
        let evaluations = Elp2000Evaluator::new()
            .evaluate_first_ten_elp1_longitude_terms_for_audit(JulianDate::new(2_451_545.0));
        let expected = [
            46.909_407_726_369_665,
            0.095_184_379_669_358,
            0.000_198_222_088_956,
            -0.000_144_609_342_556,
            -0.068_095_222_465_122,
            -30.973_748_515_740_343,
            16_018.824_329_116_887,
            28.070_474_326_497_681,
            0.048_108_914_488_250,
            0.000_081_506_280_391,
        ];

        for (evaluation, expected) in evaluations.iter().zip(expected) {
            assert_close(evaluation.contribution().value(), expected);
        }
    }

    #[test]
    fn first_ten_elp1_terms_match_independent_oracle_partial_sum() {
        let evaluations = Elp2000Evaluator::new()
            .evaluate_first_ten_elp1_longitude_terms_for_audit(JulianDate::new(2_451_545.0));
        let sum: f64 = evaluations
            .iter()
            .map(|evaluation| evaluation.contribution().value())
            .sum();

        assert_close(sum, 16_062.905_795_844_736);
    }

    #[test]
    fn elp1_subset_of_one_hundred_terms_matches_independent_oracle_checkpoints() {
        let evaluations = Elp2000Evaluator::new()
            .evaluate_elp1_longitude_subset_for_audit::<100>(JulianDate::new(2_451_545.0));
        let checkpoints = [
            (1, 46.909_407_726_369_665),
            (10, 16_062.905_795_844_736),
            (25, 15_314.322_215_712_033),
            (50, 15_304.996_677_063_071),
            (75, 15_360.039_551_360_735),
            (100, 15_362.154_930_280_751),
        ];

        for (terms, expected) in checkpoints {
            let partial_sum: f64 = evaluations
                .iter()
                .take(terms)
                .map(|evaluation| evaluation.contribution().value())
                .sum();

            assert_close(partial_sum, expected);
        }
    }

    #[test]
    fn complete_elp1_family_matches_independent_oracle() {
        let oracle = parse_oracle_rows();
        let audit =
            Elp2000Evaluator::new().audit_family(ElpFamily::Elp1, JulianDate::new(2_451_545.0));
        let evaluations = audit
            .main_problem_terms()
            .expect("ELP1 audit must produce main-problem terms");

        assert_eq!(oracle.len(), coefficients::ELP1_RECORDS);
        assert_eq!(evaluations.len(), coefficients::ELP1_RECORDS);

        for (term_index, (evaluation, row)) in evaluations.iter().zip(&oracle).enumerate() {
            assert_eq!(row.index, term_index + 1);
            assert_close_with_tolerance(
                evaluation.contribution().value(),
                row.contribution,
                ORACLE_TOLERANCE,
            );
        }

        let total: f64 = evaluations
            .iter()
            .map(|evaluation| evaluation.contribution().value())
            .sum();
        let final_oracle_sum = oracle
            .last()
            .expect("oracle must contain the complete ELP1 family")
            .partial_sum;

        assert_close_with_tolerance(total, final_oracle_sum, ORACLE_TOLERANCE);
    }

    #[test]
    fn complete_elp1_family_validation_metrics_are_stable() {
        let oracle = parse_oracle_rows();
        let audit =
            Elp2000Evaluator::new().audit_family(ElpFamily::Elp1, JulianDate::new(2_451_545.0));
        let evaluations = audit
            .main_problem_terms()
            .expect("ELP1 audit must produce main-problem terms");
        let metrics = validation_metrics(evaluations, &oracle);

        assert_close_with_tolerance(metrics.max_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.mean_abs_error, 0.0, ORACLE_TOLERANCE);
        assert_close_with_tolerance(metrics.rms_error, 0.0, ORACLE_TOLERANCE);
    }
}
