use crate::signatures::common::{CONFIDENCE_MEDIUM, SignatureError, SignatureResult};
use crate::structures::ogg::{OggHeaderType, parse_ogg_header};

/// Human readable description
pub const DESCRIPTION: &str = "Ogg Audio File";

pub fn ogg_magic() -> Vec<Vec<u8>> {
    vec![b"\x4F\x67\x67\x53".to_vec()]
}

pub fn ogg_parser(file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    // Successful return value
    let mut result = SignatureResult {
        offset,
        description: DESCRIPTION.to_string(),
        confidence: CONFIDENCE_MEDIUM,
        ..Default::default()
    };

    let mut tmp_offset = offset;
    while let Ok(header) = parse_ogg_header(&file_data[tmp_offset..]) {
        let header_size = 27 + header.page_segments as usize;
        let mut packet_size: usize = 0;
        header
            .segment_table
            .iter()
            .for_each(|s| packet_size += *s as usize);
        tmp_offset += header_size + packet_size;
        result.size += header_size + packet_size;
        if header.header_type == Some(OggHeaderType::Eos) {
            return Ok(result);
        }
    }

    Err(SignatureError)
}
