use juec::frontend::parser::{JueParser, Rule};
use pest::Parser;

#[test]
fn debug_binary_operators_parsing() {
    let test_cases = vec![
        "1 + 2", "3 - 4", "5 * 6", "7 / 8", "9 == 10", "11 != 12", "13 < 14", "15 > 16",
        "17 <= 18", // This one is failing
        "19 >= 20",
    ];

    for test_case in test_cases {
        let pairs = JueParser::parse(Rule::program, test_case);
        match pairs {
            Ok(_) => println!("✓ Parsed successfully: {}", test_case),
            Err(e) => println!("✗ Failed to parse '{}': {:?}", test_case, e),
        }
    }
}
