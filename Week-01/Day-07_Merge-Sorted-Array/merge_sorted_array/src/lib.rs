pub fn merge_arrays(arr1: &mut [i32], arr2: &[i32], m: usize) {
    let n = arr2.len();

    if m + n > arr1.len() {
        panic!("Array 1 is not large enough to add array 2!")
    }

    // Move elements to back of array 1
    for i in (0..m).rev() {
        arr1[n+i] = arr1[i];
    }

    let mut arr1_head = n;
    let mut arr2_head = 0usize;

    for i in 0..(m+n) {
        if arr2_head >= n || arr1_head < (m+n) && arr1[arr1_head] < arr2[arr2_head] {
            arr1[i] = arr1[arr1_head];
            arr1_head += 1;
        } else {
            arr1[i] = arr2[arr2_head];
            arr2_head += 1;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_readme() {
        let mut nums1 = [1,2,3,0,0,0];
        let m = 3;
        let nums2 = [2,5,6];
        merge_arrays(&mut nums1, &nums2, m);
        assert_eq!(
            nums1,
            [1,2,2,3,5,6]
        );
    }

    #[test]
    fn test_exhaust_1() {
        let mut nums1 = [1,0,0,0];
        let m = 1;
        let nums2 = [2,5,6];
        merge_arrays(&mut nums1, &nums2, m);
        assert_eq!(
            nums1,
            [1,2,5,6]
        );
    }


        #[test]
    fn test_exhaust2() {
        let mut nums1 = [5,6,7,0,0];
        let m = 3;
        let nums2 = [1,2];
        merge_arrays(&mut nums1, &nums2, m);
        assert_eq!(
            nums1,
            [1,2,5,6,7]
        );
    }

    #[test]
    #[should_panic]
    fn test_panics_when_not_enough_space() {
                let mut nums1 = [5,6,7,0,0];
        let m = 3;
        let nums2 = [1,2,3];
        merge_arrays(&mut nums1, &nums2, m);
    }
}
