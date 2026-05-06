use std::convert::TryInto;

pub fn serialization(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    // Add length as 4 bytes in big-endian order
    let length = data.len() as u32;
    result.extend_from_slice(&length.to_be_bytes());
    // Add the data itself
    result.extend_from_slice(data);
    result
}

pub fn deserialization(serialized_data: &[u8]) -> Vec<u8> {
    if serialized_data.len() < 4 {
        return Vec::new();
    }

    let length_bytes: [u8; 4] = serialized_data[0..4].try_into().unwrap();
    let length = u32::from_be_bytes(length_bytes) as usize;

    let start = 4;
    let end = start + length;
    println!("数据 {:?}", (start, end, serialized_data.len()));
    if end > serialized_data.len() {
        return Vec::new();
    }
    serialized_data[start..end].to_vec()
}


pub fn bytes2bin_(bytes1: &[u8]) -> String {
    bytes1
        .iter()
        .map(|&byte| format!("{:08b}", byte))
        .collect()
}


pub fn bin2bytes_(bin1: &str) -> Vec<u8> {
    if bin1.len() % 8 != 0 {
        return Vec::new();
    }

    (0..bin1.len() / 8)
        .map(|i| {
            let byte_str = &bin1[i * 8..(i + 1) * 8];
            u8::from_str_radix(byte_str, 2).unwrap_or(0)
        })
        .collect()
}

pub fn bytes2bin(bytes1: &[u8]) -> Vec<u8> {
    bytes1
        .iter()
        .flat_map(|&byte| {
            (0..8).rev().map(move |i| (byte >> i) & 1)
        })
        .collect()
}

pub fn bin2bytes(bin1: &[u8]) -> Vec<u8> {
    if bin1.len() % 8 != 0 {
        return Vec::new();
    }

    bin1
        .chunks(8)
        .map(|chunk| {
            chunk
                .iter()
                .fold(0u8, |acc, &bit| (acc << 1) | (bit & 1))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_roundtrip() {
        let test_data = b"Test data for serialization";
        let serialized = serialization(test_data);
        let deserialized = deserialization(&serialized);
        assert_eq!(test_data.to_vec(), deserialized);
    }

    #[test]
    fn test_empty_serialization() {
        let empty: &[u8] = &[];
        let serialized = serialization(empty);
        let deserialized = deserialization(&serialized);
        assert_eq!(empty.to_vec(), deserialized);
    }

    #[test]
    fn test_large_data_serialization() {
        let large_data: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
        let serialized = serialization(&large_data);
        let deserialized = deserialization(&serialized);
        assert_eq!(large_data, deserialized);
    }

    #[test]
    fn test_bytes2bin_roundtrip() {
        let test_data = b"ABC";
        let bin_str = bytes2bin_(test_data);
        let reconstructed = bin2bytes_(&bin_str);
        assert_eq!(test_data.to_vec(), reconstructed);
    }

    #[test]
    fn test_bytes2bin_format() {
        let test_data = vec![0b10101010, 0b01010101];
        let bin_str = bytes2bin_(&test_data);
        assert_eq!(bin_str, "1010101001010101");
    }

    #[test]
    fn test_bytes2bin_list_format() {
        let test_data = vec![0b10101010]; // 170
        let bin_list = bytes2bin(&test_data);
        let expected = vec![1, 0, 1, 0, 1, 0, 1, 0];
        assert_eq!(bin_list, expected);
    }

    #[test]
    fn test_bin2bytes_list_roundtrip() {
        let test_data = vec![1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1];
        let bytes = bin2bytes(&test_data);
        let reconstructed = bytes2bin(&bytes);
        assert_eq!(test_data, reconstructed);
    }

    #[test]
    fn test_all_byte_values() {
        let test_data: Vec<u8> = (0..=255).collect();

        // Test serialization
        let serialized = serialization(&test_data);
        let deserialized = deserialization(&serialized);
        assert_eq!(test_data, deserialized);

        // Test binary conversion
        let bin_str = bytes2bin_(&test_data);
        let reconstructed = bin2bytes_(&bin_str);
        assert_eq!(test_data, reconstructed);

        // Test binary list conversion
        let bin_list = bytes2bin(&test_data);
        let reconstructed_list = bin2bytes(&bin_list);
        assert_eq!(test_data, reconstructed_list);
    }

    #[test]
    fn test_empty_binary_strings() {
        let empty: &[u8] = &[];
        let bin_str = bytes2bin_(empty);
        assert_eq!(bin_str, "");

        let reconstructed = bin2bytes_(&bin_str);
        assert_eq!(empty.to_vec(), reconstructed);

        let bin_list = bytes2bin(empty);
        assert_eq!(bin_list, Vec::<u8>::new());

        let reconstructed_list = bin2bytes(&bin_list);
        assert_eq!(empty.to_vec(), reconstructed_list);
    }

    #[test]
    fn test_invalid_bin2bytes() {
        // Test with invalid length (not multiple of 8)
        let invalid_bin = vec![1, 0, 1]; // 3 bits, not a multiple of 8
        let result = bin2bytes(&invalid_bin);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_bin2bytes_malformed() {
        // Test with invalid binary string
        let invalid_str = "1010101"; // 7 bits, not a multiple of 8
        let result = bin2bytes_(invalid_str);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_single_byte_conversions() {
        let test_bytes = vec![0u8, 1, 127, 128, 255];

        for &byte in &test_bytes {
            let bytes = vec![byte];

            // Test binary string conversion
            let bin_str = bytes2bin_(&bytes);
            let reconstructed = bin2bytes_(&bin_str);
            assert_eq!(bytes, reconstructed);

            // Test binary list conversion
            let bin_list = bytes2bin(&bytes);
            let reconstructed_list = bin2bytes(&bin_list);
            assert_eq!(bytes, reconstructed_list);
        }
    }
}