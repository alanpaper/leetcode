use std::collections::HashMap;

/// 给定一个包含红色、白色和蓝色、共 n 个元素的数组 nums ，原地 对它们进行排序，使得相同颜色的元素相邻，并按照红色、白色、蓝色顺序排列。
/// 我们使用整数 0、 1 和 2 分别表示红色、白色和蓝色。
/// 必须在不使用库内置的 sort 函数的情况下解决这个问题。

pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut color_map = HashMap::new();
    for n in nums.clone() {
        color_map.entry(n).and_modify(|f| *f+=1).or_insert(1);
    }
    let colors = [0, 1, 2];
    let mut next_index = 0;
    for c in colors {
        if let Some(num) = color_map.get(&c) {
            for n in next_index..next_index+num {
                nums[n] = c;
            }
            next_index = next_index + num;
        }
    }
}
