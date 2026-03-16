use std::cmp::min;


/// Find the indices of local maxima in the input array.
fn get_local_arg_maxima(heights: &Vec<i32>) -> Vec<usize> {
    let mut arg_maxima = Vec::<usize>::new();

    let n = heights.len();
    for i in 0..n {
        if (i == 0 || heights[i] >= heights[i-1]) && 
           (i == n -1 || heights[i] >= heights[i+1]) {
            arg_maxima.push(i);
        }
    }
    arg_maxima
}

pub fn get_rainwater_trapped(heights: &Vec<i32>) -> i32 {
    let local_maxima = get_local_arg_maxima(heights);

    let mut units_captured = 0;

    for i in 0..local_maxima.len() - 1 {
        let start = local_maxima[i];
        let end = local_maxima[i+1];

        let water_level = min(heights[start], heights[end]);
        for j in start+1..end {
            units_captured += water_level - heights[j];
        }
    }

    units_captured

}

#[cfg(test)]
mod tests {
     use super::*;

    #[test]
    fn test_argmax_with_one_max() {
        let terrain = vec![1,2,3,2,1];
        let max_indices = get_local_arg_maxima(&terrain);
        assert_eq!(max_indices, vec![2]);
    }

    #[test]
    fn test_argmax_with_first_and_last() {
        let terrain = vec![3,2,1,2,3];
        let max_indices = get_local_arg_maxima(&terrain);
        assert_eq!(max_indices, vec![0,4]);
    }

    #[test]
    fn test_argmax_with_constant_terrain() {
        let terrain = vec![1,1,1,1,1];
        let max_indices = get_local_arg_maxima(&terrain);
        assert_eq!(max_indices, vec![0,1,2,3,4]);
    }

    #[test]
    fn test_water_captured_with_mouintain() {
        let terrain = vec![1,2,3,2,1];
        let captured = get_rainwater_trapped(&terrain);
        assert_eq!(captured, 0);
    }

        #[test]
    fn test_water_captured_with_bowl() {
        let terrain = vec![3,2,1,2,3];
        let captured = get_rainwater_trapped(&terrain);
        assert_eq!(captured, 4);
    }

    #[test]
    fn test_water_captured_with_constant_terrain() {
        let terrain = vec![1,1,1,1,1];
        let captured = get_rainwater_trapped(&terrain);
        assert_eq!(captured, 0);
    }

    #[test]
    fn test_water_captured_with_readme_example() {
        let terrain = vec![0,1,0,2,1,0,1,3,2,1,2,1];
        let captured = get_rainwater_trapped(&terrain);
        assert_eq!(captured, 6);
    }

}
