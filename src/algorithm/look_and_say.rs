type DigitCounter = (char, usize);

#[allow(dead_code, unused_variables)]
fn look_and_say(_num_to_look_at: &String) -> String {
    if _num_to_look_at.is_empty() {
        return String::from("");
    }

    let chars_in_number = _num_to_look_at.chars();
    let first_char_in_number = chars_in_number.clone().take(1).last().unwrap();

    let mut digit_tracker = vec![DigitCounter::from((first_char_in_number, 1))];

    for byte in chars_in_number.skip(1) {
        let mut last_digit_counter = digit_tracker.last_mut().unwrap();

        let last_digit = last_digit_counter.0;

        if last_digit != byte {
            digit_tracker.push(DigitCounter::from((byte, 1)));
        } else {
            last_digit_counter.1 += 1;
        }
    }

    return digit_tracker
        .iter()
        .fold(String::from(""), |accumulator, current_digit_counter| {
            let digit = current_digit_counter.0;
            let count_of_digit = current_digit_counter.1;

            format!("{}{}{}", accumulator, count_of_digit, digit)
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
    #[test_case("111221", "312211"; "when input is 111221, expect output of 312211")]
    #[test_case("312211", "13112221"; "when input is 312211, expect output of 13112221")]
    fn test_harness(input: &str, expected: &str) {
        let converted_input = String::from(input);
        let converted_expected = String::from(expected);

        let actual = look_and_say(&converted_input);

        assert_eq!(&converted_expected, &actual);
    }
}
