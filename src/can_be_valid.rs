/**
 *  2116. 判断一个括号字符串是否有效
 * 一个括号字符串是只由 '(' 和 ')' 组成 的 非空 字符串。如果一个字符串满足下面 任意 一个条件，那么它就是有效的：
 * 字符串为 ().
 * 它可以表示为 AB（A 与 B 连接），其中A 和 B 都是有效括号字符串。
 * 它可以表示为 (A) ，其中 A 是一个有效括号字符串。
 * 给你一个括号字符串 s 和一个字符串 locked ，两者长度都为 n 。locked 是一个二进制字符串，只包含 '0' 和 '1' 。对于 locked 中 每一个 下标 i ：
 */

pub fn can_be_valid(s: String, locked: String) -> bool {
    let mut stack = vec![];
    let mut locked = locked.chars().collect::<Vec<char>>();
    let mut s = s.chars().collect::<Vec<char>>();
    if s.len() % 2 == 1 {
        return false;
    }
    

}
