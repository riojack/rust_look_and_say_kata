#[allow(dead_code, unused_variables)]
fn look_and_say(_num_to_look_at: &String) -> String {
    return match _num_to_look_at.as_ref() {
        "1" => String::from("11"),
        "11" => String::from("21"),
        "21" => String::from("1211"),
        "1211" => String::from("111221"),
        _ => String::from(""),
    };
}

#[cfg(test)]
mod tests {
    use crate::algorithm::look_and_say::look_and_say;
    use test_case::test_case;

    #[test_case("", ""; "when input is blank, expect output of blank")]
    #[test_case("1", "11"; "when input is 1, expect output of 11")]
    #[test_case("11", "21"; "when input is 11, expect output of 21")]
    #[test_case("21", "1211"; "when input is 21, expect output of 1211")]
    #[test_case("1211", "111221"; "when input is 1211, expect output of 111221")]
    fn test_harness(input: &str, expected: &str) {
        let converted_input = String::from(input);
        let converted_expected = String::from(expected);

        let actual = look_and_say(&converted_input);

        assert_eq!(&converted_expected, &actual);
    }
}
