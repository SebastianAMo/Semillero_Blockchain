// I AM NOT DONE

fn square(num: i32) -> i32 {
    return num * num;
}

#[cfg(test)]
mod tests {
    use crate::square;
    #[test]
    fn call_function() {
        let answer = square(3);
        println!("The square of 3 is {}", answer);
    }
}