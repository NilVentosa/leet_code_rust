use std::collections::HashMap;

fn main() {}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> 
{
    let mut old: HashMap<i32,i32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        match old.get(num) {
             Some(&index) => return vec![index, i as i32],
             None => old.insert(target - num, i as i32),
         };
    }

    vec![]
}

#[cfg(test)]
 mod tests {
    use crate::two_sum;
     #[test]
     fn one() {
         let result = vec![3,4];
         assert_eq!(result, two_sum(vec![1,2,3,4,5], 9))
     }

     #[test]
     fn two() {
         let result: Vec<i32> = vec![];
         assert_eq!(result, two_sum(vec![1,3,9,5], 9))
     }

     #[test]
     fn three() {
         let result: Vec<i32> = vec![0,3];
         assert_eq!(result, two_sum(vec![1,2,9,8], 9))
     }

 }
