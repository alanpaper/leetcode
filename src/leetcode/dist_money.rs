use std::ops::Div;

/// 2591. 分钱
///
pub fn dist_money(money: i32, children: i32) -> i32 {
    if money < children {
        return -1;
    }
    if money < children + 7 {
        return 0;
    }

    let m_num = (money - children) / 7;
    let m_num_mod = (money - children) % 7;

    println!("{:?}--{:?}", m_num, m_num_mod);

    if m_num > children {
      return children - 1;
    }

    if m_num < children {
      if children - m_num == 1 && m_num_mod == 3 {
        return m_num - 1;
      }
      return m_num;
    }

    if m_num == children && m_num_mod != 0 {
      return m_num - 1;
    }

    m_num

}

#[test]
fn _test() {
    assert_eq!(dist_money(20, 3), 1);
}
