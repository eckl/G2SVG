use std::collections::HashMap;

/// Parsed rectangular coordinates/area (x, y, w, h)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RectArea {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl RectArea {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }
}

/// Parsed element location from `loc="x,y w,h"` or `loc="x,y,w,h"` or `loc="x,y"`
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ElementLoc {
    pub x: f64,
    pub y: f64,
    pub w: Option<f64>,
    pub h: Option<f64>,
}

/// Status and display style from `show="Q,T,F,S"` per IEC TS 61970-556 Clause 8.3
/// Q: Quality of data (IEC 61970-301 / IEC 61850)
/// T: Topological status / voltage level color code (refer to Annex B)
/// F: Flashing flag (0 = no flash, 1 = 1s, etc.)
/// S: Shape change marker (0 = normal, etc.)
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShowStatus {
    pub raw: String,
    pub q: Option<u8>,
    pub t: Option<u8>,
    pub f: Option<u8>,
    pub s: Option<u8>,
}

/// A glue point coordinate (terminal connectivity point)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GluePoint {
    pub x: f64,
    pub y: f64,
}

/// An Include tag referencing definition files
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IncludeFiles {
    pub element: Option<String>,
    pub color: Option<String>,
    pub style: Option<String>,
    pub menu: Option<String>,
}

/// A DataList tag for dynamic data refreshing
#[derive(Debug, Clone, PartialEq)]
pub struct DataList {
    pub data_type: String,
    pub num: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
}

/// A layer definition in CIM/G
#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    pub name: String,
    pub visible: Option<String>,
    pub elements: Vec<CimGElement>,
}

/// Basic shape graphic element attributes per IEC TS 61970-556 Table 1 & Table 2
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShapeAttrs {
    pub stroke: Option<String>,      // lc -> stroke
    pub stroke_width: Option<f64>,   // lw -> stroke-width
    pub stroke_dasharray: Option<String>, // ls -> stroke-dasharray
    pub fill: Option<String>,        // fc -> fill
    pub fill_rule: Option<String>,   // fm -> fill-rule ("0" -> evenodd or nonzero)
    pub transform: Option<String>,   // tf -> transform
    pub font_family: Option<String>, // ff -> font-family
    pub font_size: Option<f64>,      // fs -> font-size
    pub other_attrs: HashMap<String, String>,
}

/// Represents any CIM/G graphic element or container
#[derive(Debug, Clone, PartialEq)]
pub enum CimGElement {
    /// Container elements: PowerPlant, Substation, VoltageLevel, Bay, PowerGrid
    Container {
        tag_name: String,
        id: Option<String>,
        loc: Option<ElementLoc>,
        data: Option<String>,
        show: Option<ShowStatus>,
        attributes: HashMap<String, String>,
        children: Vec<CimGElement>,
    },

    /// Power system equipment instance (Breaker, Disconnector, BusbarSection, etc.)
    Equipment {
        tag_name: String,
        id: Option<String>,
        loc: Option<ElementLoc>,
        data: Option<String>,
        show: Option<ShowStatus>,
        anchor: Option<String>,
        attributes: HashMap<String, String>,
        children: Vec<CimGElement>,
    },

    /// Dynamic Text element (<DText ...>)
    DynamicText {
        id: Option<String>,
        loc: Option<ElementLoc>,
        data: Option<String>,
        show: Option<ShowStatus>,
        anchor: Option<String>,
        text_content: Option<String>,
        attributes: HashMap<String, String>,
    },

    /// Link element (<Link points="..." connect="..." show="..." />)
    Link {
        points: String,
        connect: Option<String>,
        show: Option<ShowStatus>,
        attributes: HashMap<String, String>,
    },

    /// Basic Shape: rect
    Rect {
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        rx: Option<f64>,
        ry: Option<f64>,
        style: ShapeAttrs,
    },

    /// Basic Shape: circle
    Circle {
        cx: f64,
        cy: f64,
        r: f64,
        style: ShapeAttrs,
    },

    /// Basic Shape: ellipse
    Ellipse {
        cx: f64,
        cy: f64,
        rx: f64,
        ry: f64,
        style: ShapeAttrs,
    },

    /// Basic Shape: line
    Line {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        style: ShapeAttrs,
    },

    /// Basic Shape: polyline
    Polyline {
        points: String,
        style: ShapeAttrs,
    },

    /// Basic Shape: polygon
    Polygon {
        points: String,
        style: ShapeAttrs,
    },

    /// Basic Shape: path
    Path {
        d: String,
        style: ShapeAttrs,
    },

    /// Basic Shape: text
    Text {
        x: f64,
        y: f64,
        value: String,
        w: Option<f64>,
        h: Option<f64>,
        style: ShapeAttrs,
    },

    /// Basic Shape: image
    Image {
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        href: String,
        style: ShapeAttrs,
    },

    /// Custom or pass-through graphic element
    Generic {
        tag_name: String,
        attributes: HashMap<String, String>,
        children: Vec<CimGElement>,
    },
}

/// The root CIM/G document representing a parsed `<G>` file
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GDocument {
    pub diagram_type: Option<String>,
    pub viewbox: Option<RectArea>,
    pub background: Option<String>,
    pub app: Option<String>,
    pub context: Option<String>,
    pub other_root_attrs: HashMap<String, String>,
    pub includes: Vec<IncludeFiles>,
    pub layers: Vec<Layer>,
    pub data_lists: Vec<DataList>,
    pub elements: Vec<CimGElement>,
}
