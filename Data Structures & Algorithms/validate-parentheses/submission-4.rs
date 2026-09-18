impl Solution {
   pub fn is_valid(s: String) -> bool {
        if s.len() == 1 {
        return false;
    }

    let mut res = "".to_string();
    let c = [']', ')', '}'];
    for ch in s.chars() {
        if c.contains(&ch) {
            let Some(curr) = res.pop() else { return false };

            if curr == '(' && ch == ')' {
                continue;
            } else if curr == '[' && ch == ']' {
                continue;
            } else if curr == '{' && ch == '}' {
                continue;
            } else {
                return false;
            }
        } else {
            res.push(ch);
        }
    }
        if res.is_empty() {
        return true;
    }
    false

}

}
