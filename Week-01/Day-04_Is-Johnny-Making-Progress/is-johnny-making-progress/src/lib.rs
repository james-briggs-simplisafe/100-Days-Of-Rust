pub fn progress_days(days: &[i32]) -> i32 {
    let mut count = 0; 
    for day in 1..days.len() {
        if days[day] > days[day-1] {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_tests() {
        assert_eq!(progress_days(&[3, 4, 1, 2]), 2);
        assert_eq!(progress_days(&[10, 11, 12, 9, 10]), 3);
        assert_eq!(progress_days(&[6, 5, 4, 3, 2, 9]), 1);
        assert_eq!(progress_days(&[9, 9]), 0);
    }
}
