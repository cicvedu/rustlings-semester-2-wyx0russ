





mod Foo {
    // No `extern` is needed; it's all Rust.
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let result1 = Foo::my_demo_function(123);
        let result2 = Foo::my_demo_function(456);

        // You don't need `unsafe` here, as these functions are safe Rust functions.
        assert_eq!(result1, 123);
        assert_eq!(result2, 456);
    }
}
