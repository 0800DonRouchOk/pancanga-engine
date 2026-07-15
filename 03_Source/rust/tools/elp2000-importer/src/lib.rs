//! Reproducible importer for the IMCCE ELP2000-82B coefficient files.
//!
//! The importer validates the 36 official `ELP1` through `ELP36` files before
//! generating Rust constants. It intentionally preserves the decimal spelling
//! found in the source files so human audits can compare generated output back
//! to the IMCCE data.

use std::fmt::{self, Write as _};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const FNV_OFFSET: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

/// Data import format version used by Pancanga Engine.
pub const IMPORT_VERSION: &str = "1";

/// Parser and generator version used to create ELP2000 data artifacts.
pub const IMPORTER_VERSION: &str = "1.0.0";

/// Scientific data source handled by this importer.
pub const THEORY_NAME: &str = "ELP2000-82B";

/// Static metadata for one official ELP2000-82B data file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElpFileSpec {
    /// File number, from 1 to 36.
    pub id: u8,
    /// Official file name, such as `ELP1`.
    pub name: &'static str,
    /// Expected logical record length published by IMCCE.
    pub logical_record_length: usize,
    /// Expected number of coefficient records after the header record.
    pub expected_records: usize,
    /// Parser kind implied by the original Fortran format.
    pub kind: ElpFileKind,
}

/// Parser kind for the three ELP2000-82B record layouts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElpFileKind {
    /// Main problem files, parsed with Fortran format `4i3,2x,f13.5,6(2x,f10.2)`.
    MainProblem,
    /// Short periodic files, parsed with Fortran format `5i3,1x,f9.5,1x,f9.5`.
    ShortPeriodic,
    /// Planetary perturbation files, parsed with Fortran format `11i3,1x,f9.5,1x,f9.5`.
    Planetary,
}

/// Parsed content of one ELP file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedElpFile {
    /// Metadata for the parsed file.
    pub spec: ElpFileSpec,
    /// FNV-1a checksum of the source bytes.
    pub source_fingerprint: u64,
    /// Parsed coefficient records.
    pub records: Vec<ElpRecord>,
}

/// One parsed coefficient record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElpRecord {
    /// Main problem term.
    MainProblem(MainProblemTerm),
    /// Short periodic term.
    ShortPeriodic(ShortPeriodicTerm),
    /// Planetary perturbation term.
    Planetary(PlanetaryTerm),
}

/// Main problem term from `ELP1`, `ELP2`, or `ELP3`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MainProblemTerm {
    /// Multipliers of Delaunay arguments.
    pub multipliers: [i16; 4],
    /// Main amplitude, preserved as source decimal text.
    pub amplitude: String,
    /// Six correction coefficients, preserved as source decimal text.
    pub corrections: [String; 6],
}

/// Short periodic term from files outside the main and planetary groups.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortPeriodicTerm {
    /// Multiplier for the zeta argument.
    pub zeta_multiplier: i16,
    /// Multipliers of Delaunay arguments.
    pub multipliers: [i16; 4],
    /// Phase in degrees, preserved as source decimal text.
    pub phase_degrees: String,
    /// Amplitude, preserved as source decimal text.
    pub amplitude: String,
}

/// Planetary perturbation term from `ELP10` through `ELP21`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanetaryTerm {
    /// Multipliers of planetary and lunar arguments.
    pub multipliers: [i16; 11],
    /// Phase in degrees, preserved as source decimal text.
    pub phase_degrees: String,
    /// Amplitude, preserved as source decimal text.
    pub amplitude: String,
}

/// Summary returned after verifying an input directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportSummary {
    /// Parsed files in official order.
    pub files: Vec<ParsedElpFile>,
}

impl ImportSummary {
    /// Returns the total number of parsed coefficient records.
    pub fn total_records(&self) -> usize {
        self.files.iter().map(|file| file.records.len()).sum()
    }

