use std::collections::HashMap;
use super::cimg::{CimGElement, GluePoint, RectArea};

/// User-defined power equipment graphic element definition in `Element.d` (or `<defs id="element">`)
#[derive(Debug, Clone, PartialEq)]
pub struct ElementTemplate {
    pub tag_name: String,
    pub id: String,
    pub box_area: Option<RectArea>,
    pub glue_points: Vec<GluePoint>,
    pub anchor: Option<String>,
    pub attributes: HashMap<String, String>,
    pub inner_elements: Vec<CimGElement>,
}

/// Color definition from `Color.d` or Annex B
#[derive(Debug, Clone, PartialEq)]
pub struct ColorEntry {
    pub name: String,
    pub voltage: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub code: Option<u8>,
}

impl ColorEntry {
    pub fn new(name: impl Into<String>, voltage: impl Into<String>, r: u8, g: u8, b: u8, code: Option<u8>) -> Self {
        Self {
            name: name.into(),
            voltage: voltage.into(),
            r,
            g,
            b,
            code,
        }
    }

    pub fn to_rgb_string(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    pub fn to_hex_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

/// Diagram object style definition from `Style.d` or Annex C
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StyleEntry {
    pub name: String,
    pub line_style: Option<String>,
    pub line_width: Option<f64>,
    pub line_color: Option<String>,
    pub fill_mode: Option<String>,
    pub fill_color: Option<String>,
    pub transform: Option<String>,
    pub font_family: Option<String>,
    pub font_size: Option<f64>,
    pub raw_attrs: HashMap<String, String>,
}

/// Registry holding all resolved definitions
#[derive(Debug, Clone, Default)]
pub struct DefinitionRegistry {
    /// Element templates mapped by tag name (lowercase)
    pub templates_by_tag: HashMap<String, Vec<ElementTemplate>>,
    /// Element templates mapped by ID
    pub templates_by_id: HashMap<String, ElementTemplate>,
    /// Color entries mapped by voltage string (e.g. "500kV" -> ColorEntry)
    pub colors_by_voltage: HashMap<String, ColorEntry>,
    /// Color entries mapped by numeric voltage code (e.g. 5 -> 500kV)
    pub colors_by_code: HashMap<u8, ColorEntry>,
    /// Color entries mapped by color name (e.g. "red")
    pub colors_by_name: HashMap<String, ColorEntry>,
    /// Styles mapped by style name
    pub styles: HashMap<String, StyleEntry>,
}

impl DefinitionRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_template(&mut self, template: ElementTemplate) {
        let tag_key = template.tag_name.to_lowercase();
        self.templates_by_id.insert(template.id.clone(), template.clone());
        self.templates_by_tag
            .entry(tag_key)
            .or_default()
            .push(template);
    }

    pub fn find_template_for_element(&self, tag_name: &str, id_or_symbol: Option<&str>) -> Option<&ElementTemplate> {
        if let Some(id) = id_or_symbol {
            if let Some(tpl) = self.templates_by_id.get(id) {
                return Some(tpl);
            }
        }
        let tag_key = tag_name.to_lowercase();
        self.templates_by_tag
            .get(&tag_key)
            .and_then(|list| list.first())
    }

    pub fn register_color(&mut self, color: ColorEntry) {
        if let Some(code) = color.code {
            self.colors_by_code.insert(code, color.clone());
        }
        let volt_key = color.voltage.to_lowercase();
        self.colors_by_voltage.insert(volt_key, color.clone());
        let name_key = color.name.to_lowercase();
        self.colors_by_name.insert(name_key, color);
    }

    pub fn get_color_by_code(&self, code: u8) -> Option<&ColorEntry> {
        self.colors_by_code.get(&code)
    }

    pub fn get_color_by_voltage(&self, voltage: &str) -> Option<&ColorEntry> {
        let key = voltage.trim().to_lowercase();
        self.colors_by_voltage.get(&key)
    }

    pub fn get_color_by_name(&self, name: &str) -> Option<&ColorEntry> {
        let key = name.trim().to_lowercase();
        self.colors_by_name.get(&key)
    }

    pub fn register_style(&mut self, style: StyleEntry) {
        self.styles.insert(style.name.clone(), style);
    }
}
