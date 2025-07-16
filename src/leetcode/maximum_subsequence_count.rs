use std::char;
/// 2207. 字符串中最多数目的子序列
/// 给你一个下标从 0 开始的字符串 text 和另一个下标从 0 开始且长度为 2 的字符串 pattern ，两者都只包含小写英文字母。
/// 你可以在 text 中任意位置插入 一个 字符，这个插入的字符必须是 pattern[0] 或者 pattern[1] 。注意，这个字符可以插入在 text 开头或者结尾的位置。
/// 请你返回插入一个字符后，text 中最多包含多少个等于 pattern 的 子序列 。
/// 子序列 指的是将一个字符串删除若干个字符后（也可以不删除），剩余字符保持原本顺序得到的字符串。
///
#[derive(Debug)]
struct Subsequence {
    count: i64,
}

impl Subsequence {
    fn new(count: i64) -> Subsequence {
        Subsequence { count }
    }
}

pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
    let chars: Vec<char> = text.chars().collect::<Vec<_>>();
    let patterns: Vec<char> = pattern.chars().collect::<Vec<_>>();
    let one_char = patterns[0];
    let two_char = patterns[1];
    let mut ans = vec![];
    let mut two_count: i64 = 0;
    for i in 0..chars.len() {
        if chars[i] == two_char {
            two_count += 1;
        }
    }
    let mut two_count_copy = two_count;
    for i in 0..chars.len() {
        if chars[i] == two_char {
            two_count_copy -= 1;
        }
        if chars[i] == one_char {
            ans.push(Subsequence::new(two_count_copy));
        }
    }
    let mut max_count = 0;
    // 放最前面
    let mut one_count = 0;
    one_count += two_count;

    for sub in &ans {
        one_count += sub.count;
    }
    max_count = max_count.max(one_count);

    // 放最后面
    if ans.len() == 1 && ans[0].count == 1 && one_char == two_char {
        return 1;
    }
    let mut count = 0;
    for sub in &ans {
        count += sub.count + 1;
    }

    max_count = max_count.max(count);
    max_count as i64
}

#[test]
fn test_1() {
    assert_eq!(
        maximum_subsequence_count(String::from("abdcdbc"), String::from("ac")),
        4
    )
}

#[test]
fn test_2() {
    assert_eq!(
        maximum_subsequence_count(String::from("aabb"), String::from("ab")),
        6
    )
}
#[test]
fn test_3() {
    assert_eq!(
        maximum_subsequence_count(
            String::from("fwymvreuftzgrcrxczjacqovduqaiig"),
            String::from("yy")
        ),
        1
    )
}

#[test]
fn test_4() {
    assert_eq!(
        maximum_subsequence_count(
            String::from("iuvgbmteyivbfwuospxmmgzagfa"),
            String::from("ti")
        ),
        3
    )
}

#[test]
fn test_5() {
    assert_eq!(
        maximum_subsequence_count(
            String::from("vnedkpkkyxelxqptfwuzcjhqmwagvrglkeivowvbjdoyydnjrqrqejoyptzoklaxcjxbrrfmpdxckfjzahparhpanwqfjrpbslsyiwbldnpjqishlsuagevjmiyktgofvnyncizswldwnngnkifmaxbmospdeslxirofgqouaapfgltgqxdhurxljcepdpndqqgfwkfiqrwuwxfamciyweehktaegynfumwnhrgrhcluenpnoieqdivznrjljcotysnlylyswvdlkgsvrotavnkifwmnvgagjykxgwaimavqsxuitknmbxppgzfwtjdvegapcplreokicxcsbdrsyfpustpxxssnouifkypwqrywprjlyddrggkcglbgcrbihgpxxosmejchmzkydhquevpschkpyulqxgduqkqgwnsowxrmgqbmltrltzqmmpjilpfxocflpkwithsjlljxdygfvstvwqsyxlkknmgpppupgjvfgmxnwmvrfuwcrsadomyddazlonjyjdeswwznkaeaasyvurpgyvjsiltiykwquesfjmuswjlrphsdthmuqkrhynmqnfqdlwnwesdmiiqvcpingbcgcsvqmsmskesrajqwmgtdoktreqssutpudfykriqhblntfabspbeddpdkownehqszbmddizdgtqmobirwbopmoqzwydnpqnvkwadajbecmajilzkfwjnpfyamudpppuxhlcngkign"),
            String::from("rr")
        ),
        496
    )
}
