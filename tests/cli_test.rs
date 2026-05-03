use std::{env, fs, path::PathBuf, process::Command};

#[test]
fn test_cli_hide_as_img_encode_decode() {
    let temp_dir = env::temp_dir();
    let input_file = temp_dir.join(format!("hide_info_cli_input_{}.bin", std::process::id()));
    let encoded_file = temp_dir.join(format!("hide_info_cli_encoded_{}.png", std::process::id()));
    let decoded_file = temp_dir.join(format!("hide_info_cli_decoded_{}.bin", std::process::id()));

    let input_data = b"cli hide_as_img encode decode test";
    fs::write(&input_file, input_data).expect("write input file");

    let exe = PathBuf::from(env!("CARGO_BIN_EXE_hide_info"));
    let status = Command::new(&exe)
        .args([
            "hide_as_img",
            "encode",
            "--input",
            input_file.to_str().unwrap(),
            "--output",
            encoded_file.to_str().unwrap(),
        ])
        .status()
        .expect("run encode command");
    assert!(status.success());

    let status = Command::new(&exe)
        .args([
            "hide_as_img",
            "decode",
            "--input",
            encoded_file.to_str().unwrap(),
            "--output",
            decoded_file.to_str().unwrap(),
        ])
        .status()
        .expect("run decode command");
    assert!(status.success());

    let decoded_data = fs::read(decoded_file).expect("read decoded output");
    assert_eq!(decoded_data, input_data);
}
