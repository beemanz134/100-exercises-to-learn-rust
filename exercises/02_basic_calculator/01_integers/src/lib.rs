fn compute(a: u8, b: u8) -> u8 {
    // TODO: change the line below to fix the compiler error and make the tests pass.
    a + b * 4u8
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
    }
}
