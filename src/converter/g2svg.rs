use crate::converter::config::ConverterConfig;
use crate::error::Result;
use crate::model::cimg::{CimGElement, GDocument, ShapeAttrs, ShowStatus};
use crate::model::defs::DefinitionRegistry;

/// Convert a GDocument to an SVG XML string
pub fn convert_g_to_svg(
    doc: &GDocument,
    registry: &DefinitionRegistry,
    config: &ConverterConfig,
) -> Result<String> {
    let mut out = String::with_capacity(16 * 1024);

    // 1. XML declaration
    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");

    // 2. Compute viewBox and dimensions
    let (vb_x, vb_y, vb_w, vb_h) = if let Some(vb) = doc.viewbox {
        (vb.x, vb.y, vb.w, vb.h)
    } else {
        (0.0, 0.0, 1920.0, 1080.0)
    };

    let width_str = config
        .svg_width
        .map(|w| w.to_string())
        .unwrap_or_else(|| vb_w.to_string());
    let height_str = config
        .svg_height
        .map(|h| h.to_string())
        .unwrap_or_else(|| vb_h.to_string());

    // 3. SVG root element with namespaces
    out.push_str("<svg\n");
    out.push_str("    xmlns=\"http://www.w3.org/2000/svg\"\n");
    out.push_str("    xmlns:xlink=\"http://www.w3.org/1999/xlink\"\n");
    out.push_str("    xmlns:cim=\"http://iec.ch/TC57/2014/CIM-schema-cim16#\"\n");
    out.push_str("    xmlns:cims=\"http://iec.ch/TC57/1999/rdf-schema-extensions-1999#\"\n");
    out.push_str("    xmlns:cimg=\"http://iec.ch/TC57/61970-556#\"\n");
    out.push_str(&format!("    width=\"{}\"\n", width_str));
    out.push_str(&format!("    height=\"{}\"\n", height_str));
    out.push_str(&format!(
        "    viewBox=\"{} {} {} {}\"\n",
        vb_x, vb_y, vb_w, vb_h
    ));

    if let Some(t) = &doc.diagram_type {
        out.push_str(&format!("    cimg:type=\"{}\"\n", escape_xml(t)));
    }
    if let Some(app) = &doc.app {
        out.push_str(&format!("    cimg:app=\"{}\"\n", escape_xml(app)));
    }
    if let Some(ctx) = &doc.context {
        out.push_str(&format!("    cimg:context=\"{}\"\n", escape_xml(ctx)));
    }
    out.push_str(">\n");

    // 4. IEC 61970-453 Diagram Layout Metadata
    if config.include_metadata {
        write_metadata(&mut out, doc);
    }

    // 5. Defs (Symbols, Styles, Colors)
    if config.embed_defs {
        write_defs(&mut out, registry);
    }

    // 6. Background
    if config.render_background {
        if let Some(bg) = &doc.background {
            out.push_str(&format!(
                "  <rect class=\"cimg-background\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n",
                vb_x, vb_y, vb_w, vb_h, escape_xml(bg)
            ));
        }
    }

    // 7. Render Layers if any
    for layer in &doc.layers {
        out.push_str(&format!(
            "  <g id=\"layer_{}\" class=\"cimg-layer\" cimg:layerName=\"{}\"",
            escape_xml(&layer.name),
            escape_xml(&layer.name)
        ));
        if let Some(vis) = &layer.visible {
            out.push_str(&format!(" cimg:visible=\"{}\"", escape_xml(vis)));
        }
        out.push_str(">\n");

        for elem in &layer.elements {
            render_element(&mut out, elem, registry, config, 2, None)?;
        }

        out.push_str("  </g>\n");
    }

    // 8. Render Top-level Elements & Containers
    for elem in &doc.elements {
        render_element(&mut out, elem, registry, config, 1, None)?;
    }

    // 9. Close SVG
    out.push_str("</svg>\n");

    Ok(out)
}

