use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum G2SvgError {
    #[error("I/O error at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("XML parsing error at {location}: {message}")]
    XmlParse {
        location: String,
        message: String,
    },

    #[error("Text decoding error: {0}")]
    Encoding(String),

    #[error("Invalid coordinate or number format '{raw}' in attribute '{attr}': {details}")]
    InvalidCoordinate {
        attr: String,
        raw: String,
        details: String,
    },

    #[error("Missing required attribute '{attr}' on element '{element}'")]
    MissingAttribute {
        attr: String,
        element: String,
    },

    #[error("Referenced element definition '{symbol}' not found")]
    DefinitionNotFound {
        symbol: String,
    },

    #[error("Conversion error: {0}")]
    Conversion(String),
}

pub type Result<T> = std::result::Result<T, G2SvgError>;
