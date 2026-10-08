use crate::error::{G2SvgError, Result};
use crate::model::cimg::{CimGElement, DataList, GDocument, IncludeFiles, Layer};
use crate::parser::coords::{parse_background, parse_loc, parse_show, parse_viewbox};
use crate::parser::defs_parser::parse_shape_or_subelement;
use roxmltree::{Document, Node};
use std::collections::HashMap;

/// Parses a CIM/G XML string into a GDocument AST
pub fn parse_cimg(xml_content: &str) -> Result<GDocument> {
    let doc = Document::parse(xml_content).map_err(|e| G2SvgError::XmlParse {
        location: "CIM/G file root".to_string(),
        message: e.to_string(),
    })?;

    let root = doc.root_element();
    if !root.tag_name().name().eq_ignore_ascii_case("G") {
        return Err(G2SvgError::XmlParse {
            location: root.tag_name().name().to_string(),
            message: format!("Expected root element <G>, found <{}>", root.tag_name().name()),
        });
    }

    let mut g_doc = GDocument::default();

    // Parse root attributes
    g_doc.diagram_type = root.attribute("type").map(|s| s.to_string());
    if let Some(vb_str) = root.attribute("viewbox").or_else(|| root.attribute("viewBox")) {
        g_doc.viewbox = parse_viewbox(vb_str).ok();
    }
    if let Some(bg_str) = root.attribute("background") {
        g_doc.background = Some(parse_background(bg_str));
    }
    g_doc.app = root.attribute("app").map(|s| s.to_string());
    g_doc.context = root.attribute("context").map(|s| s.to_string());

    for attr in root.attributes() {
        let name = attr.name();
        if !["type", "viewbox", "viewBox", "background", "app", "context"].contains(&name) {
            g_doc.other_root_attrs.insert(name.to_string(), attr.value().to_string());
        }
    }

    // Parse children of <G>
    for child in root.children().filter(|n| n.is_element()) {
        let tag = child.tag_name().name();
        let tag_lower = tag.to_lowercase();

        match tag_lower.as_str() {
            "include" => {
                g_doc.includes.push(IncludeFiles {
                    element: child.attribute("element").map(|s| s.to_string()),
                    color: child.attribute("color").map(|s| s.to_string()),
                    style: child.attribute("style").map(|s| s.to_string()),
                    menu: child.attribute("menu").map(|s| s.to_string()),
                });
            }
            "layer" => {
                let name = child.attribute("name").unwrap_or("default").to_string();
                let visible = child.attribute("visible").map(|s| s.to_string());
                let mut layer_elements = Vec::new();
                for elem_node in child.children().filter(|n| n.is_element()) {
                    if let Some(elem) = parse_cimg_element(elem_node) {
                        layer_elements.push(elem);
                    }
                }
                g_doc.layers.push(Layer {
                    name,
                    visible,
                    elements: layer_elements,
                });
            }
            "datalist" => {
                g_doc.data_lists.push(DataList {
                    data_type: child.attribute("type").unwrap_or("").to_string(),
                    num: child.attribute("num").map(|s| s.to_string()),
                    start: child.attribute("start").map(|s| s.to_string()),
                    end: child.attribute("end").map(|s| s.to_string()),
                });
            }
            _ => {
                if let Some(elem) = parse_cimg_element(child) {
                    g_doc.elements.push(elem);
                }
            }
        }
    }

    Ok(g_doc)
}

/// Recursively parses any CIM/G element or container
pub fn parse_cimg_element(node: Node) -> Option<CimGElement> {
    let tag = node.tag_name().name();
    let tag_lower = tag.to_lowercase();

    // Check basic shapes first
    if ["rect", "circle", "ellipse", "line", "polyline", "polygon", "path", "text", "image"].contains(&tag_lower.as_str()) {
        return parse_shape_or_subelement(node);
    }

    // Check DText
    if tag_lower == "dtext" {
        let id = node.attribute("id").map(|s| s.to_string());
        let loc = node.attribute("loc").and_then(|s| parse_loc(s).ok());
        let data = node.attribute("data").map(|s| s.to_string());
        let show = node.attribute("show").map(parse_show);
        let anchor = node.attribute("A").map(|s| s.to_string());
        let text_content = node.attribute("value")
            .map(|s| s.to_string())
            .or_else(|| node.text().map(|s| s.to_string()));

        let mut attrs = HashMap::new();
        for attr in node.attributes() {
            attrs.insert(attr.name().to_string(), attr.value().to_string());
        }

        return Some(CimGElement::DynamicText {
            id,
            loc,
            data,
            show,
            anchor,
            text_content,
            attributes: attrs,
        });
    }

    // Check Link
    if tag_lower == "link" {
        let points = node.attribute("points").unwrap_or("").to_string();
        let connect = node.attribute("connect").map(|s| s.to_string());
        let show = node.attribute("show").map(parse_show);

        let mut attrs = HashMap::new();
        for attr in node.attributes() {
            attrs.insert(attr.name().to_string(), attr.value().to_string());
        }

        return Some(CimGElement::Link {
            points,
            connect,
            show,
            attributes: attrs,
        });
    }

    // Check containers
    let is_container = ["powerplant", "substation", "voltagelevel", "bay", "powergrid"].contains(&tag_lower.as_str());

    let id = node.attribute("id").map(|s| s.to_string());
    let loc = node.attribute("loc").and_then(|s| parse_loc(s).ok());
    let data = node.attribute("data").map(|s| s.to_string());
    let show = node.attribute("show").map(parse_show);
    let anchor = node.attribute("A").map(|s| s.to_string());

    let mut attrs = HashMap::new();
    for attr in node.attributes() {
        attrs.insert(attr.name().to_string(), attr.value().to_string());
    }

    let mut children = Vec::new();
    for child in node.children().filter(|n| n.is_element()) {
        if let Some(child_elem) = parse_cimg_element(child) {
            children.push(child_elem);
        }
    }

    if is_container {
        Some(CimGElement::Container {
            tag_name: tag.to_string(),
            id,
            loc,
            data,
            show,
            attributes: attrs,
            children,
        })
    } else {
        Some(CimGElement::Equipment {
            tag_name: tag.to_string(),
            id,
            loc,
            data,
            show,
            anchor,
            attributes: attrs,
            children,
        })
    }
}
