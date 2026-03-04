use counter::Counter;

pub fn sock_pairs(socks: &str) -> i32 {
    let sock_counts= socks.chars().collect::<Counter<_>>();
    
1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
