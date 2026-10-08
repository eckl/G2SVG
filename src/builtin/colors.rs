use crate::model::defs::ColorEntry;

/// Standard voltage level color identifications per IEC TS 61970-556 Annex B Table B.1
pub fn get_default_colors() -> Vec<ColorEntry> {
    vec![
        ColorEntry::new("blue", "1000kV", 0, 0, 255, Some(1)),
        ColorEntry::new("blue", "800kV", 0, 0, 255, Some(2)),
        ColorEntry::new("orange", "750kV", 250, 128, 10, Some(3)),
        ColorEntry::new("orange", "660kV", 250, 128, 10, Some(4)),
        ColorEntry::new("red", "500kV", 255, 0, 0, Some(5)),
        ColorEntry::new("red", "400kV", 255, 0, 0, Some(6)),
        ColorEntry::new("brightblue", "330kV", 30, 144, 255, Some(7)),
        ColorEntry::new("purple", "220kV", 128, 0, 128, Some(8)),
        ColorEntry::new("vermeil", "110kV", 240, 65, 85, Some(9)),
        ColorEntry::new("gold", "66kV", 255, 204, 0, Some(10)),
        ColorEntry::new("yellow", "35kV", 255, 255, 0, Some(11)),
        ColorEntry::new("brown", "20kV", 226, 172, 6, Some(12)),
        ColorEntry::new("darkgreen", "15kV", 0, 128, 0, Some(13)),
        ColorEntry::new("lightgreen", "13kV", 0, 210, 0, Some(14)),
        ColorEntry::new("crimson", "10kV", 185, 72, 66, Some(15)),
        ColorEntry::new("darkblue", "6kV", 0, 0, 139, Some(16)),
        ColorEntry::new("grey", "0kV", 128, 128, 128, Some(17)),
    ]
}
