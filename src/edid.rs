use std::collections::BTreeMap;

pub fn parse_edid(bytes: &[u8]) -> Option<BTreeMap<String, String>> {
    if bytes.len() < 128 || bytes[..8] != [0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00] {
        return None;
    }

    let mut fields = BTreeMap::new();
    let manufacturer_word = u16::from_be_bytes([bytes[8], bytes[9]]);
    fields.insert(
        "EDID Manufacturer".into(),
        decode_manufacturer(manufacturer_word),
    );
    fields.insert(
        "EDID Product Code".into(),
        u16::from_le_bytes([bytes[10], bytes[11]]).to_string(),
    );

    let serial = u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
    if serial != 0 {
        fields.insert("EDID Numeric Serial".into(), serial.to_string());
    }

    if bytes[16] != 0 {
        fields.insert("Manufacture Week".into(), bytes[16].to_string());
    }
    fields.insert(
        "Manufacture Year".into(),
        (1990u16 + bytes[17] as u16).to_string(),
    );
    fields.insert(
        "EDID Version".into(),
        format!("{}.{}", bytes[18], bytes[19]),
    );
    fields.insert(
        "Video Input".into(),
        if bytes[20] & 0x80 != 0 {
            "Digital"
        } else {
            "Analog"
        }
        .into(),
    );

    if bytes[21] != 0 {
        fields.insert("Width cm".into(), bytes[21].to_string());
    }
    if bytes[22] != 0 {
        fields.insert("Height cm".into(), bytes[22].to_string());
    }
    if bytes[23] != 0xff {
        fields.insert(
            "Gamma".into(),
            format!("{:.2}", (bytes[23] as f32 + 100.0) / 100.0),
        );
    }

    for offset in [54usize, 72, 90, 108] {
        let descriptor = &bytes[offset..offset + 18];
        if descriptor[0] != 0 || descriptor[1] != 0 || descriptor[2] != 0 {
            continue;
        }
        let value = descriptor_text(&descriptor[5..18]);
        if value.is_empty() {
            continue;
        }
        match descriptor[3] {
            0xfc => {
                fields.insert("Monitor Name".into(), value);
            }
            0xff => {
                fields.insert("Monitor Serial".into(), value);
            }
            0xfe => {
                fields.insert("Monitor Text".into(), value);
            }
            _ => {}
        }
    }

    fields.insert("Extension Blocks".into(), bytes[126].to_string());
    let checksum_ok = bytes[..128]
        .iter()
        .fold(0u8, |sum, value| sum.wrapping_add(*value))
        == 0;
    fields.insert("Base Block Checksum".into(), checksum_ok.to_string());

    Some(fields)
}

fn decode_manufacturer(value: u16) -> String {
    [10u16, 5, 0]
        .into_iter()
        .map(|shift| {
            let code = ((value >> shift) & 0x1f) as u8;
            if (1..=26).contains(&code) {
                (b'A' + code - 1) as char
            } else {
                '?'
            }
        })
        .collect()
}

fn descriptor_text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes)
        .trim_matches(|c| matches!(c, '\0' | '\n' | '\r' | ' '))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{decode_manufacturer, parse_edid};

    #[test]
    fn decodes_eisa_manufacturer() {
        assert_eq!(decode_manufacturer(0x10ac), "DEL");
    }

    #[test]
    fn rejects_short_or_invalid_header() {
        assert!(parse_edid(&[0u8; 127]).is_none());
        assert!(parse_edid(&[0u8; 128]).is_none());
    }

    #[test]
    fn parses_identity_and_text_descriptors() {
        let mut edid = [0u8; 128];
        edid[..8].copy_from_slice(&[0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00]);
        edid[8..10].copy_from_slice(&0x10acu16.to_be_bytes());
        edid[10..12].copy_from_slice(&0x1234u16.to_le_bytes());
        edid[12..16].copy_from_slice(&0x01020304u32.to_le_bytes());
        edid[16] = 12;
        edid[17] = 30;
        edid[18] = 1;
        edid[19] = 4;
        edid[20] = 0x80;
        edid[21] = 60;
        edid[22] = 34;
        edid[23] = 120;

        let name = b"ExamplePanel\n";
        edid[54] = 0;
        edid[55] = 0;
        edid[56] = 0;
        edid[57] = 0xfc;
        edid[58] = 0;
        edid[59..59 + name.len()].copy_from_slice(name);

        let checksum = edid[..127]
            .iter()
            .fold(0u8, |sum, value| sum.wrapping_add(*value));
        edid[127] = 0u8.wrapping_sub(checksum);

        let fields = parse_edid(&edid).expect("valid EDID should parse");
        assert_eq!(
            fields.get("EDID Manufacturer").map(String::as_str),
            Some("DEL")
        );
        assert_eq!(
            fields.get("EDID Product Code").map(String::as_str),
            Some("4660")
        );
        assert_eq!(
            fields.get("EDID Numeric Serial").map(String::as_str),
            Some("16909060")
        );
        assert_eq!(
            fields.get("Manufacture Year").map(String::as_str),
            Some("2020")
        );
        assert_eq!(
            fields.get("Monitor Name").map(String::as_str),
            Some("ExamplePanel")
        );
        assert_eq!(
            fields.get("Base Block Checksum").map(String::as_str),
            Some("true")
        );
    }
}
