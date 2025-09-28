use crate::primitive;

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
    pub fn parse(bytes: &[u8]) -> anyhow::Result<(Self, &[u8])> {
        let (major, bytes) = primitive(u16::from_be_bytes, bytes)?;
        let (minor, bytes) = primitive(u16::from_be_bytes, bytes)?;
        Ok((Self { major, minor }, bytes))
    }
}

impl std::fmt::Display for PSArchiveVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}.{}", self.major, self.minor)
    }
}
