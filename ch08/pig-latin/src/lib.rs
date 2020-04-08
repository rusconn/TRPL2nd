pub fn pig_latin(word: &str) -> String {
    let mut chars = word.chars();
    let first = chars.next().unwrap();

    if "aeiou".contains(first) {
        format!("{}-hay", word)
    } else {
        format!("{}-{}ay", chars.as_str(), first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pig_latin_vowel() {
        let word = "apple";
        assert_eq!(pig_latin(&word), "apple-hay");
    }

    #[test]
    fn test_pig_latin_consonant() {
        let word = "first";
        assert_eq!(pig_latin(&word), "irst-fay");
    }
}
