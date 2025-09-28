use std::io::Read;

use super::{PSArchiveCompression, PSArchiveTOC, PSArchiveTableItem, PSArchiveVersion};
use anyhow::anyhow;

const PSARC_HEADER: &[u8; 4] = b"PSAR";

/// **PSArchive** contains all the information about a complete singular Playstation Archive file
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PSArchive<'a> {
    /// **version** is the Playstation Archive file version
    pub version: PSArchiveVersion,
    /// **compression** is the Playstation Archive file compression
    pub compression: PSArchiveCompression,
    /// **table_of_contents** is the table of contents for the Playstation Archive file
    pub table_of_contents: PSArchiveTOC,
    /// **manifest** is the manifest file for the Playstation Archive file
    pub manifest: PSArchiveTableItem,
    pub files: Vec<String>,
    pub contents: Vec<PSArchiveTableItem>,
    pub file_contents: &'a [u8],
}

impl<'a> PSArchive<'a> {
    pub fn parse_manifest(&self) -> anyhow::Result<Vec<String>> {
        let strings = String::from_utf8(self.parse_file(0)?)?
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>();
        Ok(strings)
    }

    pub fn parse_file(&self, item: usize) -> anyhow::Result<Vec<u8>> {
        let item = &self.contents[item];
        let bytes = &self.file_contents[item.file_offset as usize..];
        let uncompressed_size = item.uncompressed_size as usize;
        Ok(if uncompressed_size > 100 {
            let mut z = flate2::read::ZlibDecoder::new(bytes);
            let mut s = Vec::new();
            z.read_to_end(&mut s)?;
            s
        } else {
            bytes[..uncompressed_size].into()
        })
    }

    pub fn parse(bytes: &'a [u8]) -> anyhow::Result<Self> {
        if &bytes[0..4] == PSARC_HEADER {
            let version = PSArchiveVersion::parse(&bytes[0x4..0x8])?;
            let compression = PSArchiveCompression::parse(&bytes[0x8..0xC])?;
            let table_of_contents = PSArchiveTOC::parse(&bytes[0xC..0x20])?;
            let contents_result: Result<Vec<_>, _> = (0..table_of_contents.entry_count)
                .map(|elem|
                    // header
                    0x20
                    // elems already read
                    + elem * table_of_contents.entry_size)
                .map(|offset| offset as usize)
                .map(|offset| PSArchiveTableItem::parse(&bytes[offset..]))
                .collect();
            let contents = contents_result?;
            let manifest = contents
                .first()
                .ok_or_else(|| anyhow!("manifest missing"))?
                .clone();

            Ok(Self {
                version,
                compression,
                table_of_contents,
                manifest,
                files: Vec::new(),
                contents,
                file_contents: bytes,
            })
        } else {
            Err(anyhow!("Invalid header for PSArc format"))
        }
    }
}

#[cfg(test)]
#[doc(hidden)]
mod test {
    use super::{
        super::PSArchiveFlags, PSArchiveCompression, PSArchiveTOC, PSArchiveTableItem,
        PSArchiveVersion,
    };
    use crate::prelude::*;

    #[test]
    fn test_archive_parsing() {
        let bytes = include_bytes!("../../res/test.pak").to_vec();
        let result = PSArchive::parse(&*bytes);
        assert!(result.is_ok());
        let result = result.unwrap();
        result.parse_manifest();
        assert_eq!(
            result,
            PSArchive {
                version: PSArchiveVersion { major: 1, minor: 4 },
                compression: PSArchiveCompression::ZLIB,
                table_of_contents: PSArchiveTOC {
                    length: 64,
                    entry_size: 30,
                    entry_count: 1,
                    block_size: 65536,
                    flags: PSArchiveFlags::ABSOLUTE,
                },
                manifest: PSArchiveTableItem {
                    md5_digest: [0; 16],
                    block_offset: 0,
                    uncompressed_size: 17,
                    file_offset: 96,
                },
                files: vec!["/data/example.xml".to_string()],
                contents: vec![PSArchiveTableItem {
                    md5_digest: [0; 16],
                    block_offset: 1,
                    uncompressed_size: 115,
                    file_offset: 113,
                }],
                file_contents: &bytes[..]
            }
        );
        let contents = String::from_utf8(result.parse_file(0).unwrap());
        assert!(contents.is_ok());
        assert_eq!(
            contents.unwrap(),
            "<TextData>\r\n	<Property name=\"color\" value=\"#ff0000\" />\r\n	<Property name=\"text\" value=\"Hello there!\" />\r\n</TextData>"
        );
    }
}
