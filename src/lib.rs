pub mod builtin;
pub mod converter;
pub mod error;
pub mod model;
pub mod parser;

use std::fs::File;
use std::io::Write;
use std::path::Path;

pub use converter::{convert_g_to_svg, ConverterConfig};
pub use error::{G2SvgError, Result};
pub use model::*;

/// Converts a CIM/G file into a CIM/SVG file
pub fn convert_file(input_path: &Path, output_path: &Path, config: &ConverterConfig) -> Result<()> {
    log::info!("Converting CIM/G file: {:?}", input_path);

    // Read and decode input file
    let content = parser::read_file_to_string(input_path)?;

    // Parse CIM/G document
    let g_doc = parser::parse_cimg(&content)?;

    // Initialize definition registry with standard IEC TS 61970-556 defaults
    let mut registry = builtin::create_default_registry();

    // Check for included definition files
    let parent_dir = input_path.parent().unwrap_or_else(|| Path::new("."));
    let mut search_dirs = vec![parent_dir.to_path_buf()];
    search_dirs.extend(config.custom_def_dirs.clone());

    for inc in &g_doc.includes {
        let def_files = [
            inc.element.as_deref(),
            inc.color.as_deref(),
            inc.style.as_deref(),
            inc.menu.as_deref(),
        ];

        for opt_file in def_files.into_iter().flatten() {
            let mut found = false;
            for dir in &search_dirs {
                let candidate = dir.join(opt_file);
                if candidate.exists() {
                    log::info!("Loading definition file: {:?}", candidate);
                    if let Ok(def_content) = parser::read_file_to_string(&candidate) {
                        if let Err(e) = parser::parse_definitions(&def_content, &mut registry) {
                            log::warn!("Warning while parsing {:?}: {}", candidate, e);
                        } else {
                            found = true;
                            break;
                        }
                    }
                }
            }
            if !found {
                log::debug!("Included definition file '{}' not found in search paths, using default standard definitions", opt_file);
            }
        }
    }

    // Convert document to SVG
    let svg_str = convert_g_to_svg(&g_doc, &registry, config)?;

    // Ensure parent directory exists for output
    if let Some(parent) = output_path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            std::fs::create_dir_all(parent).map_err(|e| G2SvgError::Io {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }
    }

    // Write SVG to output file
    let mut out_file = File::create(output_path).map_err(|e| G2SvgError::Io {
        path: output_path.to_path_buf(),
        source: e,
    })?;

    out_file
        .write_all(svg_str.as_bytes())
        .map_err(|e| G2SvgError::Io {
            path: output_path.to_path_buf(),
            source: e,
        })?;

    log::info!("Successfully written CIM/SVG to: {:?}", output_path);
    Ok(())
}

/// Converts a CIM/G XML string directly to SVG
pub fn convert_str(xml_content: &str, config: &ConverterConfig) -> Result<String> {
    let g_doc = parser::parse_cimg(xml_content)?;
    let registry = builtin::create_default_registry();
    convert_g_to_svg(&g_doc, &registry, config)
}