    /// Returns a deterministic fingerprint of the complete imported dataset.
    pub fn dataset_fingerprint(&self) -> u64 {
        self.files.iter().fold(FNV_OFFSET, |hash, file| {
            let hash = fnv1a_update(hash, file.spec.name.as_bytes());
            let hash = fnv1a_update(hash, &file.source_fingerprint.to_le_bytes());
            fnv1a_update(hash, &(file.records.len() as u64).to_le_bytes())
        })
    }
}

/// Error produced while parsing or generating ELP data.
#[derive(Debug)]
pub enum ImportError {
    /// Filesystem error.
    Io { path: PathBuf, source: io::Error },
    /// Expected file was not present.
    MissingFile { path: PathBuf },
    /// Source file did not match the expected ELP layout.
    InvalidFile { file: String, message: String },
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImportError::Io { path, source } => {
                write!(f, "{}: {}", path.display(), source)
            }
            ImportError::MissingFile { path } => {
                write!(f, "missing required ELP file: {}", path.display())
            }
            ImportError::InvalidFile { file, message } => {
                write!(f, "{}: {}", file, message)
            }
        }
    }
}

impl std::error::Error for ImportError {}

/// Metadata for the 36 official ELP2000-82B files.
pub const ELP_FILE_SPECS: &[ElpFileSpec] = &[
    spec(1, 99, 1023, ElpFileKind::MainProblem),
    spec(2, 99, 918, ElpFileKind::MainProblem),
    spec(3, 99, 704, ElpFileKind::MainProblem),
    spec(4, 45, 347, ElpFileKind::ShortPeriodic),
    spec(5, 45, 316, ElpFileKind::ShortPeriodic),
    spec(6, 45, 237, ElpFileKind::ShortPeriodic),
    spec(7, 45, 14, ElpFileKind::ShortPeriodic),
    spec(8, 45, 11, ElpFileKind::ShortPeriodic),
    spec(9, 45, 8, ElpFileKind::ShortPeriodic),
    spec(10, 63, 14328, ElpFileKind::Planetary),
    spec(11, 63, 5233, ElpFileKind::Planetary),
    spec(12, 63, 6631, ElpFileKind::Planetary),
    spec(13, 63, 4384, ElpFileKind::Planetary),
    spec(14, 63, 833, ElpFileKind::Planetary),
    spec(15, 63, 1715, ElpFileKind::Planetary),
    spec(16, 63, 170, ElpFileKind::Planetary),
    spec(17, 63, 150, ElpFileKind::Planetary),
    spec(18, 63, 114, ElpFileKind::Planetary),
    spec(19, 63, 226, ElpFileKind::Planetary),
    spec(20, 63, 188, ElpFileKind::Planetary),
    spec(21, 63, 169, ElpFileKind::Planetary),
    spec(22, 45, 3, ElpFileKind::ShortPeriodic),
    spec(23, 45, 2, ElpFileKind::ShortPeriodic),
    spec(24, 45, 2, ElpFileKind::ShortPeriodic),
    spec(25, 45, 6, ElpFileKind::ShortPeriodic),
    spec(26, 45, 4, ElpFileKind::ShortPeriodic),
    spec(27, 45, 5, ElpFileKind::ShortPeriodic),
    spec(28, 45, 20, ElpFileKind::ShortPeriodic),
    spec(29, 45, 12, ElpFileKind::ShortPeriodic),
    spec(30, 45, 14, ElpFileKind::ShortPeriodic),
    spec(31, 45, 11, ElpFileKind::ShortPeriodic),
    spec(32, 45, 4, ElpFileKind::ShortPeriodic),
    spec(33, 45, 10, ElpFileKind::ShortPeriodic),
    spec(34, 53, 28, ElpFileKind::ShortPeriodic),
    spec(35, 52, 13, ElpFileKind::ShortPeriodic),
    spec(36, 52, 19, ElpFileKind::ShortPeriodic),
];

const fn spec(
    id: u8,
    logical_record_length: usize,
    expected_records: usize,
    kind: ElpFileKind,
) -> ElpFileSpec {
    ElpFileSpec {
        id,
        name: file_name(id),
        logical_record_length,
        expected_records,
        kind,
    }
}

