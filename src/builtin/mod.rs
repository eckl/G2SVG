pub mod colors;
pub mod elements;
pub mod styles;

use crate::model::defs::DefinitionRegistry;

/// Creates a DefinitionRegistry populated with standard IEC TS 61970-556 Annex A, B, and C definitions
pub fn create_default_registry() -> DefinitionRegistry {
    let mut reg = DefinitionRegistry::new();

    for color in colors::get_default_colors() {
        reg.register_color(color);
    }

    for style in styles::get_default_styles() {
        reg.register_style(style);
    }

    for template in elements::get_default_element_templates() {
        reg.register_template(template);
    }

    reg
}
