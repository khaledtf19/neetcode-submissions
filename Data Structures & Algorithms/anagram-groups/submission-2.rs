impl Solution {
 pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<[u32; 26], Vec<String>> = HashMap::new();
    for name in strs {
        let mut count = [0; 26];
        for ch in name.chars() {
            count[ch as usize - 'a' as usize] += 1;
        }
        map.entry(count).or_default().push(name);
    }
    map.into_values().collect()
}


}
