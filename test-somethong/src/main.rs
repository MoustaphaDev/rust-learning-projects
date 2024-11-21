fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left as i32 <= right as i32 {
        let mid = (left + right) / 2;
        if nums[mid] < target {
            left = mid + 1;
        } else if nums[mid] > target {
            right = mid - 1;
        } else {
            return mid as i32;
        }
    }
    return -1;
}

fn main() {
    let nums: Vec<i32> = vec![-1, 0, 3, 4, 9, 12];
    let res = search(nums, 9);
    println!("{}", res);

    let nums: Vec<i32> = vec![5];
    let res = search(nums, -5);
    println!("{}", res);
}
