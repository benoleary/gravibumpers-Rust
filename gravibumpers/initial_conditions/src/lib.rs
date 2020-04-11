pub fn hold_place(input_int: i32) -> i32 {
    println!(
        "initial_conditions::hold_place(input_int = {input_int})",
        input_int = input_int
    );
    123
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_placeholder() {
        let placeholder_value = hold_place(0);
        assert_eq!(
            123, placeholder_value,
            "placeholder test, left is expected, right is actual"
        );
    }
}
