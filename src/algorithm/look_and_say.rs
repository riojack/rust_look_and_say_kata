type DigitCounter = (char, i64);

#[allow(dead_code, unused_variables)]
fn look_and_say(_num_to_look_at: &String) -> String {
    let mut tracker = vec![DigitCounter::from((' ', 0))];

    for byte in _num_to_look_at.chars() {
        let mut last_digit_counter = tracker.last_mut().unwrap();

        let last_digit = last_digit_counter.0;

        if last_digit != byte {
            tracker.push(DigitCounter::from((byte, 1)));
        } else {
            last_digit_counter.1 += 1;
        }
    }

    return tracker
        .iter()
        .filter(|digit_counter| digit_counter.1 > 0)
        .fold(String::from(""), |accumulator, current_digit_counter| {
            format!(
                "{}{}{}",
                accumulator, current_digit_counter.1, current_digit_counter.0
            )
        });
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
