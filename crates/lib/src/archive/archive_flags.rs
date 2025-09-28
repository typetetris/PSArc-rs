use crate::prelude::*;
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

/// Should parse 4 bytes, any more or less will result in an error
impl Parsable for PSArchiveFlags {
    type Error = anyhow::Error;
    fn parse(bytes: impl ConvertAsBytes) -> Result<Self, Self::Error> {
        let bytes = &bytes.convert_as_bytes()[..];
        match *bytes {
            [0, 0, 0, 0] => Ok(Self::RELATIVE),
            [0, 0, 0, 1] => Ok(Self::IGNORECASE),
            [0, 0, 0, 2] => Ok(Self::ABSOLUTE),
            _ => Err(anyhow!("Invalid flags")),
        }
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

#[cfg(test)]
#[doc(hidden)]
mod test {
    use super::PSArchiveFlags;
    use crate::prelude::*;

    #[test]
    fn test_flags_parsing() {
        let bytes = include_bytes!("../../res/test.pak")[0x1C..0x20].to_vec();
        let result = PSArchiveFlags::parse(bytes);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, PSArchiveFlags::ABSOLUTE);
    }
}
