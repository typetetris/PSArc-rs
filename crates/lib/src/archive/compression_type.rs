use anyhow::anyhow;

const LZMA_COMPRESSION: &[u8; 4] = b"lzma";
const ZLIB_COMPRESSION: &[u8; 4] = b"zlib";

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

impl PSArchiveCompression {
    pub fn parse(bytes: &[u8]) -> anyhow::Result<Self> {
        let snippet = &bytes[0..4];
        if LZMA_COMPRESSION == snippet {
            return Ok(Self::LZMA);
        } else if ZLIB_COMPRESSION == snippet {
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
