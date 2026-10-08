use crate::model::cimg::{CimGElement, GluePoint, RectArea, ShapeAttrs};
use crate::model::defs::ElementTemplate;
use std::collections::HashMap;

/// Standard power equipment graphic element definitions per IEC TS 61970-556 Annex A
pub fn get_default_element_templates() -> Vec<ElementTemplate> {
    let mut templates = Vec::new();

    // 1. Breaker
    templates.push(ElementTemplate {
        tag_name: "Breaker".to_string(),
        id: "breaker0".to_string(),
        box_area: Some(RectArea::new(0.0, 0.0, 42.0, 18.0)),
        glue_points: vec![
            GluePoint { x: 18.0, y: 3.0 },
            GluePoint { x: 18.0, y: 15.0 },
        ],
        anchor: Some("F1".to_string()),
        attributes: HashMap::new(),
        inner_elements: vec![
            CimGElement::Rect {
                x: 4.0,
                y: 3.0,
                w: 12.0,
                h: 28.0,
                rx: None,
                ry: None,
                style: ShapeAttrs {
                    stroke: Some("currentColor".to_string()),
                    stroke_width: Some(1.0),
                    fill: Some("none".to_string()),
                    fill_rule: Some("0".to_string()),
                    ..Default::default()
                },
            },
        ],
    });

    // 2. Disconnector
    templates.push(ElementTemplate {
        tag_name: "Disconnector".to_string(),
        id: "disconnector0".to_string(),
        box_area: Some(RectArea::new(0.0, 0.0, 36.0, 18.0)),
        glue_points: vec![
            GluePoint { x: 6.0, y: 9.0 },
            GluePoint { x: 30.0, y: 9.0 },
        ],
        anchor: Some("F1".to_string()),
        attributes: HashMap::new(),
        inner_elements: vec![
            CimGElement::Circle {
                cx: 6.0,
                cy: 9.0,
                r: 2.0,
                style: ShapeAttrs {
                    stroke: Some("currentColor".to_string()),
                    fill: Some("none".to_string()),
                    ..Default::default()
                },
            },
            CimGElement::Line {
                x1: 6.0,
                y1: 9.0,
                x2: 30.0,
                y2: 9.0,
                style: ShapeAttrs {
                    stroke: Some("currentColor".to_string()),
                    stroke_width: Some(1.0),
                    ..Default::default()
                },
            },
            CimGElement::Circle {
                cx: 30.0,
                cy: 9.0,
                r: 2.0,
                style: ShapeAttrs {
                    stroke: Some("currentColor".to_string()),
                    fill: Some("none".to_string()),
                    ..Default::default()
                },
            },
        ],
    });

    // 3. GroundDisconnector
    templates.push(ElementTemplate {
        tag_name: "GroundDisconnector".to_string(),
        id: "grounddisconnector0".to_string(),
        box_area: Some(RectArea::new(0.0, 0.0, 40.0, 24.0)),
        glue_points: vec![GluePoint { x: 6.0, y: 12.0 }],
        anchor: Some("F1".to_string()),
        attributes: HashMap::new(),
        inner_elements: vec![
            CimGElement::Circle {
                cx: 6.0,
                cy: 12.0,
                r: 2.0,
                style: ShapeAttrs::default(),
            },
            CimGElement::Line { x1: 6.0, y1: 12.0, x2: 14.0, y2: 12.0, style: ShapeAttrs::default() },
            CimGElement::Line { x1: 14.0, y1: 9.0, x2: 14.0, y2: 15.0, style: ShapeAttrs::default() },
            CimGElement::Line { x1: 14.0, y1: 12.0, x2: 22.0, y2: 5.0, style: ShapeAttrs::default() },
            CimGElement::Line { x1: 22.0, y1: 12.0, x2: 30.0, y2: 12.0, style: ShapeAttrs::default() },
            CimGElement::Line { x1: 30.0, y1: 7.0, x2: 30.0, y2: 17.0, style: ShapeAttrs::default() },
            CimGElement::Line { x1: 32.0, y1: 8.0, x2: 32.0, y2: 16.0, style: ShapeAttrs::default() },
            CimGElement::Line { x1: 34.0, y1: 9.0, x2: 34.0, y2: 15.0, style: ShapeAttrs::default() },
        ],
    });

    // 4. Generator
    templates.push(ElementTemplate {
        tag_name: "Generator".to_string(),
        id: "generator0".to_string(),
        box_area: Some(RectArea::new(0.0, 0.0, 38.0, 40.0)),
        glue_points: vec![GluePoint { x: 19.0, y: 6.0 }],
        anchor: Some("F1".to_string()),
        attributes: HashMap::new(),
        inner_elements: vec![
            CimGElement::Circle {
                cx: 19.0,
                cy: 20.0,
                r: 14.0,
                style: ShapeAttrs {
                    stroke: Some("currentColor".to_string()),
                    fill: Some("none".to_string()),
                    stroke_width: Some(1.0),
                    ..Default::default()
                },
            },
            CimGElement::Path {
                d: "M5,20 a7,7 1 0 14,0".to_string(),
                style: ShapeAttrs {
                    stroke: Some("currentColor".to_string()),
                    fill: Some("none".to_string()),
                    ..Default::default()
                },
            },
            CimGElement::Path {
                d: "M33,20 a7,7 1 0 -14,0".to_string(),
                style: ShapeAttrs {
                    stroke: Some("currentColor".to_string()),
                    fill: Some("none".to_string()),
                    ..Default::default()
                },
            },
        ],
    });

    // 5. PowerTransformer2 (Two-winding transformer)
    templates.push(ElementTemplate {
        tag_name: "PowerTransformer2".to_string(),
        id: "powertransformer2_0".to_string(),
        box_area: Some(RectArea::new(0.0, 0.0, 62.0, 40.0)),
        glue_points: vec![
            GluePoint { x: 20.0, y: 4.0 },
            GluePoint { x: 20.0, y: 56.0 },
        ],
        anchor: Some("F1".to_string()),
        attributes: HashMap::new(),
        inner_elements: vec![
            CimGElement::Circle {
                cx: 20.0,
                cy: 20.0,
                r: 16.0,
                style: ShapeAttrs {
                    stroke: Some("currentColor".to_string()),
                    fill: Some("none".to_string()),
                    stroke_width: Some(1.0),
                    ..Default::default()
                },
            },
            CimGElement::Circle {
                cx: 20.0,
                cy: 40.0,
                r: 16.0,
                style: ShapeAttrs {
                    stroke: Some("currentColor".to_string()),
                    fill: Some("none".to_string()),
                    stroke_width: Some(1.0),
                    ..Default::default()
                },
            },
        ],
    });

    // PowerTransformer alias
    templates.push(ElementTemplate {
        tag_name: "PowerTransformer".to_string(),
        id: "powertransformer_0".to_string(),
        box_area: Some(RectArea::new(0.0, 0.0, 62.0, 40.0)),
        glue_points: vec![
            GluePoint { x: 20.0, y: 4.0 },
            GluePoint { x: 20.0, y: 56.0 },
        ],
        anchor: Some("F1".to_string()),
        attributes: HashMap::new(),
        inner_elements: vec![
            CimGElement::Circle {
                cx: 20.0,
                cy: 20.0,
                r: 16.0,
                style: ShapeAttrs {
                    stroke: Some("currentColor".to_string()),
                    fill: Some("none".to_string()),
                    stroke_width: Some(1.0),
                    ..Default::default()
                },
            },
            CimGElement::Circle {
                cx: 20.0,
                cy: 40.0,
                r: 16.0,
                style: ShapeAttrs {
                    stroke: Some("currentColor".to_string()),
                    fill: Some("none".to_string()),
                    stroke_width: Some(1.0),
                    ..Default::default()
                },
            },
        ],
    });

    // 6. PowerTransformer3 (Three-winding transformer)
    templates.push(ElementTemplate {
        tag_name: "PowerTransformer3".to_string(),
        id: "powertransformer3_0".to_string(),
        box_area: Some(RectArea::new(0.0, 0.0, 62.0, 40.0)),
        glue_points: vec![
            GluePoint { x: 40.0, y: 4.0 },
            GluePoint { x: 14.0, y: 29.0 },
            GluePoint { x: 66.0, y: 29.0 },
        ],
        anchor: Some("F1".to_string()),
        attributes: HashMap::new(),
        inner_elements: vec![
            CimGElement::Circle { cx: 40.0, cy: 20.0, r: 16.0, style: ShapeAttrs::default() },
            CimGElement::Circle { cx: 25.0, cy: 40.0, r: 16.0, style: ShapeAttrs::default() },
            CimGElement::Circle { cx: 55.0, cy: 40.0, r: 16.0, style: ShapeAttrs::default() },
        ],
    });

    // 7. ShuntCapacitor
    templates.push(ElementTemplate {
        tag_name: "ShuntCapacitor".to_string(),
        id: "shuntcapacitor0".to_string(),
        box_area: Some(RectArea::new(0.0, 0.0, 26.0, 40.0)),
        glue_points: vec![GluePoint { x: 13.0, y: 8.0 }],
        anchor: Some("F1".to_string()),
        attributes: HashMap::new(),
        inner_elements: vec![
            CimGElement::Line { x1: 13.0, y1: 5.0, x2: 13.0, y2: 16.0, style: ShapeAttrs::default() },
            CimGElement::Line { x1: 4.0, y1: 16.0, x2: 22.0, y2: 16.0, style: ShapeAttrs::default() },
            CimGElement::Line { x1: 4.0, y1: 22.0, x2: 22.0, y2: 22.0, style: ShapeAttrs::default() },
            CimGElement::Line { x1: 13.0, y1: 30.0, x2: 13.0, y2: 38.0, style: ShapeAttrs::default() },
        ],
    });

    // 8. ShuntReactor
    templates.push(ElementTemplate {
        tag_name: "ShuntReactor".to_string(),
        id: "shuntreactor0".to_string(),
        box_area: Some(RectArea::new(0.0, 0.0, 26.0, 40.0)),
        glue_points: vec![GluePoint { x: 13.0, y: 3.0 }],
        anchor: Some("F1".to_string()),
        attributes: HashMap::new(),
        inner_elements: vec![
            CimGElement::Path { d: "M13,10 a8,8 0 1 0 -5,8".to_string(), style: ShapeAttrs::default() },
            CimGElement::Line { x1: 13.0, y1: 3.0, x2: 13.0, y2: 10.0, style: ShapeAttrs::default() },
            CimGElement::Line { x1: 5.0, y1: 18.0, x2: 13.0, y2: 18.0, style: ShapeAttrs::default() },
            CimGElement::Line { x1: 13.0, y1: 18.0, x2: 13.0, y2: 37.0, style: ShapeAttrs::default() },
            CimGElement::Line { x1: 4.0, y1: 37.0, x2: 22.0, y2: 37.0, style: ShapeAttrs::default() },
            CimGElement::Line { x1: 6.0, y1: 39.0, x2: 20.0, y2: 39.0, style: ShapeAttrs::default() },
            CimGElement::Line { x1: 8.0, y1: 41.0, x2: 18.0, y2: 41.0, style: ShapeAttrs::default() },
        ],
    });

    // 9. BusbarSection
    templates.push(ElementTemplate {
        tag_name: "BusbarSection".to_string(),
        id: "busbarsection0".to_string(),
        box_area: Some(RectArea::new(0.0, 0.0, 100.0, 4.0)),
        glue_points: vec![],
        anchor: None,
        attributes: HashMap::new(),
        inner_elements: vec![
            CimGElement::Line {
                x1: 0.0,
                y1: 2.0,
                x2: 100.0,
                y2: 2.0,
                style: ShapeAttrs {
                    stroke: Some("currentColor".to_string()),
                    stroke_width: Some(4.0),
                    ..Default::default()
                },
            },
        ],
    });

    // Bus alias
    templates.push(ElementTemplate {
        tag_name: "Bus".to_string(),
        id: "bus0".to_string(),
        box_area: Some(RectArea::new(0.0, 0.0, 100.0, 4.0)),
        glue_points: vec![],
        anchor: None,
        attributes: HashMap::new(),
        inner_elements: vec![
            CimGElement::Line {
                x1: 0.0,
                y1: 2.0,
                x2: 100.0,
                y2: 2.0,
                style: ShapeAttrs {
                    stroke: Some("currentColor".to_string()),
                    stroke_width: Some(4.0),
                    ..Default::default()
                },
            },
        ],
    });

    // 10. ACLineSegment
    templates.push(ElementTemplate {
        tag_name: "ACLineSegment".to_string(),
        id: "aclinesegment0".to_string(),
        box_area: Some(RectArea::new(0.0, 0.0, 100.0, 20.0)),
        glue_points: vec![],
        anchor: None,
        attributes: HashMap::new(),
        inner_elements: vec![
            CimGElement::Line {
                x1: 0.0,
                y1: 10.0,
                x2: 100.0,
                y2: 10.0,
                style: ShapeAttrs {
                    stroke: Some("currentColor".to_string()),
                    stroke_width: Some(1.5),
                    ..Default::default()
                },
            },
        ],
    });

    // 11. SubstationMark
    templates.push(ElementTemplate {
        tag_name: "SubstationMark".to_string(),
        id: "substationmark0".to_string(),
        box_area: Some(RectArea::new(0.0, 0.0, 25.0, 25.0)),
        glue_points: vec![],
        anchor: None,
        attributes: HashMap::new(),
        inner_elements: vec![
            CimGElement::Circle {
                cx: 12.0,
                cy: 12.0,
                r: 10.0,
                style: ShapeAttrs {
                    stroke: Some("currentColor".to_string()),
                    fill: Some("none".to_string()),
                    stroke_width: Some(1.5),
                    ..Default::default()
                },
            },
        ],
    });

    // 12. PowerPlantMark
    templates.push(ElementTemplate {
        tag_name: "PowerPlantMark".to_string(),
        id: "powerplantmark0".to_string(),
        box_area: Some(RectArea::new(0.0, 0.0, 60.0, 40.0)),
        glue_points: vec![],
        anchor: None,
        attributes: HashMap::new(),
        inner_elements: vec![
            CimGElement::Rect {
                x: 2.0,
                y: 2.0,
                w: 56.0,
                h: 36.0,
                rx: None,
                ry: None,
                style: ShapeAttrs {
                    stroke: Some("currentColor".to_string()),
                    fill: Some("none".to_string()),
                    ..Default::default()
                },
            },
            CimGElement::Path { d: "M5,20 a7,7 1 0 14,0".to_string(), style: ShapeAttrs::default() },
            CimGElement::Path { d: "M33,20 a7,7 1 0 -14,0".to_string(), style: ShapeAttrs::default() },
        ],
    });

    templates
}
