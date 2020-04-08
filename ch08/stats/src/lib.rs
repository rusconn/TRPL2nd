use std::collections::HashMap;

pub fn mean(nums: &[i32]) -> f64 {
    nums.iter().sum::<i32>() as f64 / nums.len() as f64
}

pub fn median(nums: &[i32]) -> i32 {
    let mut vec = nums.to_owned();
    vec.sort();
    vec[vec.len() / 2]
}

pub fn mode(nums: &[i32]) -> i32 {
    let freqs = nums.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });
    freqs.into_iter().max_by_key(|pair| pair.1).unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        let nums = [3, 1, 4, 1, 5];
        assert_eq!(mean(&nums), 2.8);
    }

    #[test]
    fn test_median() {
        let nums = [3, 1, 4, 1, 5];
        assert_eq!(median(&nums), 3);
    }

    #[test]
    fn test_mode() {
        let nums = [3, 1, 4, 1, 5];
        assert_eq!(mode(&nums), 1);
    }
}
