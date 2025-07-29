/// 71. 简化路径
/// 

pub fn simplify_path(path: String) -> String {
    let paths = path.split("/").collect::<Vec<_>>();
    let mut stack = vec![];
    for p in paths.iter() {
        if p.is_empty() {
            continue;
        }
        if p == &"/" {
            continue;
        } else if p == &"." {
            continue;
        } else if p == &".." {
            stack.pop();
        } else {
            stack.push(*p);
        }
    }

    println!("{:?}", stack);

    if stack.len() == 0 {
        return "/".to_string();
    }
    
    let mut ans = String::new();
    for s in stack.iter() {
        ans = format!("{}/{}",ans, s);
    }
    ans
}



#[test]
fn test_1() {
    assert_eq!(simplify_path("/home/".to_string()), "/home")
}
#[test]
fn test_2() {
    assert_eq!(simplify_path("/home/user/Documents/../Pictures".to_string()), "/home/user/Pictures")
}
#[test]
fn test_3() {
    assert_eq!(simplify_path("/home/./../".to_string()), "/")
}
#[test]
fn test_4() {
    assert_eq!(simplify_path("/../".to_string()), "/")
}
#[test]
fn test_5() {
    assert_eq!(simplify_path("/.../a/../b/c/../d/./".to_string()), "/.../b/d")
}
