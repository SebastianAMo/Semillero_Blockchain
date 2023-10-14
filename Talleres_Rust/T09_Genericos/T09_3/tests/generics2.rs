// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.


// I AM NOT DONE

struct Wrapper<T> { // add generic type
    value: T, // change to generic type
}


impl <T> Wrapper <T>{ // add generic type
    pub fn new(value: T) -> Self {//change to generic type
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