fn write_metadata(out: &mut String, doc: &GDocument) {
    out.push_str("  <metadata>\n");
    out.push_str("    <cimg:Diagram");
    if let Some(t) = &doc.diagram_type {
        out.push_str(&format!(" cimg:diagramType=\"{}\"", escape_xml(t)));
    }
    if let Some(app) = &doc.app {
        out.push_str(&format!(" cimg:application=\"{}\"", escape_xml(app)));
    }
    if let Some(ctx) = &doc.context {
        out.push_str(&format!(" cimg:context=\"{}\"", escape_xml(ctx)));
    }
    out.push_str(">\n");

    for dl in &doc.data_lists {
        out.push_str(&format!(
            "      <cimg:DataList cimg:type=\"{}\"",
            escape_xml(&dl.data_type)
        ));
        if let Some(num) = &dl.num {
            out.push_str(&format!(" cimg:num=\"{}\"", escape_xml(num)));
        }
        if let Some(start) = &dl.start {
            out.push_str(&format!(" cimg:start=\"{}\"", escape_xml(start)));
        }
        if let Some(end) = &dl.end {
            out.push_str(&format!(" cimg:end=\"{}\"", escape_xml(end)));
        }
        out.push_str(" />\n");
    }

    out.push_str("    </cimg:Diagram>\n");
    out.push_str("  </metadata>\n");
}

fn write_defs(out: &mut String, registry: &DefinitionRegistry) {
    out.push_str("  <defs>\n");

    // CSS Style Block based on Color.d and Style.d
    out.push_str("    <style type=\"text/css\">\n");
    out.push_str("      <![CDATA[\n");
    out.push_str("        .cimg-equipment { stroke: currentColor; fill: none; }\n");
    out.push_str("        .cimg-busbar { stroke-width: 4px; }\n");
    out.push_str("        .cimg-link { stroke: currentColor; stroke-width: 1.5px; fill: none; }\n");
    out.push_str("        .cimg-dtext { font-family: Arial, sans-serif; font-size: 14px; fill: currentColor; }\n");

    // Voltage level classes from Annex B / Color.d
    for (volt, color) in &registry.colors_by_voltage {
        let clean_volt = volt.replace(' ', "");
        out.push_str(&format!(
            "        .volt-{} {{ stroke: {}; fill: none; }}\n",
            clean_volt,
            color.to_rgb_string()
        ));
        out.push_str(&format!(
            "        .volt-{}-fill {{ fill: {}; }}\n",
            clean_volt,
            color.to_rgb_string()
        ));
    }

    // Default styles from Annex C
    for (name, style) in &registry.styles {
        let class_name = name.replace(' ', "-").to_lowercase();
        out.push_str(&format!("        .style-{} {{", class_name));
        if let Some(ls) = &style.line_style {
            out.push_str(&format!(" stroke-dasharray: {};", ls));
        }
        if let Some(lw) = style.line_width {
            out.push_str(&format!(" stroke-width: {}px;", lw));
        }
        if let Some(lc) = &style.line_color {
            out.push_str(&format!(" stroke: {};", lc));
        }
        if let Some(fc) = &style.fill_color {
            out.push_str(&format!(" fill: {};", fc));
        }
        if let Some(fs) = style.font_size {
            out.push_str(&format!(" font-size: {}px;", fs));
        }
        if let Some(ff) = &style.font_family {
            out.push_str(&format!(" font-family: {};", ff));
        }
        out.push_str(" }\n");
    }

    out.push_str("      ]]>\n");
    out.push_str("    </style>\n");

    // Symbol definitions for power system elements
    for tpl in registry.templates_by_id.values() {
        out.push_str(&format!("    <g id=\"{}\" class=\"symbol-{}\"", escape_xml(&tpl.id), escape_xml(&tpl.tag_name.to_lowercase())));
        if let Some(b) = tpl.box_area {
            out.push_str(&format!(" data-box=\"{} {} {} {}\"", b.x, b.y, b.w, b.h));
        }
        out.push_str(">\n");

        for inner in &tpl.inner_elements {
            render_shape_element(out, inner, 3);
        }

        out.push_str("    </g>\n");
    }

    out.push_str("  </defs>\n");
}

