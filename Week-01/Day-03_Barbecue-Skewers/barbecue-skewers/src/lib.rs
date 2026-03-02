
fn count_skewers(skewers: &[&str]) -> (i32, i32) {
    let mut veg = 0;
    let mut meat = 0;

    for &skewer in skewers {
        if skewer.contains("x") {
            meat += 1;
        } else {
            veg += 1;
        }
    }

    return (veg, meat);
        
}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn two_and_three() {
        let skewers = [
            "--oooo-ooo--",
            "--xx--x--xx--",
            "--o---o--oo--",
            "--xx--x--ox--",
            "--xx--x--ox--"
        ];
        let counts = count_skewers(&skewers);
        assert_eq!(counts, (2, 3)); 
    }

    #[test]
    fn three_and_two() {
        let skewers = [
            "--oooo-ooo--",
            "--xxxxxxxx--",
            "--o---",
            "-o-----o---x--",
            "--o---o-----"
        ];
        let counts = count_skewers(&skewers);
        assert_eq!(counts, (3, 2)); 
    }

}

