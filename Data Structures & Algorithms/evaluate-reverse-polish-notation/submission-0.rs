impl Solution {
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    if tokens.len() == 0 {
        return 0;
    }

    let mut res_stack: Vec<i32> = vec![];
    for token in tokens {
        if is_numeric(&token) {
            res_stack.push(token.parse().unwrap());
            continue;
        }

        let right = res_stack.pop().unwrap();
        let left = res_stack.pop().unwrap();
        match token.as_str() {
            "*" => res_stack.push(left * right),
            "+" => res_stack.push(left + right),
            "-" => res_stack.push(left - right),
            "/" => res_stack.push(left / right),
            _ => {}
        }
    }
    res_stack.pop().unwrap()
}


}
fn is_numeric(s: &str) -> bool {
    s.trim().parse::<f64>().is_ok()
}