const fn file_name(id: u8) -> &'static str {
    match id {
        1 => "ELP1",
        2 => "ELP2",
        3 => "ELP3",
        4 => "ELP4",
        5 => "ELP5",
        6 => "ELP6",
        7 => "ELP7",
        8 => "ELP8",
        9 => "ELP9",
        10 => "ELP10",
        11 => "ELP11",
        12 => "ELP12",
        13 => "ELP13",
        14 => "ELP14",
        15 => "ELP15",
        16 => "ELP16",
        17 => "ELP17",
        18 => "ELP18",
        19 => "ELP19",
        20 => "ELP20",
        21 => "ELP21",
        22 => "ELP22",
        23 => "ELP23",
        24 => "ELP24",
        25 => "ELP25",
        26 => "ELP26",
        27 => "ELP27",
        28 => "ELP28",
        29 => "ELP29",
        30 => "ELP30",
        31 => "ELP31",
        32 => "ELP32",
        33 => "ELP33",
        34 => "ELP34",
        35 => "ELP35",
        36 => "ELP36",
        _ => "",
    }
}

/// Verifies and parses all 36 ELP files in an input directory.
pub fn import_directory(path: impl AsRef<Path>) -> Result<ImportSummary, ImportError> {
    let root = path.as_ref();
    let mut files = Vec::with_capacity(ELP_FILE_SPECS.len());

    for spec in ELP_FILE_SPECS {
        let file_path = root.join(spec.name);
        if !file_path.exists() {
            return Err(ImportError::MissingFile { path: file_path });
        }

        let bytes = fs::read(&file_path).map_err(|source| ImportError::Io {
            path: file_path.clone(),
            source,
        })?;
        let text = String::from_utf8(bytes.clone()).map_err(|error| ImportError::InvalidFile {
            file: spec.name.to_string(),
            message: format!("source is not UTF-8: {}", error),
        })?;
        files.push(parse_file(*spec, &text, fnv1a_64(&bytes))?);
    }

    Ok(ImportSummary { files })
}

/// Parses one ELP file using its official metadata.
pub fn parse_file(
    spec: ElpFileSpec,
    text: &str,
    source_fingerprint: u64,
) -> Result<ParsedElpFile, ImportError> {
    let mut lines = text.lines();
    lines
        .next()
        .ok_or_else(|| invalid(spec, "missing header record"))?;

    let records = lines
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(index, line)| parse_record(spec, index + 2, line))
        .collect::<Result<Vec<_>, _>>()?;

    if records.len() != spec.expected_records {
        return Err(invalid(
            spec,
            format!(
                "expected {} coefficient records, found {}",
                spec.expected_records,
                records.len()
            ),
        ));
    }

    Ok(ParsedElpFile {
        spec,
        source_fingerprint,
        records,
    })
}

/// Generates Rust constants from a verified import summary.
pub fn generate_rust(summary: &ImportSummary) -> Result<String, fmt::Error> {
    let mut output = String::new();

    writeln!(
        output,
        "//! ELP2000-82B lunar coefficients generated by `elp2000-importer`."
    )?;
    writeln!(output, "//!")?;
    writeln!(
        output,
        "//! Do not edit by hand. Regenerate from the official IMCCE `ELP1` through `ELP36` files."
    )?;
    writeln!(output)?;
    writeln!(output, "#![allow(clippy::excessive_precision)]")?;
    writeln!(output)?;
    writeln!(output, "#[derive(Debug, Clone, Copy)]")?;
    writeln!(output, "pub(super) struct ElpMainProblemTerm {{")?;
    writeln!(output, "    pub(super) multipliers: [i16; 4],")?;
    writeln!(output, "    pub(super) amplitude: f64,")?;
    writeln!(output, "    pub(super) corrections: [f64; 6],")?;
    writeln!(output, "}}")?;
    writeln!(output)?;
    writeln!(output, "#[derive(Debug, Clone, Copy)]")?;
    writeln!(output, "pub(super) struct ElpShortPeriodicTerm {{")?;
    writeln!(output, "    pub(super) zeta_multiplier: i16,")?;
    writeln!(output, "    pub(super) multipliers: [i16; 4],")?;
    writeln!(output, "    pub(super) phase_degrees: f64,")?;
    writeln!(output, "    pub(super) amplitude: f64,")?;
    writeln!(output, "}}")?;
    writeln!(output)?;
    writeln!(output, "#[derive(Debug, Clone, Copy)]")?;
    writeln!(output, "pub(super) struct ElpPlanetaryTerm {{")?;
    writeln!(output, "    pub(super) multipliers: [i16; 11],")?;
    writeln!(output, "    pub(super) phase_degrees: f64,")?;
    writeln!(output, "    pub(super) amplitude: f64,")?;
    writeln!(output, "}}")?;

    for file in &summary.files {
        write_file_constants(&mut output, file)?;
    }

    Ok(output)
}

