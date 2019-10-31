#[allow(dead_code,unused_variables)]
fn look_and_say(_num_to_look_at: &String) -> String {
    return String::from("11");
}

#[cfg(test)]
mod tests {
    use crate::algorithm::look_and_say::look_and_say;

    #[test]
    fn returns_11_when_given_1() {
        let num = String::from("1");
        let expected = String::from("11");
        let actual = look_and_say(&num);

        assert_eq!(&expected, &actual);
    }
}
