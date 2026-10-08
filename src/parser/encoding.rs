use crate::error::{G2SvgError, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Reads raw file bytes and decodes into a UTF-8 String, respecting BOM or XML encoding declaration
pub fn read_file_to_string(path: &Path) -> Result<String> {
    let mut file = File::open(path).map_err(|e| G2SvgError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(|e| G2SvgError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;

    decode_xml_bytes(&bytes)
}

/// Decodes raw XML bytes to a String
pub fn decode_xml_bytes(bytes: &[u8]) -> Result<String> {
    // 1. Check UTF-8 BOM
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        let s = String::from_utf8(bytes[3..].to_vec())
            .map_err(|e| G2SvgError::Encoding(format!("Invalid UTF-8 after BOM: {}", e)))?;
        return Ok(s);
    }

    // 2. Scan initial slice for <?xml ... encoding="..." ?>
    let header_len = bytes.len().min(512);
    let header_ascii = String::from_utf8_lossy(&bytes[..header_len]);
    if let Some(enc) = extract_encoding_attr(&header_ascii) {
        let enc_upper = enc.to_uppercase();
        if enc_upper == "GBK" || enc_upper == "GB2312" || enc_upper == "GB18030" {
            let (cow, _, had_errors) = encoding_rs::GBK.decode(bytes);
            if had_errors {
                log::warn!("Some non-decodable characters encountered while decoding with GBK");
            }
            return Ok(cow.into_owned());
        } else if enc_upper == "ISO-8859-1" || enc_upper == "LATIN1" {
            let (cow, _, _) = encoding_rs::WINDOWS_1252.decode(bytes);
            return Ok(cow.into_owned());
        }
    }

    // 3. Try standard UTF-8 decode
    match std::str::from_utf8(bytes) {
        Ok(s) => Ok(s.to_string()),
        Err(_) => {
            // Fallback to GBK which is standard for Chinese SCADA if UTF-8 fails
            let (cow, _, _) = encoding_rs::GBK.decode(bytes);
            Ok(cow.into_owned())
        }
    }
}

fn extract_encoding_attr(header: &str) -> Option<String> {
    let lower = header.to_lowercase();
    if let Some(idx) = lower.find("encoding=") {
        let rest = &header[idx + 9..];
        let quote = rest.chars().next()?;
        if quote == '"' || quote == '\'' {
            let val = &rest[1..];
            if let Some(end_idx) = val.find(quote) {
                return Some(val[..end_idx].to_string());
            }
        }
    }
    None
}