/// Generates a deterministic JSON manifest for an imported ELP2000 dataset.
///
/// The manifest deliberately omits wall-clock timestamps so repeated imports
/// from the same source files produce byte-identical output on every machine.
pub fn generate_manifest(
    summary: &ImportSummary,
    generated_rust: &str,
) -> Result<String, fmt::Error> {
    let mut output = String::new();

    writeln!(output, "{{")?;
    writeln!(output, "  \"theory\": \"{}\",", THEORY_NAME)?;
    writeln!(output, "  \"import_version\": \"{}\",", IMPORT_VERSION)?;
    writeln!(output, "  \"importer_version\": \"{}\",", IMPORTER_VERSION)?;
    writeln!(output, "  \"source\": \"IMCCE / Observatoire de Paris\",")?;
    writeln!(output, "  \"files\": {},", summary.files.len())?;
    writeln!(output, "  \"records\": {},", summary.total_records())?;
    writeln!(
        output,
        "  \"dataset_fingerprint\": \"0x{:016x}\",",
        summary.dataset_fingerprint()
    )?;
    writeln!(
        output,
        "  \"generated_rust_fingerprint\": \"0x{:016x}\",",
        fingerprint(generated_rust.as_bytes())
    )?;
    writeln!(
        output,
        "  \"timestamp_policy\": \"omitted for reproducibility\","
    )?;
    writeln!(output, "  \"source_files\": [")?;

    for (index, file) in summary.files.iter().enumerate() {
        let comma = if index + 1 == summary.files.len() {
            ""
        } else {
            ","
        };
        writeln!(output, "    {{")?;
        writeln!(output, "      \"name\": \"{}\",", file.spec.name)?;
        writeln!(output, "      \"records\": {},", file.records.len())?;
        writeln!(
            output,
            "      \"fingerprint\": \"0x{:016x}\"",
            file.source_fingerprint
        )?;
        writeln!(output, "    }}{}", comma)?;
    }

    writeln!(output, "  ]")?;
    writeln!(output, "}}")?;
    Ok(output)
}

fn write_file_constants(output: &mut String, file: &ParsedElpFile) -> Result<(), fmt::Error> {
    writeln!(output)?;
    writeln!(
        output,
        "pub(super) const {}_FINGERPRINT: u64 = 0x{:016x};",
        file.spec.name, file.source_fingerprint
    )?;
    writeln!(
        output,
        "pub(super) const {}_RECORDS: usize = {};",
        file.spec.name,
        file.records.len()
    )?;

    match file.spec.kind {
        ElpFileKind::MainProblem => write_main_problem_terms(output, file),
        ElpFileKind::ShortPeriodic => write_short_periodic_terms(output, file),
        ElpFileKind::Planetary => write_planetary_terms(output, file),
    }
}

fn write_main_problem_terms(output: &mut String, file: &ParsedElpFile) -> Result<(), fmt::Error> {
    writeln!(
        output,
        "pub(super) const {}: &[ElpMainProblemTerm] = &[",
        file.spec.name
    )?;
    for record in &file.records {
        let ElpRecord::MainProblem(term) = record else {
            continue;
        };
        writeln!(output, "    ElpMainProblemTerm {{")?;
        writeln!(output, "        multipliers: {:?},", term.multipliers)?;
        writeln!(output, "        amplitude: {},", term.amplitude)?;
        writeln!(output, "        corrections: [")?;
        for correction in &term.corrections {
            writeln!(output, "            {},", correction)?;
        }
        writeln!(output, "        ],")?;
        writeln!(output, "    }},")?;
    }
    writeln!(output, "];")
}

