use clap::Parser;
use g2svg::{convert_file, ConverterConfig};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "g2svg",
    author = "Antigravity",
    version = "0.1.0",
    about = "Converts power system CIM/G files to CIM/SVG conforming to IEC 61970-453 and IEC TS 61970-556"
)]
struct Args {
    /// Path to input CIM/G file or directory containing .g files
    #[arg(value_name = "INPUT")]
    input: PathBuf,

    /// Output SVG file or directory
    #[arg(short, long, value_name = "OUTPUT")]
    output: Option<PathBuf>,

    /// Additional directory containing definition files (Element.d, Color.d, Style.d)
    #[arg(short = 'd', long = "def-dir", value_name = "DIR")]
    def_dirs: Vec<PathBuf>,

    /// Inline symbol shapes directly into groups instead of using <use> tags
    #[arg(long)]
    inline_symbols: bool,

    /// Do not embed <defs> in the SVG output
    #[arg(long)]
    no_defs: bool,

    /// Do not include IEC 61970-453 metadata
    #[arg(long)]
    no_metadata: bool,

    /// Override output SVG width
    #[arg(long)]
    width: Option<f64>,

    /// Override output SVG height
    #[arg(long)]
    height: Option<f64>,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let log_level = if args.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    let config = ConverterConfig {
        embed_defs: !args.no_defs,
        inline_symbols: args.inline_symbols,
        include_metadata: !args.no_metadata,
        render_background: true,
        voltage_colors: true,
        custom_def_dirs: args.def_dirs,
        svg_width: args.width,
        svg_height: args.height,
    };

    if args.input.is_dir() {
        // Process directory
        let out_dir = args.output.unwrap_or_else(|| args.input.join("svg_output"));
        if let Err(e) = process_directory(&args.input, &out_dir, &config) {
            eprintln!("Error processing directory: {}", e);
            return ExitCode::FAILURE;
        }
    } else {
        // Process single file
        let out_path = args.output.unwrap_or_else(|| {
            let mut p = args.input.clone();
            p.set_extension("svg");
            p
        });

        if let Err(e) = convert_file(&args.input, &out_path, &config) {
            eprintln!("Conversion failed for {:?}: {}", args.input, e);
            return ExitCode::FAILURE;
        }
        println!("Successfully converted {:?} -> {:?}", args.input, out_path);
    }

    ExitCode::SUCCESS
}

fn process_directory(in_dir: &Path, out_dir: &Path, config: &ConverterConfig) -> Result<(), Box<dyn std::error::Error>> {
    let entries = std::fs::read_dir(in_dir)?;
    let mut count = 0;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext.eq_ignore_ascii_case("g") || ext.eq_ignore_ascii_case("xml") {
                    let file_stem = path.file_stem().unwrap();
                    let out_file = out_dir.join(format!("{}.svg", file_stem.to_string_lossy()));
                    println!("Converting: {:?} -> {:?}", path.file_name().unwrap(), out_file.file_name().unwrap());
                    if let Err(e) = convert_file(&path, &out_file, config) {
                        eprintln!("  Failed: {}", e);
                    } else {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Completed conversion of {} file(s).", count);
    Ok(())
}
