pub fn count_words(str: String) -> i32 {
    let mut result = 0;

    str.split_whitespace().for_each(|_| result += 1);

    result
}

#[cfg(test)]
pub mod test {
    use crate::utils::utils::count_words;

    #[test]
    fn testauthorize_count_word() {
        assert_eq!(3, count_words("test test test".to_string()))
    }
}