fn write_short_periodic_terms(output: &mut String, file: &ParsedElpFile) -> Result<(), fmt::Error> {
    writeln!(
        output,
        "pub(super) const {}: &[ElpShortPeriodicTerm] = &[",
        file.spec.name
    )?;
    for record in &file.records {
        let ElpRecord::ShortPeriodic(term) = record else {
            continue;
        };
        writeln!(output, "    ElpShortPeriodicTerm {{")?;
        writeln!(output, "        zeta_multiplier: {},", term.zeta_multiplier)?;
        writeln!(output, "        multipliers: {:?},", term.multipliers)?;
        writeln!(output, "        phase_degrees: {},", term.phase_degrees)?;
        writeln!(output, "        amplitude: {},", term.amplitude)?;
        writeln!(output, "    }},")?;
    }
    writeln!(output, "];")
}

fn write_planetary_terms(output: &mut String, file: &ParsedElpFile) -> Result<(), fmt::Error> {
    writeln!(
        output,
        "pub(super) const {}: &[ElpPlanetaryTerm] = &[",
        file.spec.name
    )?;
    for record in &file.records {
        let ElpRecord::Planetary(term) = record else {
            continue;
        };
        writeln!(output, "    ElpPlanetaryTerm {{")?;
        writeln!(output, "        multipliers: {:?},", term.multipliers)?;
        writeln!(output, "        phase_degrees: {},", term.phase_degrees)?;
        writeln!(output, "        amplitude: {},", term.amplitude)?;
        writeln!(output, "    }},")?;
    }
    writeln!(output, "];")
}

fn parse_record(
    spec: ElpFileSpec,
    line_number: usize,
    line: &str,
) -> Result<ElpRecord, ImportError> {
    if line.len() > spec.logical_record_length {
        return Err(invalid(
            spec,
            format!(
                "line {} is longer than published Lrecl {}",
                line_number, spec.logical_record_length
            ),
        ));
    }

    match spec.kind {
        ElpFileKind::MainProblem => parse_main_problem(spec, line_number, line),
        ElpFileKind::ShortPeriodic => parse_short_periodic(spec, line_number, line),
        ElpFileKind::Planetary => parse_planetary(spec, line_number, line),
    }
}

fn parse_main_problem(
    spec: ElpFileSpec,
    line_number: usize,
    line: &str,
) -> Result<ElpRecord, ImportError> {
    let fields = fixed_fields(
        line,
        &[3, 3, 3, 3, 2, 13, 2, 10, 2, 10, 2, 10, 2, 10, 2, 10, 2, 10],
    );
    let multipliers = [
        parse_i16(spec, line_number, fields[0])?,
        parse_i16(spec, line_number, fields[1])?,
        parse_i16(spec, line_number, fields[2])?,
        parse_i16(spec, line_number, fields[3])?,
    ];
    let amplitude = parse_decimal(spec, line_number, fields[5])?;
    let corrections = [
        parse_decimal(spec, line_number, fields[7])?,
        parse_decimal(spec, line_number, fields[9])?,
        parse_decimal(spec, line_number, fields[11])?,
        parse_decimal(spec, line_number, fields[13])?,
        parse_decimal(spec, line_number, fields[15])?,
        parse_decimal(spec, line_number, fields[17])?,
    ];

    Ok(ElpRecord::MainProblem(MainProblemTerm {
        multipliers,
        amplitude,
        corrections,
    }))
}

