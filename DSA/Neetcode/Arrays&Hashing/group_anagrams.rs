use std::collections::HashMap ;

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new() ;

    for s in strs {

        let mut chars: Vec<char> = s.chars().collect() ;    // "eat" = ["e", "a", "t"]
        chars.sort_unstable() ; // ["a", "e", "t"]
        let key = chars.into_iter().collect::<String>() ; // "aet"

        map.entry(key).or_insert(Vec::new()).push(s) ;  // if the key exists, example aet -> adding eat to it's value, else adding an empty Vec and pushing s as the first value for the key, after the fist value for the key is added we will skip .or_insert(Vec::new()) part and jump to .push()
    }

    map.into_values().collect()
}


fn main() {
    let strs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];

    let result = group_anagrams(strs);
    for group in result {
        println!("{:?}", group);
    }
}