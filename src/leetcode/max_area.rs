pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_total = 0;

    for h_index in 0..height.len() {
        for y_index in (h_index + 1)..height.len() {
            let area = ((y_index - h_index) as i32).abs() * (height[h_index].min(height[y_index]));
            if area > max_total {
                max_total = area;
            }
        }
    }

    max_total
}

pub fn max_area_one(height: Vec<i32>) -> i32 {
    let mut max_total = 0;
    let mut left_index = 0;
    let mut right_index = height.len() - 1;

    while left_index < right_index {
        let area = height[left_index].min(height[right_index])
            * (((right_index - left_index) as i32));

        if area > max_total {
            max_total = area;
        }

        if height[left_index] > height[right_index] {
            right_index -= 1;
        } else {
            left_index += 1;
        }
    }
    max_total
}

#[test]
fn test_1() {
    assert_eq!(max_area_one(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
}
