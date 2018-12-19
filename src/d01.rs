pub fn calibrate(values: &[i32]) -> i32 {
    values.iter().sum()
}

pub fn first_duplicate_frequency(values: &[i32]) -> i32 {
    use std::collections::HashSet;

    let mut current_frequency = 0i32;
    let mut frequencies = HashSet::new();

    for value in values.iter().cycle() {
        frequencies.insert(current_frequency);
        current_frequency += value;
        if frequencies.contains(&current_frequency) {
            break;
        }
    }

    current_frequency
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibrate() {
        assert_eq!(3, calibrate(&[1, 1, 1]));
        assert_eq!(0, calibrate(&[1, 1, -2]));
        assert_eq!(-6, calibrate(&[-1, -2, -3]));
    }

    #[test]
    fn test_first_duplicate_frequency() {
        assert_eq!(0, first_duplicate_frequency(&[1, -1]));
        assert_eq!(10, first_duplicate_frequency(&[3, 3, 4, -2, -4]));
        assert_eq!(5, first_duplicate_frequency(&[-6, 3, 8, 5, -6]));
        assert_eq!(14, first_duplicate_frequency(&[7, 7, -2, -7, -4]));
    }
}
