/// 2300. 咒语和药水的成功对数
pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut potions_ans = potions.clone();
    potions_ans.sort();
    let mut ans = vec![];
    for s in spells {
        let total: i32 = potions_ans.len() as i32
            - binarySearch(&potions_ans, ((success - 1) / (s as i64)) as i32);
        ans.push(total);
    }
    ans
}

fn binarySearch(nums: &Vec<i32>, target: i32) -> i32 {
    let mut low = 0;
    let mut high: i32 = nums.len() as i32 - 1;
    let mut res = nums.len() as i32;

    while low <= high {
        let mid = low + ((high - low) >> 1);
        if nums[mid as usize] > target {
            res = mid;
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    res as i32
}

#[test]
fn test() {
    assert_eq!(
        successful_pairs(vec![15, 8, 19], vec![38, 36, 23], 328),
        vec![3, 0, 3]
    )
}
