use std::path::PathBuf;

/// Options for CIM/G to CIM/SVG conversion
#[derive(Debug, Clone)]
pub struct ConverterConfig {
    /// Whether to embed <defs> in the SVG output
    pub embed_defs: bool,
    /// If true, inlines symbol geometry directly into <g> elements instead of referencing with <use>
    pub inline_symbols: bool,
    /// Whether to include IEC 61970-453 metadata block
    pub include_metadata: bool,
    /// Whether to render a background <rect> if background attribute is present
    pub render_background: bool,
    /// Whether to resolve voltage colors from show="Q,T,F,S" or volt attribute
    pub voltage_colors: bool,
    /// Custom directories to search for Element.d, Color.d, Style.d
    pub custom_def_dirs: Vec<PathBuf>,
    /// Optional overridden SVG width
    pub svg_width: Option<f64>,
    /// Optional overridden SVG height
    pub svg_height: Option<f64>,
}

impl Default for ConverterConfig {
    fn default() -> Self {
        Self {
            embed_defs: true,
            inline_symbols: false,
            include_metadata: true,
            render_background: true,
            voltage_colors: true,
            custom_def_dirs: Vec::new(),
            svg_width: None,
            svg_height: None,
        }
    }
}
