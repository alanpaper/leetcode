use std::collections::{HashMap, HashSet};
/// 721. 账户合并
/// 给定一个列表 accounts，每个元素 accounts[i] 是一个字符串列表，
/// 其中第一个元素 accounts[i][0] 是 名称 (name)，其余元素是 emails 表示该账户的邮箱地址。
/// 现在，我们想合并这些账户。如果两个账户都有一些共同的邮箱地址，则两个账户必定属于同一个人。
/// 请注意，即使两个账户具有相同的名称，它们也可能属于不同的人，
/// 因为人们可能具有相同的名称。一个人最初可以拥有任意数量的账户，但其所有账户都具有相同的名称。
/// 合并账户后，按以下格式返回账户：每个账户的第一个元素是名称，其余元素是 按字符 ASCII 顺序排列 的邮箱地址。
/// 账户本身可以以 任意顺序 返回。
pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut email_map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut email_name_map: HashMap<String, String> = HashMap::new();
    
    for (i, account) in accounts.iter().enumerate() {
        let user_name = format!("{}+{}", account[0], i);
        let mut emails = HashSet::new();
        let mut can_merge = false;
        let mut pre_name = String::new();
        
        // 第一遍检查是否有可以合并的邮箱
        for email in account.iter() {
            emails.insert(email.clone());
            if let Some(existing_name) = email_name_map.get(email) {
                can_merge = true;
                pre_name = existing_name.clone();
                break;
            }
        }
        
        // 第二遍更新映射关系
        for email in account.iter() {
            if can_merge {
                email_name_map.insert(email.clone(), pre_name.clone());
                email_map.entry(pre_name.clone())
                    .and_modify(|f| { f.insert(email.clone()); });
            } else {
                email_name_map.insert(email.clone(), user_name.clone());
            }
        }
        
        if !can_merge {
            email_map.insert(user_name, emails);
        }
    }

    let mut ans = vec![];

    for email in email_map.iter() {
        let name = email.0.split("+").collect::<Vec<_>>()[0];
        let mut a = vec![];
        a.push(String::from(name));
        let mut result = email.1.iter().collect::<Vec<_>>();
        result.sort();
        for e in result {
            a.push(e.to_string());
        }
        ans.push(a);
    }

    ans
}

#[test]
fn test_1() {
    let accounts = vec![
        vec![
            "John".to_string(),
            "johnsmith@mail.com".to_string(),
            "john00@mail.com".to_string(),
        ],
        vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
        vec![
            "John".to_string(),
            "johnsmith@mail.com".to_string(),
            "john_newyork@mail.com".to_string(),
        ],
        vec!["Mary".to_string(), "mary@mail.com".to_string()],
    ];
    let ans = vec![
        vec![
            "John".to_string(),
            "john00@mail.com".to_string(),
            "john_newyork@mail.com".to_string(),
            "johnsmith@mail.com".to_string(),
        ],
        vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
        vec!["Mary".to_string(), "mary@mail.com".to_string()],
    ];
    assert_eq!(accounts_merge(accounts.clone()), ans);
}


#[test]
fn test_2() {
    let accounts = vec![
        vec!["David".to_string(),"David0@m.co".to_string(),"David1@m.co".to_string()],
        vec!["David".to_string(),"David3@m.co".to_string(),"David4@m.co".to_string()],
        vec!["David".to_string(),"David4@m.co".to_string(),"David5@m.co".to_string()],
        vec!["David".to_string(),"David2@m.co".to_string(),"David3@m.co".to_string()],
        vec!["David".to_string(),"David1@m.co".to_string(),"David2@m.co".to_string()]
    ];
    let ans = vec![
        vec![
            "David".to_string(),
            "David0@m.co".to_string(),
            "David1@m.co".to_string(),
            "David2@m.co".to_string(),
            "David3@m.co".to_string(),
            "David4@m.co".to_string(),
            "David5@m.co".to_string()
        ]
    ];
    assert_eq!(accounts_merge(accounts.clone()), ans);
}
