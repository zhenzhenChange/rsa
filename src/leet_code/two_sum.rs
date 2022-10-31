use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cache = HashMap::new();

        for i in 0..nums.len() {
            if let Some(v) = cache.get(&(target - nums[i])) {
                vec![*v, i as i32];
            }

            cache.insert(nums[i], i as i32);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        let result = Solution::two_sum(nums, target);

        assert_eq!(result, vec![0, 1], "output: {:?}", result);
    }

    #[test]
    fn test2() {
        let nums = vec![3, 2, 4];
        let target = 6;

        let result = Solution::two_sum(nums, target);

        assert_eq!(result, vec![1, 2], "output: {:?}", result);
    }

    #[test]
    fn test3() {
        let nums = vec![3, 3];
        let target = 6;

        let result = Solution::two_sum(nums, target);

        assert_eq!(result, vec![0, 1], "output: {:?}", result);
    }
}
