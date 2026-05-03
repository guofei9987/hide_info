use hide_info::hide_as_img::HideAsImg;
use std::fs;

#[test]
fn test_hide_as_img_one() {
    let test_data = b"Hello, World! This is a test message.";
    let img_filename = format!("./files/hide_as_img_{}.png", 9);

    let hide_as_img = HideAsImg::new();
    let img_bytes = hide_as_img.encode(test_data).expect("Failed to encode");
    fs::write(&img_filename, img_bytes).unwrap();
    println!("Generated image: {}", img_filename);

    let data_bytes = fs::read(img_filename).unwrap();
    println!("Decoding image: {}", data_bytes.len());
    let decoded = hide_as_img.decode(&data_bytes).expect("Failed to decode");

    assert_eq!(test_data.to_vec(), decoded);
}
#[test]
fn test_hide_as_img_batch() {
    let test_cases = [
        b"Hello, World! This is a test message.".to_vec(),
        b"Secret message hidden in plain sight!".to_vec(),
        (0..10000).map(|i| (i % 256) as u8).collect(),
        vec![1, 2],
    ];

    for (idx, test_case) in test_cases.iter().enumerate() {
        let hide_as_img = HideAsImg::new();
        let img_filename = format!("./files/hide_as_img_{}.png", idx);

        let img_bytes = hide_as_img.encode(test_case).expect("Failed to encode");
        fs::write(&img_filename, img_bytes).unwrap();
        println!("Generated image: {}", img_filename);

        let data_bytes = fs::read(img_filename).unwrap();
        println!("Decoding image: {}", data_bytes.len());
        let decoded = hide_as_img.decode(&data_bytes).expect("Failed to decode");
        assert_eq!(test_case.to_vec(), decoded);
    }
}
