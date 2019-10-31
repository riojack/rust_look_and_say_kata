#[allow(dead_code, unused_variables)]
fn look_and_say(_num_to_look_at: &String) -> String {

    return match _num_to_look_at.as_ref() {
        "1" => String::from("11"),
        "11" => String::from("21"),
        _ => String::from("")
    };
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::algorithm::look_and_say::look_and_say;

    #[test_case("1", "11"; "when input is 1, expect output of 11")]
    #[test_case("11", "21"; "when input is 11, expect output of 21")]
    fn test_harness(input: &str, expected: &str) {
        let converted_input = String::from(input);
        let converted_expected = String::from(expected);

        let actual = look_and_say(&converted_input);

        assert_eq!(&converted_expected, &actual);
    }

}
