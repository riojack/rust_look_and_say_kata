fn hello_world() {
    println!("Hello, world!")
}

#[cfg(test)]
mod tests {
    #[test]
    fn two_equals_two() {
        assert_eq!(2, 2);
    }
}
