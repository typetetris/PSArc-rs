use anyhow::anyhow;

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
    pub fn parse(bytes: &[u8]) -> anyhow::Result<(Self, &[u8])> {
        let (snippet, rest) = bytes
            .split_at_checked(4)
            .ok_or_else(|| anyhow!("too short"))?;
        let result = match snippet {
            b"lzma" => Self::LZMA,
            b"zlib" => Self::ZLIB,
            _ => anyhow::bail!("Invalid compression type"),
        };
        Ok((result, rest))
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
