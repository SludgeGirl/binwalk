use crate::structures::common::{self, StructureError};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum OggHeaderType {
    NewPacket = 0x00,
    Continuation = 0x01,
    #[default]
    Bos = 0x02,
    Eos = 0x04,
}

impl TryFrom<u8> for OggHeaderType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(OggHeaderType::NewPacket),
            0x01 => Ok(OggHeaderType::Continuation),
            0x02 => Ok(OggHeaderType::Bos),
            0x04 => Ok(OggHeaderType::Eos),
            _ => Err("Invalid Ogg header type"),
        }
    }
}

/// Struct to store useful Ogg header info
#[derive(Debug, Clone, Default)]
pub struct OggHeader {
    pub header_type: Option<OggHeaderType>,
    pub page_segments: u8,
    pub segment_table: Vec<u8>,
}

/// Parses a Ogg file header
pub fn parse_ogg_header(file_data: &[u8]) -> Result<OggHeader, StructureError> {
    let page_segments = file_data[26];
    let ogg_header_structure_base = vec![
        ("magic", "u32"),
        ("version", "u8"),
        ("header_type", "u8"),
        ("granule_position", "u64"),
        ("bitstream_serial_number", "u32"),
        ("page_sequence_number", "u32"),
        ("checksum", "u32"),
        ("page_segments", "u8"),
    ];
    let mut ogg_header_structure_tmp: Vec<(String, String)> = Vec::new();

    for (key, val) in ogg_header_structure_base {
        ogg_header_structure_tmp.push((key.into(), val.into()))
    }

    for i in 0..page_segments {
        let key = format!("segment_table_{i}");
        ogg_header_structure_tmp.push((key, "u8".into()));
    }

    let mut ogg_header_structure = Vec::new();

    for (key, val) in &ogg_header_structure_tmp {
        ogg_header_structure.push((key.as_str(), val.as_str()));
    }

    if let Ok(ogg_header) = common::parse(file_data, &ogg_header_structure, "little") {
        let mut segment_table = vec![];

        for i in 0..page_segments {
            segment_table.push(*ogg_header.get(&format!("segment_table_{i}")).expect("meow") as u8)
        }

        let mut header_type = None;
        if let Ok(tmp_header_type) =
            OggHeaderType::try_from(*ogg_header.get("header_type").expect("meow") as u8)
        {
            header_type = Some(tmp_header_type);
        }

        if *ogg_header.get("version").expect("meow") == 0 {
            return Ok(OggHeader {
                header_type,
                page_segments,
                segment_table,
            });
        }
    }

    Err(StructureError)
}
