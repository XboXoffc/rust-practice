fn main() {}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> usize {
    nums.retain(|&x| x != val);
    nums.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut vec1 = vec![3, 2, 2, 3];
        let vec1_new = vec![2, 2];
        let mut vec2 = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let vec2_new = vec![0, 1, 3, 0, 4];

        assert_eq!(remove_element(&mut vec1, 3), 2);
        assert_eq!(vec1, vec1_new);
        assert_eq!(remove_element(&mut vec2, 2), 5);
        assert_eq!(vec2, vec2_new);
    }
}
