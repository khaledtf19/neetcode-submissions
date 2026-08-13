impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for name in strs {
        let mut sorted: Vec<char> = name.chars().collect();
        sorted.sort_unstable();
        let sorted: String = sorted.into_iter().collect();
        map.entry(sorted)
            .and_modify(|list| list.push(name.clone()))
            .or_insert(vec![name]);
    }
    map.into_iter().map(|(_, list)| list).collect()
}

}
