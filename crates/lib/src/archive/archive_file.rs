use std::io::Read;

use super::{PSArchiveCompression, PSArchiveTOC, PSArchiveTableItem, PSArchiveVersion};
use crate::{
    prelude::ParsableContext,
    traits::{ConvertAsBytes, Parsable},
};
use anyhow::anyhow;

const PSARC_HEADER: &str = "PSAR";

/// **PSArchive** contains all the information about a complete singular Playstation Archive file
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PSArchive {
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
    pub file_size: usize,
}

impl PSArchive {
    pub fn get_manifest_size_offset(&self) -> (usize, usize) {
        (
            self.contents[0].file_offset as usize - self.manifest.file_offset as usize,
            self.manifest.file_offset as usize,
        )
    }

    pub fn get_size_offset(&self, item: usize) -> (usize, usize) {
        if item + 1 == self.contents.len() {
            (
                self.file_size - self.contents[item].file_offset as usize,
                self.contents[item].file_offset as usize,
            )
        } else {
            (
                self.contents[item + 1].file_offset as usize
                    - self.contents[item].file_offset as usize
                    - 1,
                self.contents[item].file_offset as usize,
            )
        }
    }

    pub fn parse_manifest(&mut self, bytes: impl ConvertAsBytes) -> Vec<String> {
        let bytes = bytes.convert_as_bytes();
        let (size, _) = self.get_manifest_size_offset();
        if size > 100 {
            let mut z = flate2::read::ZlibDecoder::new(&bytes[..]);
            let mut s = String::new();
            z.read_to_string(&mut s).unwrap();
            let strings = s
                .split("\n")
                .collect::<Vec<&str>>()
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>();
            self.files = strings.clone();
            strings
        } else {
            let s = String::from_utf8(bytes).unwrap();
            let strings = s
                .split("\n")
                .collect::<Vec<&str>>()
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>();
            self.files = strings.clone();
            strings
        }
    }

    pub fn parse_file(&self, item: usize, bytes: impl ConvertAsBytes) -> Vec<u8> {
        let bytes = bytes.convert_as_bytes();
        let item = &self.contents[item];
        if item.uncompressed_size > 100 {
            let mut z = flate2::read::ZlibDecoder::new(&bytes[..]);
            let mut s = Vec::new();
            z.read_to_end(&mut s).unwrap();
            s
        } else {
            bytes
        }
    }
}

impl Parsable for PSArchive {
    type Error = anyhow::Error;
    fn parse(bytes: impl ConvertAsBytes) -> Result<Self, Self::Error> {
        let bytes = bytes.convert_as_bytes();
        let header = (&bytes[0..4]).convert_as_bytes();
        if header == PSARC_HEADER.convert_as_bytes() {
            let version = PSArchiveVersion::parse(&bytes[0x4..0x8])?;
            let compression = PSArchiveCompression::parse(&bytes[0x8..0xC])?;
            let table_of_contents = PSArchiveTOC::parse(&bytes[0xC..0x20])?;
            let mut manifest = PSArchiveTableItem::new(PSArchiveCompression::ZLIB);
            let manifest = manifest.parse(&bytes[0x20..0x3E])?;
            let mut contents = Vec::new();
            for i in 0..table_of_contents.entry_count as usize {
                let item_bytes = &bytes[0x3E + i * table_of_contents.entry_size as usize
                    ..0x3E
                        + i * table_of_contents.entry_size as usize
                        + table_of_contents.entry_size as usize];
                let mut item = PSArchiveTableItem::new(PSArchiveCompression::ZLIB);
                let item = item.parse(item_bytes).unwrap();
                contents.push(item);
            }
            Ok(Self {
                version,
                compression,
                table_of_contents,
                manifest,
                files: Vec::new(),
                contents,
                file_size: 0,
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
        let mut result = result.unwrap();
        result.file_size = bytes.len();
        let (size, offset) = result.get_manifest_size_offset();
        result.parse_manifest(&bytes[offset..offset + size]);
        assert_eq!(
            result,
            PSArchive {
                version: PSArchiveVersion { major: 1, minor: 4 },
                compression: PSArchiveCompression::ZLIB,
                table_of_contents: PSArchiveTOC {
                    length: 64,
                    entry_size: 30,
                    entry_count: 1,
                    flags: PSArchiveFlags::ABSOLUTE,
                },
                manifest: PSArchiveTableItem {
                    compression_type: PSArchiveCompression::ZLIB,
                    block_offset: 0,
                    uncompressed_size: 17,
                    file_offset: 96,
                },
                files: vec!["/data/example.xml".to_string()],
                contents: vec![PSArchiveTableItem {
                    compression_type: PSArchiveCompression::ZLIB,
                    block_offset: 1,
                    uncompressed_size: 115,
                    file_offset: 113,
                }],
                file_size: 196,
            }
        );
        let (size, offset) = result.get_size_offset(0);
        let contents = String::from_utf8(result.parse_file(0, &bytes[offset..offset + size]));
        assert!(contents.is_ok());
        assert_eq!(contents.unwrap(), "<TextData>\r\n	<Property name=\"color\" value=\"#ff0000\" />\r\n	<Property name=\"text\" value=\"Hello there!\" />\r\n</TextData>");
    }
}
