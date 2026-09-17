fn main() {}

fn merge_sorted(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut output:Vec<i32> = Vec::new();
    let mut cur_a:usize = 0;
    let mut cur_b:usize = 0;
    
    while a.len() != cur_a && b.len() != cur_b {
        if a[cur_a] < b[cur_b] {
            output.push(a[cur_a]);
            cur_a += 1;
        } else {
            output.push(b[cur_b]);
            cur_b += 1;
        }
    }

    while a.len() != cur_a {
        output.push(a[cur_a]);
        cur_a += 1;
    }

    while b.len() != cur_b {
        output.push(b[cur_b]);
        cur_b += 1;
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_sorted() {
        assert_eq!(merge_sorted(vec![1, 3, 5], vec![2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(merge_sorted(vec![1, 2, 2], vec![2, 3]), vec![1, 2, 2, 2, 3]);
    }
}
