use anyhow::anyhow;

/// **PSArchiveFlags** is the flags for the Playstation Archive file
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PSArchiveFlags {
    /// Relative paths
    RELATIVE,
    /// Case-insensitive paths
    IGNORECASE,
    /// Absolute paths
    ABSOLUTE,
    /// Error parsing flags
    ERROR,
}

impl PSArchiveFlags {
    pub fn parse(bytes: &[u8]) -> anyhow::Result<(Self, &[u8])> {
        let (snippet, bytes) = bytes
            .split_at_checked(4)
            .ok_or_else(|| anyhow!("too short"))?;
        let result = match snippet {
            [0, 0, 0, 0] => Self::RELATIVE,
            [0, 0, 0, 1] => Self::IGNORECASE,
            [0, 0, 0, 2] => Self::ABSOLUTE,
            _ => anyhow::bail!("Invalid flags"),
        };
        Ok((result, bytes))
    }
}

impl Default for PSArchiveFlags {
    fn default() -> Self {
        Self::ERROR
    }
}

impl std::fmt::Display for PSArchiveFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RELATIVE => {
                write!(f, "relative")
            }
            Self::IGNORECASE => {
                write!(f, "ignorecase")
            }
            Self::ABSOLUTE => {
                write!(f, "absolute")
            }
            Self::ERROR => {
                write!(f, "Error parsing Archive Flags")
            }
        }
    }
}