fn parse_short_periodic(
    spec: ElpFileSpec,
    line_number: usize,
    line: &str,
) -> Result<ElpRecord, ImportError> {
    let fields = fixed_fields(line, &[3, 3, 3, 3, 3, 1, 9, 1, 9]);
    let zeta_multiplier = parse_i16(spec, line_number, fields[0])?;
    let multipliers = [
        parse_i16(spec, line_number, fields[1])?,
        parse_i16(spec, line_number, fields[2])?,
        parse_i16(spec, line_number, fields[3])?,
        parse_i16(spec, line_number, fields[4])?,
    ];
    let phase_degrees = parse_decimal(spec, line_number, fields[6])?;
    let amplitude = parse_decimal(spec, line_number, fields[8])?;

    Ok(ElpRecord::ShortPeriodic(ShortPeriodicTerm {
        zeta_multiplier,
        multipliers,
        phase_degrees,
        amplitude,
    }))
}

fn parse_planetary(
    spec: ElpFileSpec,
    line_number: usize,
    line: &str,
) -> Result<ElpRecord, ImportError> {
    let fields = fixed_fields(line, &[3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 9, 1, 9]);
    let multipliers = [
        parse_i16(spec, line_number, fields[0])?,
        parse_i16(spec, line_number, fields[1])?,
        parse_i16(spec, line_number, fields[2])?,
        parse_i16(spec, line_number, fields[3])?,
        parse_i16(spec, line_number, fields[4])?,
        parse_i16(spec, line_number, fields[5])?,
        parse_i16(spec, line_number, fields[6])?,
        parse_i16(spec, line_number, fields[7])?,
        parse_i16(spec, line_number, fields[8])?,
        parse_i16(spec, line_number, fields[9])?,
        parse_i16(spec, line_number, fields[10])?,
    ];
    let phase_degrees = parse_decimal(spec, line_number, fields[12])?;
    let amplitude = parse_decimal(spec, line_number, fields[14])?;

    Ok(ElpRecord::Planetary(PlanetaryTerm {
        multipliers,
        phase_degrees,
        amplitude,
    }))
}

fn fixed_fields<'a>(line: &'a str, widths: &[usize]) -> Vec<&'a str> {
    let mut start = 0;
    let mut fields = Vec::with_capacity(widths.len());
    for width in widths {
        if start >= line.len() {
            fields.push("");
        } else {
            let end = (start + width).min(line.len());
            fields.push(&line[start..end]);
        }
        start += width;
    }
    fields
}

fn parse_i16(spec: ElpFileSpec, line_number: usize, field: &str) -> Result<i16, ImportError> {
    field.trim().parse::<i16>().map_err(|error| {
        invalid(
            spec,
            format!(
                "line {} has invalid integer field {:?}: {}",
                line_number, field, error
            ),
        )
    })
}

fn parse_decimal(
    spec: ElpFileSpec,
    line_number: usize,
    field: &str,
) -> Result<String, ImportError> {
    let decimal = field.trim();
    decimal.parse::<f64>().map_err(|error| {
        invalid(
            spec,
            format!(
                "line {} has invalid decimal field {:?}: {}",
                line_number, field, error
            ),
        )
    })?;
    Ok(decimal.to_string())
}

fn fnv1a_64(bytes: &[u8]) -> u64 {
    fingerprint(bytes)
}

/// Returns the deterministic FNV-1a fingerprint used by generated manifests.
pub fn fingerprint(bytes: &[u8]) -> u64 {
    fnv1a_update(FNV_OFFSET, bytes)
}

fn fnv1a_update(hash: u64, bytes: &[u8]) -> u64 {
    bytes.iter().fold(hash, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(FNV_PRIME)
    })
}

