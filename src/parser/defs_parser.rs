use crate::error::{G2SvgError, Result};
use crate::model::cimg::{CimGElement, ShapeAttrs};
use crate::model::defs::{ColorEntry, DefinitionRegistry, ElementTemplate, StyleEntry};
use crate::parser::coords::{parse_box, parse_glue_points, parse_number_list};
use roxmltree::{Document, Node};
use std::collections::HashMap;

/// Parse a definition XML file or string into a DefinitionRegistry
pub fn parse_definitions(xml_content: &str, registry: &mut DefinitionRegistry) -> Result<()> {
    let doc = Document::parse(xml_content).map_err(|e| G2SvgError::XmlParse {
        location: "Definition file".to_string(),
        message: e.to_string(),
    })?;

    for node in doc.descendants() {
        if node.is_element() && node.tag_name().name().eq_ignore_ascii_case("defs") {
            let def_type = node.attribute("id").unwrap_or("").to_lowercase();
            if def_type.contains("element") {
                parse_element_defs(node, registry)?;
            } else if def_type.contains("color") {
                parse_color_defs(node, registry)?;
            } else if def_type.contains("style") {
                parse_style_defs(node, registry)?;
            } else {
                // If id is not explicitly specified, inspect children
                for child in node.children().filter(|n| n.is_element()) {
                    let child_tag = child.tag_name().name().to_lowercase();
                    if child_tag == "color" {
                        parse_color_node(child, registry);
                    } else if child_tag == "style" {
                        parse_style_node(child, registry);
                    } else {
                        parse_single_element_template(child, registry)?;
                    }
                }
            }
        }
    }

    Ok(())
}

fn parse_element_defs(node: Node, registry: &mut DefinitionRegistry) -> Result<()> {
    for child in node.children().filter(|n| n.is_element()) {
        parse_single_element_template(child, registry)?;
    }
    Ok(())
}

fn parse_single_element_template(node: Node, registry: &mut DefinitionRegistry) -> Result<()> {
    let tag_name = node.tag_name().name().to_string();
    let id = node.attribute("id").unwrap_or(&tag_name).to_string();
    let box_str = node.attribute("box").or_else(|| node.attribute("BOX"));
    let box_area = if let Some(b) = box_str {
        parse_box(b).ok()
    } else {
        None
    };

    let glue_str = node.attribute("glue").or_else(|| node.attribute("GLUE"));
    let glue_points = if let Some(g) = glue_str {
        parse_glue_points(g)
    } else {
        Vec::new()
    };

    let anchor = node.attribute("A").map(|s| s.to_string());

    let mut attributes = HashMap::new();
    for attr in node.attributes() {
        attributes.insert(attr.name().to_string(), attr.value().to_string());
    }

    let mut inner_elements = Vec::new();
    for child in node.children().filter(|n| n.is_element()) {
        if let Some(elem) = parse_shape_or_subelement(child) {
            inner_elements.push(elem);
        }
    }

    let tpl = ElementTemplate {
        tag_name,
        id,
        box_area,
        glue_points,
        anchor,
        attributes,
        inner_elements,
    };

    registry.register_template(tpl);
    Ok(())
}

fn parse_color_defs(node: Node, registry: &mut DefinitionRegistry) -> Result<()> {
    for child in node.children().filter(|n| n.is_element()) {
        parse_color_node(child, registry);
    }
    Ok(())
}

fn parse_color_node(node: Node, registry: &mut DefinitionRegistry) {
    let name = node.attribute("name").unwrap_or("").to_string();
    let voltage = node.attribute("voltage").unwrap_or("").to_string();
    let rgb_str = node.attribute("rgb").unwrap_or("");
    let nums = parse_number_list(rgb_str);
    let (r, g, b) = if nums.len() >= 3 {
        (nums[0] as u8, nums[1] as u8, nums[2] as u8)
    } else {
        (0, 0, 0)
    };

    let code = node.attribute("code").and_then(|c| c.parse::<u8>().ok());

    registry.register_color(ColorEntry {
        name,
        voltage,
        r,
        g,
        b,
        code,
    });
}

fn parse_style_defs(node: Node, registry: &mut DefinitionRegistry) -> Result<()> {
    for child in node.children().filter(|n| n.is_element()) {
        parse_style_node(child, registry);
    }
    Ok(())
}

fn parse_style_node(node: Node, registry: &mut DefinitionRegistry) {
    let name = node.attribute("name").unwrap_or("").to_string();
    let mut style = StyleEntry {
        name: name.clone(),
        line_style: node.attribute("ls").map(|s| s.to_string()),
        line_width: node.attribute("lw").and_then(|s| s.parse::<f64>().ok()),
        line_color: node.attribute("lc").map(|s| s.to_string()),
        fill_mode: node.attribute("fm").map(|s| s.to_string()),
        fill_color: node.attribute("fc").map(|s| s.to_string()),
        transform: node.attribute("tf").map(|s| s.to_string()),
        font_family: node.attribute("ff").map(|s| s.to_string()),
        font_size: node.attribute("fs").and_then(|s| s.parse::<f64>().ok()),
        raw_attrs: HashMap::new(),
    };

    for attr in node.attributes() {
        style.raw_attrs.insert(attr.name().to_string(), attr.value().to_string());
    }

    registry.register_style(style);
}

