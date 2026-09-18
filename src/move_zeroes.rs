fn main(){}

fn move_zeroes(nums: &mut Vec<i32>) {
    let src_len = nums.len();
    nums.retain(|&x| x != 0);
    for i in 0..src_len-nums.len() {
        nums.push(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_zeroes() {
        let mut vec1 = vec![0,1,0,3,12];
        move_zeroes(&mut vec1);
        assert_eq!(vec1, vec![1,3,12,0,0]);

        let mut vec2 = vec![0,0,1];
        move_zeroes(&mut vec2);
        assert_eq!(vec2, vec![1,0,0]);
        
        let mut vec3 = vec![1,2,3];
        move_zeroes(&mut vec3);
        assert_eq!(vec3, vec![1,2,3]);
    }
}
