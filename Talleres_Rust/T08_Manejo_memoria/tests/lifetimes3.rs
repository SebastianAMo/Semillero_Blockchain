
// TODO: modify only this function.

fn copy_and_return(vector: &mut Vec<String>, value: &str) -> String {
    let value_str = String::from(value);
    vector.push(value_str.clone());
    value_str
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lifetimes3 () {
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    );
}
}