pub fn parse_shape_or_subelement(node: Node) -> Option<CimGElement> {
    let tag = node.tag_name().name();
    let tag_lower = tag.to_lowercase();

    let style = parse_shape_attrs(node);

    match tag_lower.as_str() {
        "rect" => {
            let x = node.attribute("x").and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let y = node.attribute("y").and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let w = node.attribute("w").or_else(|| node.attribute("width")).and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let h = node.attribute("h").or_else(|| node.attribute("height")).and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let rx = node.attribute("rx").and_then(|s| s.parse().ok());
            let ry = node.attribute("ry").and_then(|s| s.parse().ok());
            Some(CimGElement::Rect { x, y, w, h, rx, ry, style })
        }
        "circle" => {
            let cx = node.attribute("cx").and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let cy = node.attribute("cy").and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let r = node.attribute("r").and_then(|s| s.parse().ok()).unwrap_or(0.0);
            Some(CimGElement::Circle { cx, cy, r, style })
        }
        "ellipse" => {
            let cx = node.attribute("cx").and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let cy = node.attribute("cy").and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let rx = node.attribute("rx").and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let ry = node.attribute("ry").and_then(|s| s.parse().ok()).unwrap_or(0.0);
            Some(CimGElement::Ellipse { cx, cy, rx, ry, style })
        }
        "line" => {
            let x1 = node.attribute("x1").and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let y1 = node.attribute("y1").and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let x2 = node.attribute("x2").and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let y2 = node.attribute("y2").and_then(|s| s.parse().ok()).unwrap_or(0.0);
            Some(CimGElement::Line { x1, y1, x2, y2, style })
        }
        "polyline" => {
            let points = node.attribute("points").unwrap_or("").to_string();
            Some(CimGElement::Polyline { points, style })
        }
        "polygon" => {
            let points = node.attribute("points").unwrap_or("").to_string();
            Some(CimGElement::Polygon { points, style })
        }
        "path" => {
            let d = node.attribute("d").unwrap_or("").to_string();
            Some(CimGElement::Path { d, style })
        }
        "text" => {
            let x = node.attribute("x").and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let y = node.attribute("y").and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let val = node.attribute("value")
                .map(|s| s.to_string())
                .or_else(|| node.text().map(|s| s.to_string()))
                .unwrap_or_default();
            let w = node.attribute("w").and_then(|s| s.parse().ok());
            let h = node.attribute("h").and_then(|s| s.parse().ok());
            Some(CimGElement::Text { x, y, value: val, w, h, style })
        }
        "image" => {
            let x = node.attribute("x").and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let y = node.attribute("y").and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let w = node.attribute("w").or_else(|| node.attribute("width")).and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let h = node.attribute("h").or_else(|| node.attribute("height")).and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let href = node.attribute("xlink:href")
                .or_else(|| node.attribute("href"))
                .unwrap_or("")
                .to_string();
            Some(CimGElement::Image { x, y, w, h, href, style })
        }
        _ => {
            // Other elements
            let mut attrs = HashMap::new();
            for attr in node.attributes() {
                attrs.insert(attr.name().to_string(), attr.value().to_string());
            }
            let mut children = Vec::new();
            for child in node.children().filter(|n| n.is_element()) {
                if let Some(sub) = parse_shape_or_subelement(child) {
                    children.push(sub);
                }
            }
            Some(CimGElement::Generic {
                tag_name: tag.to_string(),
                attributes: attrs,
                children,
            })
        }
    }
}

pub fn parse_shape_attrs(node: Node) -> ShapeAttrs {
    let mut stroke = node.attribute("lc").map(|s| s.to_string());
    if stroke.is_none() {
        stroke = node.attribute("stroke").map(|s| s.to_string());
    }

    let mut stroke_width = node.attribute("lw").and_then(|s| s.parse().ok());
    if stroke_width.is_none() {
        stroke_width = node.attribute("stroke-width").and_then(|s| s.parse().ok());
    }

    let mut stroke_dasharray = node.attribute("ls").map(|s| s.to_string());
    if stroke_dasharray.is_none() {
        stroke_dasharray = node.attribute("stroke-dasharray").map(|s| s.to_string());
    }

    let mut fill = node.attribute("fc").map(|s| s.to_string());
    if fill.is_none() {
        fill = node.attribute("fill").map(|s| s.to_string());
    }

    let mut fill_rule = node.attribute("fm").map(|s| s.to_string());
    if fill_rule.is_none() {
        fill_rule = node.attribute("fill-rule").map(|s| s.to_string());
    }

    let mut transform = node.attribute("tf").map(|s| s.to_string());
    if transform.is_none() {
        transform = node.attribute("transform").map(|s| s.to_string());
    }

    let mut font_family = node.attribute("ff").map(|s| s.to_string());
    if font_family.is_none() {
        font_family = node.attribute("font-family").map(|s| s.to_string());
    }

    let mut font_size = node.attribute("fs").and_then(|s| s.parse().ok());
    if font_size.is_none() {
        font_size = node.attribute("font-size").and_then(|s| s.parse().ok());
    }

    let mut other_attrs = HashMap::new();
    for attr in node.attributes() {
        let n = attr.name();
        if !["x", "y", "w", "h", "width", "height", "rx", "ry", "cx", "cy", "r", "x1", "y1", "x2", "y2",
             "points", "d", "value", "lc", "lw", "ls", "fc", "fm", "tf", "fs", "ff",
             "stroke", "stroke-width", "stroke-dasharray", "fill", "fill-rule", "transform",
             "font-family", "font-size", "xlink:href", "href"].contains(&n) {
            other_attrs.insert(n.to_string(), attr.value().to_string());
        }
    }

    ShapeAttrs {
        stroke,
        stroke_width,
        stroke_dasharray,
        fill,
        fill_rule,
        transform,
        font_family,
        font_size,
        other_attrs,
    }
}
