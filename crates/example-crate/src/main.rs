fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn fake_test_for_ci_until_we_get_real_ones() {
        assert_eq!(1 + 1, 2);
    }
}
