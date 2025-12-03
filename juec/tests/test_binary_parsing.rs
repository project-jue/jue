use juec::frontend::parser::{JueParser, Rule};
use pest::Parser;

#[test]
fn test_binary_operators_parsing() {
    let test_cases = vec![
        "1 + 2",
        "3 - 4",
        "5 * 6",
        "7 / 8",
        "9 == 10",
        "11 != 12",
        "13 < 14",
        "15 > 16",
        "17 <= 18",
        "19 >= 20",
        "2 + 3 * 4",
        "(2 + 3) * 4",
        "5 + 6 - 7 * 8",
        "(1 + 2) * (3 + 4)",
        "10 / 2 + 3",
        "10 / (2 + 3)",
    ];

    for test_case in test_cases {
        let pairs = JueParser::parse(Rule::program, test_case);
        assert!(pairs.is_ok(), "Failed to parse: {}", test_case);
    }
}
