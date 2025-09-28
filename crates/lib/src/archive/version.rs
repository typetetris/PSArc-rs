use anyhow::anyhow;

/// **ArchiveVersion** contains the major and minor version numbers of an Playstation Archive file
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PSArchiveVersion {
    /// **major** is the major version number of the Playstation Archive file
    pub major: u16,
    /// **minor** is the minor version number of the Playstation Archive file
    pub minor: u16,
}

impl PSArchiveVersion {
    pub fn parse(bytes: &[u8]) -> anyhow::Result<Self> {
        match bytes.len() {
            4 => {
                let major = bytes[1] as u16 + ((bytes[0] as u16) << 8);
                let minor = bytes[3] as u16 + ((bytes[2] as u16) << 8);
                Ok(Self { major, minor })
            }
            _ => Err(anyhow!("Invalid length of bytes")),
        }
    }
}

impl std::fmt::Display for PSArchiveVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}.{}", self.major, self.minor)
    }
}

#[cfg(test)]
#[doc(hidden)]
mod test {
    use super::PSArchiveVersion;

    #[test]
    fn test_version_parsing() {
        let bytes = include_bytes!("../../res/test.pak")[0x4..0x8].to_vec();
        let result = PSArchiveVersion::parse(&bytes[..]);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.major, 1);
        assert_eq!(result.minor, 4);
    }

    #[test]
    fn test_version_display() {
        let bytes = include_bytes!("../../res/test.pak")[0x4..0x8].to_vec();
        let result = PSArchiveVersion::parse(&bytes[..]).unwrap();
        assert_eq!(format!("{result}"), "v1.4");
    }
}
