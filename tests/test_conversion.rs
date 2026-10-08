use g2svg::{convert_file, convert_str, ConverterConfig};
use roxmltree::Document;
use std::fs;
use std::path::Path;

#[test]
fn test_convert_annex_d_power_plant() {
    let input = Path::new("samples/annex_d_power_plant.g");
    let output = Path::new("samples/annex_d_power_plant.svg");
    let config = ConverterConfig::default();

    convert_file(input, output, &config).expect("Conversion of Annex D sample should succeed");

    assert!(output.exists());
    let svg_content = fs::read_to_string(output).expect("Should read output SVG");

    // Validate SVG XML
    let doc = Document::parse(&svg_content).expect("Output SVG must be valid XML");
    let root = doc.root_element();
    assert_eq!(root.tag_name().name(), "svg");
    assert_eq!(root.attribute("viewBox"), Some("0 0 645 1050"));

    // Check that namespaces are present
    assert_eq!(
        root.lookup_namespace_uri(Some("cimg")),
        Some("http://iec.ch/TC57/61970-556#")
    );

    // Verify equipment and elements exist in output
    assert!(svg_content.contains("id=\"plant-A\""));
    assert!(svg_content.contains("id=\"bus1\""));
    assert!(svg_content.contains("id=\"CB01\""));
    assert!(svg_content.contains("id=\"G1\""));
    assert!(svg_content.contains("id=\"T1\""));

    // Verify background rect
    assert!(svg_content.contains("class=\"cimg-background\""));
}

#[test]
fn test_convert_substation_and_inlining() {
    let input = Path::new("samples/substation_fig16.g");
    let output_use = Path::new("samples/substation_use.svg");
    let output_inline = Path::new("samples/substation_inline.svg");

    let config_use = ConverterConfig {
        inline_symbols: false,
        ..Default::default()
    };
    convert_file(input, output_use, &config_use).expect("Symbol reference conversion should succeed");

    let config_inline = ConverterConfig {
        inline_symbols: true,
        ..Default::default()
    };
    convert_file(input, output_inline, &config_inline).expect("Inline symbol conversion should succeed");

    let svg_use = fs::read_to_string(output_use).unwrap();
    let svg_inline = fs::read_to_string(output_inline).unwrap();

    assert!(svg_use.contains("<use xlink:href=\"#breaker0\""));
    assert!(!svg_inline.contains("<use xlink:href=\"#breaker0\""));
    assert!(svg_inline.contains("rect") || svg_inline.contains("circle"));

    // Validate XML syntax of both
    Document::parse(&svg_use).expect("Valid SVG");
    Document::parse(&svg_inline).expect("Valid SVG");
}

#[test]
fn test_convert_gbk_string() {
    let gbk_xml = "<?xml version=\"1.0\" encoding=\"GBK\"?>\n<G viewbox=\"0,0 800,600\"><text x=\"100\" y=\"100\" value=\"变电站接线图\" /></G>";
    let config = ConverterConfig::default();
    let svg = convert_str(gbk_xml, &config).expect("Conversion of string with Chinese text should succeed");

    assert!(svg.contains("变电站接线图"));
    Document::parse(&svg).expect("Valid SVG output");
}

#[test]
fn test_voltage_color_mapping() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<G viewbox="0,0 500,500">
  <Breaker id="CB_500" loc="10,10 20,20" show="0,5,0,0" />
  <Breaker id="CB_220" loc="10,50 20,20" show="0,8,0,0" />
</G>"#;
    let config = ConverterConfig::default();
    let svg = convert_str(xml, &config).unwrap();

    // Show 0,5,0,0 maps to 500kV red (255, 0, 0)
    assert!(svg.contains("rgb(255, 0, 0)"));
    // Show 0,8,0,0 maps to 220kV purple (128, 0, 128)
    assert!(svg.contains("rgb(128, 0, 128)"));
}
