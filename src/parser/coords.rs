use crate::error::{G2SvgError, Result};
use crate::model::cimg::{ElementLoc, GluePoint, RectArea, ShowStatus};

/// Parse a string of comma or space separated numbers into a Vec<f64>
pub fn parse_number_list(s: &str) -> Vec<f64> {
    let normalized = s.replace(',', " ");
    normalized
        .split_whitespace()
        .filter_map(|part| part.parse::<f64>().ok())
        .collect()
}

/// Parse viewbox="x, y, w, h" or "x,y w,h" or "x y w h"
pub fn parse_viewbox(s: &str) -> Result<RectArea> {
    let nums = parse_number_list(s);
    if nums.len() >= 4 {
        Ok(RectArea::new(nums[0], nums[1], nums[2], nums[3]))
    } else {
        Err(G2SvgError::InvalidCoordinate {
            attr: "viewbox".to_string(),
            raw: s.to_string(),
            details: format!("Expected 4 numbers (x, y, w, h), found {}", nums.len()),
        })
    }
}

/// Parse loc="x,y w,h" or "x, y, w, h" or "x,y"
pub fn parse_loc(s: &str) -> Result<ElementLoc> {
    let nums = parse_number_list(s);
    match nums.len() {
        2 => Ok(ElementLoc {
            x: nums[0],
            y: nums[1],
            w: None,
            h: None,
        }),
        4.. => Ok(ElementLoc {
            x: nums[0],
            y: nums[1],
            w: Some(nums[2]),
            h: Some(nums[3]),
        }),
        1 => Ok(ElementLoc {
            x: nums[0],
            y: 0.0,
            w: None,
            h: None,
        }),
        _ => Err(G2SvgError::InvalidCoordinate {
            attr: "loc".to_string(),
            raw: s.to_string(),
            details: "loc must have at least x, y coordinates".to_string(),
        }),
    }
}

/// Parse box="X, Y W, H" in definition elements
pub fn parse_box(s: &str) -> Result<RectArea> {
    let nums = parse_number_list(s);
    if nums.len() >= 4 {
        Ok(RectArea::new(nums[0], nums[1], nums[2], nums[3]))
    } else {
        Err(G2SvgError::InvalidCoordinate {
            attr: "box".to_string(),
            raw: s.to_string(),
            details: format!("Expected 4 numbers for box (x, y, w, h), found {}", nums.len()),
        })
    }
}

/// Parse glue="x0,y0 x1,y1 ..."
pub fn parse_glue_points(s: &str) -> Vec<GluePoint> {
    let nums = parse_number_list(s);
    let mut points = Vec::new();
    let mut i = 0;
    while i + 1 < nums.len() {
        points.push(GluePoint {
            x: nums[i],
            y: nums[i + 1],
        });
        i += 2;
    }
    points
}

/// Parse show="Q,T,F,S" or show="status"
pub fn parse_show(s: &str) -> ShowStatus {
    let trimmed = s.trim();
    let parts: Vec<&str> = trimmed.split(',').map(|p| p.trim()).collect();
    if parts.len() == 4 {
        let parse_byte = |v: &str| -> Option<u8> {
            if let Ok(n) = v.parse::<u8>() {
                Some(n)
            } else if let Ok(n) = u8::from_str_radix(v.trim_start_matches("0x"), 16) {
                Some(n)
            } else {
                None
            }
        };

        ShowStatus {
            raw: trimmed.to_string(),
            q: parse_byte(parts[0]),
            t: parse_byte(parts[1]),
            f: parse_byte(parts[2]),
            s: parse_byte(parts[3]),
        }
    } else {
        ShowStatus {
            raw: trimmed.to_string(),
            q: None,
            t: None,
            f: None,
            s: None,
        }
    }
}

/// Parse background attribute (e.g. "236, 236, 236" or "white" or "bg.png")
pub fn parse_background(s: &str) -> String {
    let trimmed = s.trim();
    let nums = parse_number_list(trimmed);
    if nums.len() >= 3 {
        format!("rgb({}, {}, {})", nums[0] as u8, nums[1] as u8, nums[2] as u8)
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_loc() {
        let loc1 = parse_loc("104, 246, 20, 20").unwrap();
        assert_eq!(loc1.x, 104.0);
        assert_eq!(loc1.y, 246.0);
        assert_eq!(loc1.w, Some(20.0));
        assert_eq!(loc1.h, Some(20.0));

        let loc2 = parse_loc("50,56 16,32").unwrap();
        assert_eq!(loc2.x, 50.0);
        assert_eq!(loc2.y, 56.0);
        assert_eq!(loc2.w, Some(16.0));
        assert_eq!(loc2.h, Some(32.0));

        let loc3 = parse_loc("77, 859").unwrap();
        assert_eq!(loc3.x, 77.0);
        assert_eq!(loc3.y, 859.0);
        assert_eq!(loc3.w, None);
    }

    #[test]
    fn test_parse_viewbox() {
        let vb = parse_viewbox("0, 0, 645, 1050").unwrap();
        assert_eq!(vb.x, 0.0);
        assert_eq!(vb.y, 0.0);
        assert_eq!(vb.w, 645.0);
        assert_eq!(vb.h, 1050.0);
    }

    #[test]
    fn test_parse_glue() {
        let pts = parse_glue_points("18,3 18,15");
        assert_eq!(pts.len(), 2);
        assert_eq!(pts[0].x, 18.0);
        assert_eq!(pts[0].y, 3.0);
        assert_eq!(pts[1].x, 18.0);
        assert_eq!(pts[1].y, 15.0);
    }

    #[test]
    fn test_parse_show() {
        let s = parse_show("0, 5, 0, 0");
        assert_eq!(s.q, Some(0));
        assert_eq!(s.t, Some(5));
        assert_eq!(s.f, Some(0));
        assert_eq!(s.s, Some(0));
    }
}
