use crate::traits::{ConvertAsBytes, Parsable};
use anyhow::anyhow;

const LZMA_COMPRESSION: &str = "lzma";
const ZLIB_COMPRESSION: &str = "zlib";

/// **PSArchiveCompression** is the type of compression that the Playstation Archive file has
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PSArchiveCompression {
    /// LZMA Compression type
    LZMA,
    /// ZLIB Compression type
    ZLIB,
    /// Error parsing compression type
    ERROR,
}

/// Should parse 4 bytes, any more or less will result in an error
impl Parsable for PSArchiveCompression {
    type Error = anyhow::Error;
    fn parse(bytes: impl ConvertAsBytes) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let bytes = bytes.convert_as_bytes();
        if LZMA_COMPRESSION.convert_as_bytes() == bytes {
            return Ok(Self::LZMA);
        } else if ZLIB_COMPRESSION.convert_as_bytes() == bytes {
            return Ok(Self::ZLIB);
        }
        Err(anyhow!("Invalid compression type"))
    }
}

impl Default for PSArchiveCompression {
    fn default() -> Self {
        Self::ERROR
    }
}

impl std::fmt::Display for PSArchiveCompression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LZMA => {
                write!(f, "lzma")
            }
            Self::ZLIB => {
                write!(f, "zlib")
            }
            Self::ERROR => {
                write!(f, "Error parsing Archive Compression")
            }
        }
    }
}

#[cfg(test)]
#[doc(hidden)]
mod test {
    use super::PSArchiveCompression;
    use crate::prelude::*;

    #[test]
    fn test_compression_parsing_lzma() {
        let bytes = "lzma".as_bytes();
        let result = PSArchiveCompression::parse(bytes);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, PSArchiveCompression::LZMA);
    }

    #[test]
    fn test_compression_parsing_zlib() {
        let bytes = "zlib".as_bytes();
        let result = PSArchiveCompression::parse(bytes);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, PSArchiveCompression::ZLIB);
    }

    #[test]
    fn test_compression_parsing_error() {
        let bytes = "nope".as_bytes();
        let result = PSArchiveCompression::parse(bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_compression_display() {
        let bytes = "lzma".as_bytes();
        let result = PSArchiveCompression::parse(bytes).unwrap();
        assert_eq!(format!("{result}"), "lzma");
    }
}
