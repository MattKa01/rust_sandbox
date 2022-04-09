//leetcode "two-sum" problem

//Return two indexes which sum up to target

pub fn run(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];

    'outer: for y in 0..nums.len(){
        for x in 0..nums.len() {
            if nums[y] + nums[x] == target {
                res.push(nums[y]);
                res.push(nums[x]);
                break 'outer;
            }
      }
    }

    res
}