use super::*;

#[test]
/// Tests if the `split_mana_string` function works as expected.
fn test_split_mana_string() {
    { // Test a correct string.
        let correct_mana_string = "{2}{R}{U}";
        assert_eq!(
            split_mana_string(correct_mana_string),
            vec!("{2}", "{R}", "{U}")
        );
    } { // Test an incorrect string.
        let incorrect_mana_string = "test{2}{{R}}{U}fails";
        assert_eq!(
            split_mana_string(incorrect_mana_string),
            vec!("test{2}", "{{R}", "}", "{U}", "fails")
        );
    } { // Test an empty string.
        let empty_mana_string = "";
        assert_eq!(
            split_mana_string(empty_mana_string),
            vec!("")
        );
    }
}
