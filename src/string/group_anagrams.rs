// question 49

// best solution
//
// use std::collections::HashMap;
//
// impl Solution {
//     pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
//         let mut hash_group = HashMap::new();
//         for s in strs.into_iter() {
//             let mut hash_key = [0; 26];
//
//             for c in s.chars() {
//                 hash_key[(c as u32 - 'a' as u32) as usize] += 1;
//             }
//
//             hash_group.entry(hash_key).or_insert(Vec::new()).push(s);
//         }
//
//         hash_group.into_iter().map(|(_, group)| group).collect()
//     }
// }
//
use std::collections::HashMap;
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<&str>> = HashMap::new();
    strs.iter().for_each(|x| {
        let mut ordered = x.chars().collect::<Vec<_>>();
        ordered.sort();
        let ordered: String = ordered.into_iter().collect();
        map.entry(ordered).or_insert(vec![]).push(&x);
    });

    map.values()
        .map(|v| v.iter().map(|s| String::from(*s)).collect::<Vec<String>>())
        .collect()
}
