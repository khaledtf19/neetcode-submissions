impl Solution {
   pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    if tokens.len() == 0 {
        return 0;
    }

    let mut res_stack: Vec<i32> = vec![];
    for token in tokens {
        match token.as_str() {
            "*" => {
                let right = res_stack.pop().unwrap();
                let left = res_stack.pop().unwrap();
                res_stack.push(left * right)
            }
            "+" => {
                let right = res_stack.pop().unwrap();
                let left = res_stack.pop().unwrap();
                res_stack.push(left + right)
            }
            "-" => {
                let right = res_stack.pop().unwrap();
                let left = res_stack.pop().unwrap();
                res_stack.push(left - right)
            }
            "/" => {
                let right = res_stack.pop().unwrap();
                let left = res_stack.pop().unwrap();
                res_stack.push(left / right)
            }
            _ => res_stack.push(token.parse().unwrap()),
        }
    }
    res_stack.pop().unwrap()
}


}