fn render_element(
    out: &mut String,
    elem: &CimGElement,
    registry: &DefinitionRegistry,
    config: &ConverterConfig,
    indent_level: usize,
    inherited_voltage: Option<&str>,
) -> Result<()> {
    let indent = "  ".repeat(indent_level);

    match elem {
        CimGElement::Container {
            tag_name,
            id,
            loc,
            data,
            show,
            attributes,
            children,
        } => {
            let container_volt = attributes
                .get("volt")
                .or_else(|| attributes.get("voltage"))
                .map(|s| s.as_str())
                .or(inherited_voltage);

            out.push_str(&format!("{}<g class=\"cimg-container cimg-{}\"", indent, escape_xml(&tag_name.to_lowercase())));
            if let Some(i) = id {
                out.push_str(&format!(" id=\"{}\"", escape_xml(i)));
            }
            if let Some(l) = loc {
                if l.x != 0.0 || l.y != 0.0 {
                    out.push_str(&format!(" transform=\"translate({}, {})\"", l.x, l.y));
                }
            }
            if let Some(d) = data {
                out.push_str(&format!(" cims:data=\"{}\"", escape_xml(d)));
            }
            if let Some(s) = show {
                out.push_str(&format!(" cims:show=\"{}\"", escape_xml(&s.raw)));
            }
            for (k, v) in attributes {
                if !["id", "loc", "data", "show", "volt", "voltage"].contains(&k.as_str()) {
                    out.push_str(&format!(" cimg:{}=\"{}\"", escape_xml(k), escape_xml(v)));
                }
            }
            out.push_str(">\n");

            for child in children {
                render_element(out, child, registry, config, indent_level + 1, container_volt)?;
            }

            out.push_str(&format!("{}</g>\n", indent));
        }

        CimGElement::Equipment {
            tag_name,
            id,
            loc,
            data,
            show,
            anchor,
            attributes,
            children,
        } => {
            let elem_volt = attributes
                .get("volt")
                .or_else(|| attributes.get("voltage"))
                .map(|s| s.as_str())
                .or(inherited_voltage);

            let (color_stroke, color_class) = resolve_element_color(show, elem_volt, registry);

            let tpl_opt = registry.find_template_for_element(tag_name, None);

            // Compute transform matrix or translation
            let loc_val = (*loc).unwrap_or_default();
            let mut transform_str = format!("translate({}, {})", loc_val.x, loc_val.y);

            let mut scale_x: f64 = 1.0;
            let mut scale_y: f64 = 1.0;
            let mut rotation_deg: f64 = 0.0;

            if let Some(tpl) = tpl_opt {
                if let Some(box_area) = tpl.box_area {
                    if let (Some(w), Some(h)) = (loc_val.w, loc_val.h) {
                        let is_busbar = tag_name.eq_ignore_ascii_case("busbarsection") || tag_name.eq_ignore_ascii_case("bus");
                        if is_busbar {
                            if h <= 1.0 && w > 1.0 {
                                // Horizontal busbar of length w
                                if box_area.w > 0.0 {
                                    scale_x = w / box_area.w;
                                }
                                scale_y = 1.0;
                            } else if w <= 1.0 && h > 1.0 {
                                // Vertical busbar of length h
                                if box_area.w > 0.0 {
                                    scale_x = h / box_area.w;
                                }
                                scale_y = 1.0;
                                rotation_deg = 90.0;
                            } else {
                                if box_area.w > 0.0 && w > 0.0 { scale_x = w / box_area.w; }
                                if box_area.h > 0.0 && h > 0.0 { scale_y = h / box_area.h; }
                            }
                        } else {
                            if box_area.w > 0.0 && w > 0.0 && (box_area.w - w).abs() > 0.01 {
                                scale_x = w / box_area.w;
                            }
                            if box_area.h > 0.0 && h > 0.0 && (box_area.h - h).abs() > 0.01 {
                                scale_y = h / box_area.h;
                            }
                        }
                    }
                }
            }
            if (scale_x - 1.0).abs() > 0.001 || (scale_y - 1.0).abs() > 0.001 {
                transform_str.push_str(&format!(" scale({:.4}, {:.4})", scale_x, scale_y));
            }
            if rotation_deg.abs() > 0.01 {
                transform_str.push_str(&format!(" rotate({})", rotation_deg));
            }

            out.push_str(&format!("{}<g class=\"cimg-equipment cimg-{}", indent, escape_xml(&tag_name.to_lowercase())));
            if let Some(cls) = &color_class {
                out.push_str(&format!(" {}", cls));
            }
            out.push_str("\"");

            if let Some(i) = id {
                out.push_str(&format!(" id=\"{}\"", escape_xml(i)));
            }
            out.push_str(&format!(" transform=\"{}\"", transform_str));

            if let Some(d) = data {
                out.push_str(&format!(" cims:data=\"{}\"", escape_xml(d)));
            }
            if let Some(s) = show {
                out.push_str(&format!(" cims:show=\"{}\"", escape_xml(&s.raw)));
            }
            if let Some(a) = anchor {
                out.push_str(&format!(" cims:A=\"{}\"", escape_xml(a)));
            }
            if let Some(col) = &color_stroke {
                out.push_str(&format!(" stroke=\"{}\"", col));
            }

            for (k, v) in attributes {
                if !["id", "loc", "data", "show", "A", "volt", "voltage"].contains(&k.as_str()) {
                    out.push_str(&format!(" cimg:{}=\"{}\"", escape_xml(k), escape_xml(v)));
                }
            }
            out.push_str(">\n");

            // Render equipment representation
            if config.inline_symbols || tpl_opt.is_none() {
                // If inlining or template not in defs, render inner elements
                if let Some(tpl) = tpl_opt {
                    for inner in &tpl.inner_elements {
                        render_shape_element(out, inner, indent_level + 1);
                    }
                } else {
                    // Default fallback: draw small representation or label
                    out.push_str(&format!(
                        "{}  <rect width=\"20\" height=\"20\" fill=\"none\" stroke=\"currentColor\" />\n",
                        indent
                    ));
                }
            } else {
                let tpl = tpl_opt.unwrap();
                out.push_str(&format!("{}  <use xlink:href=\"#{}\"", indent, escape_xml(&tpl.id)));
                if let Some(col) = &color_stroke {
                    out.push_str(&format!(" stroke=\"{}\"", col));
                }
                out.push_str(" />\n");
            }

            // Also render explicit children if any
            for child in children {
                render_element(out, child, registry, config, indent_level + 1, elem_volt)?;
            }

            out.push_str(&format!("{}</g>\n", indent));
        }

        CimGElement::DynamicText {
            id,
            loc,
            data,
            show,
            anchor,
            text_content,
            attributes,
        } => {
            let loc_val = (*loc).unwrap_or_default();
            out.push_str(&format!("{}<text class=\"cimg-dtext\"", indent));
            if let Some(i) = id {
                out.push_str(&format!(" id=\"{}\"", escape_xml(i)));
            }
            out.push_str(&format!(" x=\"{}\" y=\"{}\"", loc_val.x, loc_val.y));
            if let Some(d) = data {
                out.push_str(&format!(" cims:data=\"{}\"", escape_xml(d)));
            }
            if let Some(s) = show {
                out.push_str(&format!(" cims:show=\"{}\"", escape_xml(&s.raw)));
            }
            if let Some(a) = anchor {
                out.push_str(&format!(" cims:A=\"{}\"", escape_xml(a)));
            }
            for (k, v) in attributes {
                if !["id", "loc", "data", "show", "A", "value"].contains(&k.as_str()) {
                    out.push_str(&format!(" cimg:{}=\"{}\"", escape_xml(k), escape_xml(v)));
                }
            }
            out.push_str(">");
            let display_text = text_content
                .as_deref()
                .or_else(|| data.as_deref())
                .unwrap_or("0.0");
            out.push_str(&escape_xml(display_text));
            out.push_str("</text>\n");
        }

        CimGElement::Link {
            points,
            connect,
            show,
            attributes,
        } => {
            out.push_str(&format!("{}<polyline class=\"cimg-link\" points=\"{}\"", indent, escape_xml(points)));
            if let Some(c) = connect {
                out.push_str(&format!(" cims:connect=\"{}\"", escape_xml(c)));
            }
            if let Some(s) = show {
                out.push_str(&format!(" cims:show=\"{}\"", escape_xml(&s.raw)));
            }
            for (k, v) in attributes {
                if !["points", "connect", "show"].contains(&k.as_str()) {
                    out.push_str(&format!(" cimg:{}=\"{}\"", escape_xml(k), escape_xml(v)));
                }
            }
            out.push_str(" />\n");
        }

        // Basic Shapes
        _ => {
            render_shape_element(out, elem, indent_level);
        }
    }

    Ok(())
}

