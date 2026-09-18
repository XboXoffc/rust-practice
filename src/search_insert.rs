fn main() {  }

fn search_insert(nums: &[i32], target: i32) -> usize {
    let mut output: usize = 0;
    for (index, &num) in nums.iter().enumerate() {
        if target <= num {
            break;
        }
        output = index + 1;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert() {
        assert_eq!(search_insert(&[1, 3, 5, 6], 5), 2);
        assert_eq!(search_insert(&[1, 3, 5, 6], 2), 1);
        assert_eq!(search_insert(&[1, 3, 5, 6], 7), 4);
        assert_eq!(search_insert(&[1, 3, 5, 6], 0), 0);
    }
}
