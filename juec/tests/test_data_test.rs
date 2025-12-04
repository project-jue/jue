#[test]
fn test_test_data_crate_works() {
    // Test that the test_data crate is accessible and works
    let data_dir = test_data::data_dir();
    println!("Data directory: {:?}", data_dir);

    // Test that we can access a file
    let test_file = data_dir.join("shared_samples/phase_1_parsing/01_arithmetic_expressions.jue");
    println!("Test file path: {:?}", test_file);

    // Test that the file exists
    assert!(test_file.exists(), "Test file should exist");

    // Test that we can read the file
    let content = std::fs::read_to_string(test_file).expect("Should be able to read test file");
    println!("File content length: {}", content.len());
    assert!(!content.is_empty(), "File should not be empty");
}
