use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use elp2000_importer::{generate_manifest, generate_rust, import_directory};

const DEFAULT_INPUT_DIR: &str = "../../06_Data/Reference/ELP2000/raw";
const DEFAULT_OUTPUT_RS: &str = "../../06_Data/Reference/ELP2000/generated/elp2000_coefficients.rs";
const DEFAULT_MANIFEST: &str = "../../06_Data/Reference/ELP2000/generated/elp2000_manifest.json";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<_>>();

    match args.as_slice() {
        [_] => generate_outputs(DEFAULT_INPUT_DIR, DEFAULT_OUTPUT_RS, DEFAULT_MANIFEST),
        [_, command, input_dir] if command == "verify" => {
            let summary = import_directory(input_dir)?;
            println!("ELP2000 import verification passed.");
            println!("Files: {}", summary.files.len());
            println!("Records: {}", summary.total_records());
            Ok(())
        }
        [_, command, input_dir, output_file] if command == "generate" => {
            let summary = import_directory(input_dir)?;
            let rust = generate_rust(&summary)?;
            fs::write(output_file, rust)?;
            println!("ELP2000 Rust coefficient file generated.");
            println!("Files: {}", summary.files.len());
            println!("Records: {}", summary.total_records());
            println!("Output: {output_file}");
            Ok(())
        }
        [_, command, input_dir, output_file, manifest_file] if command == "generate-all" => {
            generate_outputs(input_dir, output_file, manifest_file)
        }
        [program, ..] => {
            eprintln!("Usage:");
            eprintln!("  {program}");
            eprintln!("  {program} verify <elp-input-dir>");
            eprintln!("  {program} generate <elp-input-dir> <output-rs>");
            eprintln!("  {program} generate-all <elp-input-dir> <output-rs> <manifest-json>");
            Err("invalid arguments".into())
        }
        [] => Err("missing program name".into()),
    }
}

fn generate_outputs(
    input_dir: impl AsRef<Path>,
    output_file: impl AsRef<Path>,
    manifest_file: impl AsRef<Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let input_dir = input_dir.as_ref();
    let output_file = output_file.as_ref();
    let manifest_file = manifest_file.as_ref();
    let summary = import_directory(input_dir)?;
    let rust = generate_rust(&summary)?;
    let manifest = generate_manifest(&summary, &rust)?;

    create_parent_dir(output_file)?;
    create_parent_dir(manifest_file)?;
    fs::write(output_file, rust)?;
    fs::write(manifest_file, manifest)?;

    println!("ELP2000 deterministic import generated.");
    println!("Files: {}", summary.files.len());
    println!("Records: {}", summary.total_records());
    println!(
        "Dataset fingerprint: 0x{:016x}",
        summary.dataset_fingerprint()
    );
    println!("Rust output: {}", output_file.display());
    println!("Manifest: {}", manifest_file.display());
    Ok(())
}

fn create_parent_dir(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(PathBuf::from(parent))?;
    }
    Ok(())
}