fn render_shape_element(out: &mut String, elem: &CimGElement, indent_level: usize) {
    let indent = "  ".repeat(indent_level);

    match elem {
        CimGElement::Rect {
            x,
            y,
            w,
            h,
            rx,
            ry,
            style,
        } => {
            out.push_str(&format!("{}<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"", indent, x, y, w, h));
            if let Some(rx_val) = rx {
                out.push_str(&format!(" rx=\"{}\"", rx_val));
            }
            if let Some(ry_val) = ry {
                out.push_str(&format!(" ry=\"{}\"", ry_val));
            }
            write_shape_style_attrs(out, style);
            out.push_str(" />\n");
        }

        CimGElement::Circle { cx, cy, r, style } => {
            out.push_str(&format!("{}<circle cx=\"{}\" cy=\"{}\" r=\"{}\"", indent, cx, cy, r));
            write_shape_style_attrs(out, style);
            out.push_str(" />\n");
        }

        CimGElement::Ellipse { cx, cy, rx, ry, style } => {
            out.push_str(&format!("{}<ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\"", indent, cx, cy, rx, ry));
            write_shape_style_attrs(out, style);
            out.push_str(" />\n");
        }

        CimGElement::Line { x1, y1, x2, y2, style } => {
            out.push_str(&format!("{}<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"", indent, x1, y1, x2, y2));
            write_shape_style_attrs(out, style);
            out.push_str(" />\n");
        }

        CimGElement::Polyline { points, style } => {
            out.push_str(&format!("{}<polyline points=\"{}\"", indent, escape_xml(points)));
            write_shape_style_attrs(out, style);
            out.push_str(" />\n");
        }

        CimGElement::Polygon { points, style } => {
            out.push_str(&format!("{}<polygon points=\"{}\"", indent, escape_xml(points)));
            write_shape_style_attrs(out, style);
            out.push_str(" />\n");
        }

        CimGElement::Path { d, style } => {
            out.push_str(&format!("{}<path d=\"{}\"", indent, escape_xml(d)));
            write_shape_style_attrs(out, style);
            out.push_str(" />\n");
        }

        CimGElement::Text {
            x,
            y,
            value,
            w: _,
            h: _,
            style,
        } => {
            out.push_str(&format!("{}<text x=\"{}\" y=\"{}\"", indent, x, y));
            write_shape_style_attrs(out, style);
            out.push_str(">");
            out.push_str(&escape_xml(value));
            out.push_str("</text>\n");
        }

        CimGElement::Image {
            x,
            y,
            w,
            h,
            href,
            style,
        } => {
            out.push_str(&format!(
                "{}<image x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" xlink:href=\"{}\"",
                indent, x, y, w, h, escape_xml(href)
            ));
            write_shape_style_attrs(out, style);
            out.push_str(" />\n");
        }

        CimGElement::Generic {
            tag_name,
            attributes,
            children,
        } => {
            out.push_str(&format!("{}<{}", indent, escape_xml(tag_name)));
            for (k, v) in attributes {
                out.push_str(&format!(" {}=\"{}\"", escape_xml(k), escape_xml(v)));
            }
            if children.is_empty() {
                out.push_str(" />\n");
            } else {
                out.push_str(">\n");
                for child in children {
                    render_shape_element(out, child, indent_level + 1);
                }
                out.push_str(&format!("{}</{}>\n", indent, escape_xml(tag_name)));
            }
        }

        _ => {}
    }
}

fn write_shape_style_attrs(out: &mut String, style: &ShapeAttrs) {
    if let Some(s) = &style.stroke {
        out.push_str(&format!(" stroke=\"{}\"", escape_xml(s)));
    }
    if let Some(w) = style.stroke_width {
        out.push_str(&format!(" stroke-width=\"{}\"", w));
    }
    if let Some(ls) = &style.stroke_dasharray {
        out.push_str(&format!(" stroke-dasharray=\"{}\"", escape_xml(ls)));
    }
    if let Some(f) = &style.fill {
        out.push_str(&format!(" fill=\"{}\"", escape_xml(f)));
    }
    if let Some(fm) = &style.fill_rule {
        let rule = if fm == "0" { "evenodd" } else { "nonzero" };
        out.push_str(&format!(" fill-rule=\"{}\"", rule));
    }
    if let Some(tf) = &style.transform {
        out.push_str(&format!(" transform=\"{}\"", escape_xml(tf)));
    }
    if let Some(fs) = style.font_size {
        out.push_str(&format!(" font-size=\"{}\"", fs));
    }
    if let Some(ff) = &style.font_family {
        out.push_str(&format!(" font-family=\"{}\"", escape_xml(ff)));
    }
    for (k, v) in &style.other_attrs {
        out.push_str(&format!(" {}=\"{}\"", escape_xml(k), escape_xml(v)));
    }
}

fn resolve_element_color(
    show: &Option<ShowStatus>,
    volt: Option<&str>,
    registry: &DefinitionRegistry,
) -> (Option<String>, Option<String>) {
    // 1. Try topology status T code
    if let Some(s) = show {
        if let Some(t_code) = s.t {
            if let Some(color_entry) = registry.get_color_by_code(t_code) {
                let clean_volt = color_entry.voltage.replace(' ', "");
                return (
                    Some(color_entry.to_rgb_string()),
                    Some(format!("volt-{}", clean_volt)),
                );
            }
        }
    }

    // 2. Try voltage level string
    if let Some(v) = volt {
        if let Some(color_entry) = registry.get_color_by_voltage(v) {
            let clean_volt = color_entry.voltage.replace(' ', "");
            return (
                Some(color_entry.to_rgb_string()),
                Some(format!("volt-{}", clean_volt)),
            );
        }
    }

    (None, None)
}

fn escape_xml(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}
