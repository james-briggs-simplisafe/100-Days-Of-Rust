pub fn next_prime(n: i32) -> i32 {
    let mut primes = Vec::<i32>::new();
    for i in 2..i32::MAX {
        //println!("Primes found: {:?}", primes);
        let mut is_composite = false;
        for p in &primes {
            if i % p == 0 {
                is_composite = true;
                break;
            }
        }
        if is_composite {
            continue;
        }
    
        if i > n {
                return i;
            } 
                primes.push(i);
       }
    
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(next_prime(1), 2);
    }

    #[test]
    fn test_thirteen() {
        assert_eq!(next_prime(13), 17);
    }

    #[test]
    fn test_ninety_seven() {
        assert_eq!(next_prime(97), 101);
    }

    #[test]
    fn test_negative_two() {
        assert_eq!(next_prime(-2), 2);
    }
}