fn invalid(spec: ElpFileSpec, message: impl Into<String>) -> ImportError {
    ImportError::InvalidFile {
        file: spec.name.to_string(),
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_contains_all_official_files() {
        assert_eq!(ELP_FILE_SPECS.len(), 36);
        assert_eq!(ELP_FILE_SPECS[0].name, "ELP1");
        assert_eq!(ELP_FILE_SPECS[0].expected_records, 1023);
        assert_eq!(ELP_FILE_SPECS[9].name, "ELP10");
        assert_eq!(ELP_FILE_SPECS[9].expected_records, 14328);
        assert_eq!(ELP_FILE_SPECS[35].name, "ELP36");
        assert_eq!(ELP_FILE_SPECS[35].expected_records, 19);
    }

    #[test]
    fn parses_main_problem_fixed_width_record() {
        let spec = ElpFileSpec {
            expected_records: 1,
            ..ELP_FILE_SPECS[0]
        };
        let text = format!(
            "\n{:>3}{:>3}{:>3}{:>3}  {:>13}{:>2}{:>10}{:>2}{:>10}{:>2}{:>10}{:>2}{:>10}{:>2}{:>10}{:>2}{:>10}\n",
            0, 0, 1, -2, "628.87740", "", "0.00", "", "1.23", "", "-4.50", "", "6.70", "", "8.90", "", "10.11"
        );

        let parsed = parse_file(spec, &text, 0).unwrap();

        assert_eq!(parsed.records.len(), 1);
        let ElpRecord::MainProblem(term) = &parsed.records[0] else {
            panic!("expected main problem term");
        };
        assert_eq!(term.multipliers, [0, 0, 1, -2]);
        assert_eq!(term.amplitude, "628.87740");
        assert_eq!(term.corrections[2], "-4.50");
    }

    #[test]
    fn parses_short_periodic_fixed_width_record() {
        let spec = ElpFileSpec {
            expected_records: 1,
            ..ELP_FILE_SPECS[3]
        };
        let text = format!(
            "\n{:>3}{:>3}{:>3}{:>3}{:>3} {:>9} {:>9}\n",
            1, -2, 0, 3, -1, "12.34567", "89.12345"
        );

        let parsed = parse_file(spec, &text, 0).unwrap();

        let ElpRecord::ShortPeriodic(term) = &parsed.records[0] else {
            panic!("expected short periodic term");
        };
        assert_eq!(term.zeta_multiplier, 1);
        assert_eq!(term.multipliers, [-2, 0, 3, -1]);
        assert_eq!(term.phase_degrees, "12.34567");
        assert_eq!(term.amplitude, "89.12345");
    }

    #[test]
    fn parses_planetary_fixed_width_record() {
        let spec = ElpFileSpec {
            expected_records: 1,
            ..ELP_FILE_SPECS[9]
        };
        let text = format!(
            "\n{:>3}{:>3}{:>3}{:>3}{:>3}{:>3}{:>3}{:>3}{:>3}{:>3}{:>3} {:>9} {:>9}\n",
            1, 0, -1, 2, -2, 3, 0, 1, -3, 2, -1, "45.00000", "12.50000"
        );

        let parsed = parse_file(spec, &text, 0).unwrap();

        let ElpRecord::Planetary(term) = &parsed.records[0] else {
            panic!("expected planetary term");
        };
        assert_eq!(term.multipliers, [1, 0, -1, 2, -2, 3, 0, 1, -3, 2, -1]);
        assert_eq!(term.phase_degrees, "45.00000");
        assert_eq!(term.amplitude, "12.50000");
    }

    #[test]
    fn rejects_unexpected_record_count() {
        let spec = ElpFileSpec {
            expected_records: 2,
            ..ELP_FILE_SPECS[3]
        };
        let text = format!(
            "\n{:>3}{:>3}{:>3}{:>3}{:>3} {:>9} {:>9}\n",
            1, -2, 0, 3, -1, "12.34567", "89.12345"
        );

        let error = parse_file(spec, &text, 0).unwrap_err();

        assert!(error.to_string().contains("expected 2 coefficient records"));
    }

    #[test]
    fn generated_rust_preserves_decimal_spelling() {
        let spec = ElpFileSpec {
            expected_records: 1,
            ..ELP_FILE_SPECS[3]
        };
        let parsed = parse_file(
            spec,
            &format!(
                "\n{:>3}{:>3}{:>3}{:>3}{:>3} {:>9} {:>9}\n",
                1, -2, 0, 3, -1, "12.34567", "89.12345"
            ),
            0x1234,
        )
        .unwrap();
        let rust = generate_rust(&ImportSummary {
            files: vec![parsed],
        })
        .unwrap();

        assert!(rust.contains("phase_degrees: 12.34567"));
        assert!(rust.contains("amplitude: 89.12345"));
        assert!(rust.contains("ELP4_FINGERPRINT"));
    }

    #[test]
    fn manifest_is_deterministic_and_omits_timestamps() {
        let spec = ElpFileSpec {
            expected_records: 1,
            ..ELP_FILE_SPECS[3]
        };
        let parsed = parse_file(
            spec,
            &format!(
                "\n{:>3}{:>3}{:>3}{:>3}{:>3} {:>9} {:>9}\n",
                1, -2, 0, 3, -1, "12.34567", "89.12345"
            ),
            0x1234,
        )
        .unwrap();
        let summary = ImportSummary {
            files: vec![parsed],
        };

        let rust = generate_rust(&summary).unwrap();
        let first = generate_manifest(&summary, &rust).unwrap();
        let second = generate_manifest(&summary, &rust).unwrap();

        assert_eq!(first, second);
        assert!(first.contains("\"import_version\": \"1\""));
        assert!(first.contains("\"importer_version\": \"1.0.0\""));
        assert!(first.contains("\"generated_rust_fingerprint\""));
        assert!(first.contains("\"timestamp_policy\": \"omitted for reproducibility\""));
        assert!(!first.contains("generated_at"));
    }

    #[test]
    fn generation_is_byte_stable_for_same_import_summary() {
        let spec = ElpFileSpec {
            expected_records: 1,
            ..ELP_FILE_SPECS[3]
        };
        let parsed = parse_file(
            spec,
            &format!(
                "\n{:>3}{:>3}{:>3}{:>3}{:>3} {:>9} {:>9}\n",
                1, -2, 0, 3, -1, "12.34567", "89.12345"
            ),
            0x1234,
        )
        .unwrap();
        let summary = ImportSummary {
            files: vec![parsed],
        };

        let first_rust = generate_rust(&summary).unwrap();
        let first_manifest = generate_manifest(&summary, &first_rust).unwrap();
        let second_rust = generate_rust(&summary).unwrap();
        let second_manifest = generate_manifest(&summary, &second_rust).unwrap();

        assert_eq!(first_rust.as_bytes(), second_rust.as_bytes());
        assert_eq!(first_manifest.as_bytes(), second_manifest.as_bytes());
        assert_eq!(
            fingerprint(first_rust.as_bytes()),
            fingerprint(second_rust.as_bytes())
        );
        assert!(first_manifest.contains(&format!(
            "\"generated_rust_fingerprint\": \"0x{:016x}\"",
            fingerprint(first_rust.as_bytes())
        )));
    }

    #[test]
    fn official_dataset_imports_expected_counts() {
        let summary = import_directory(official_raw_dir()).unwrap();

        assert_eq!(summary.files.len(), 36);
        assert_eq!(summary.total_records(), 37_872);

        for (file, spec) in summary.files.iter().zip(ELP_FILE_SPECS) {
            assert_eq!(file.spec.name, spec.name);
            assert_eq!(file.records.len(), spec.expected_records);
        }
    }

    #[test]
    fn generated_artifacts_are_in_sync_with_official_dataset() {
        let summary = import_directory(official_raw_dir()).unwrap();
        let generated_rust = generate_rust(&summary).unwrap();
        let generated_manifest = generate_manifest(&summary, &generated_rust).unwrap();

        let stored_rust = fs::read_to_string(
            project_root().join("06_Data/Reference/ELP2000/generated/elp2000_coefficients.rs"),
        )
        .unwrap();
        let stored_manifest = fs::read_to_string(
            project_root().join("06_Data/Reference/ELP2000/generated/elp2000_manifest.json"),
        )
        .unwrap();

        assert_eq!(generated_rust.as_bytes(), stored_rust.as_bytes());
        assert_eq!(generated_manifest.as_bytes(), stored_manifest.as_bytes());
        assert_eq!(summary.dataset_fingerprint(), 0x3aa7ebfdd62d91dc);
        assert_eq!(fingerprint(generated_rust.as_bytes()), 0x338c91efeeaee125);
    }

    fn official_raw_dir() -> PathBuf {
        project_root().join("06_Data/Reference/ELP2000/raw")
    }

    fn project_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../../../")
            .canonicalize()
            .unwrap()
    }
}
