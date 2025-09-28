use super::PSArchiveFlags;

/// **PSArchiveTOC** contains the table of content and details about each resource
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PSArchiveTOC {
    /// **length** is the total length of the table of contents
    pub length: u32,
    /// **entry_size** contains each entries size
    pub entry_size: u32,
    /// **entry_count** is the amount of entries in the Playstation Archive file
    pub entry_count: u32,
    /// **block_size** is the block size
    pub block_size: u32,
    // **flags** is the flags of the Playstation Archive file
    pub flags: PSArchiveFlags,
}

impl PSArchiveTOC {
    pub fn parse(bytes: &[u8]) -> anyhow::Result<Self> {
        let length = u32::from_be_bytes(bytes[0..4].try_into()?) - 32;
        let entry_size = u32::from_be_bytes(bytes[4..8].try_into()?);
        let entry_count = u32::from_be_bytes(bytes[8..12].try_into()?);
        let block_size = u32::from_be_bytes(bytes[12..16].try_into()?);
        let flags = PSArchiveFlags::parse(bytes[16..20].try_into()?)?;
        Ok(Self {
            length,
            entry_size,
            entry_count,
            block_size,
            flags,
        })
    }
}

#[cfg(test)]
#[doc(hidden)]
mod test {
    use super::{PSArchiveFlags, PSArchiveTOC};

    #[test]
    fn test_toc_parsing() {
        let bytes = include_bytes!("../../res/test.pak")[0xC..].to_vec();
        let result = PSArchiveTOC::parse(&bytes[..]);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.length, 64);
        assert_eq!(result.entry_size, 30);
        assert_eq!(result.entry_count, 1);
        assert_eq!(result.flags, PSArchiveFlags::ABSOLUTE);
    }
}
