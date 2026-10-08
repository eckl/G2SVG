use crate::model::defs::StyleEntry;

/// Default styles per IEC TS 61970-556 Annex C
pub fn get_default_styles() -> Vec<StyleEntry> {
    vec![
        StyleEntry {
            name: "default line style".to_string(),
            line_style: Some("1".to_string()),
            ..Default::default()
        },
        StyleEntry {
            name: "default line width".to_string(),
            line_width: Some(1.0),
            ..Default::default()
        },
        StyleEntry {
            name: "default line color".to_string(),
            line_color: Some("rgb(0, 0, 255)".to_string()),
            ..Default::default()
        },
        StyleEntry {
            name: "default fill mode".to_string(),
            fill_mode: Some("0".to_string()),
            ..Default::default()
        },
        StyleEntry {
            name: "default fill color".to_string(),
            fill_color: Some("rgb(0, 0, 255)".to_string()),
            ..Default::default()
        },
        StyleEntry {
            name: "default transform".to_string(),
            transform: Some("rotate(0)scale(1,1)".to_string()),
            ..Default::default()
        },
        StyleEntry {
            name: "default font family".to_string(),
            font_family: Some("Arial".to_string()),
            ..Default::default()
        },
        StyleEntry {
            name: "default font size".to_string(),
            font_size: Some(14.0),
            ..Default::default()
        },
    ]
}
