fn main() {}

fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut new_vec:Vec<i32> = digits.clone();
    let mut cur:usize = new_vec.len()-1;

    loop {
        if new_vec[cur] < 9 {
            new_vec[cur] += 1;
            break;
        } else {
            new_vec[cur] = 0;
            if cur == 0 {
                new_vec.insert(0, 1);
            }
        }
        
        if cur > 0 {
            cur -= 1;
        } else {
            break;
        }
    }

    new_vec
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        assert_eq!(plus_one(vec![1,2,3]), vec![1,2,4]);
        assert_eq!(plus_one(vec![4,3,2,1]), vec![4,3,2,2]);
        assert_eq!(plus_one(vec![9]), vec![1,0]);
        assert_eq!(plus_one(vec![9,9,9]), vec![1,0,0,0]);
    }
}
