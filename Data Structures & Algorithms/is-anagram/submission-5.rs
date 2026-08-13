impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut s_map = std::collections::HashMap::new();
        let mut t_map = std::collections::HashMap::new();
        for ch in s.chars() {
            s_map
                .entry(ch)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        for ch in t.chars() {
            t_map
                .entry(ch)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        for (key, value) in s_map.iter() {
            if let Some(sec_value) = t_map.get(key) {
                if sec_value != value {
                    return false;
                }
            }else{
                return false;
            }
        }
        true
    }
}
