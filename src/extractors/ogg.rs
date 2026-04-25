use crate::extractors::common::{Chroot, ExtractionResult, Extractor, ExtractorType};
use crate::structures::ogg::{OggHeaderType, parse_ogg_header};

pub fn ogg_extractor() -> Extractor {
    Extractor {
        utility: ExtractorType::Internal(extract_ogg_file),
        ..Default::default()
    }
}

/// Internal extractor for carve pieces of ogg images to disk
pub fn extract_ogg_file(
    file_data: &[u8],
    offset: usize,
    output_directory: Option<&str>,
) -> ExtractionResult {
    const OUTFILE_NAME: &str = "track.ogg";
    let mut result = ExtractionResult {
        ..Default::default()
    };

    let mut tmp_offset = offset;
    let mut data_size = 0;
    let mut reached_eos = false;
    while let Ok(header) = parse_ogg_header(&file_data[tmp_offset..]) {
        let header_size = 27 + header.page_segments as usize;
        let mut packet_size: usize = 0;
        header
            .segment_table
            .iter()
            .for_each(|s| packet_size += *s as usize);
        tmp_offset += header_size + packet_size;
        data_size += header_size + packet_size;
        if header.header_type == Some(OggHeaderType::Eos) {
            reached_eos = true;
            break;
        }
    }

    if reached_eos {
        result.size = Some(data_size);
        result.success = true;

        if output_directory.is_some() {
            let chroot = Chroot::new(output_directory);
            result.success =
                chroot.carve_file(OUTFILE_NAME, file_data, offset, result.size.unwrap());
        }
    }

    result
}
