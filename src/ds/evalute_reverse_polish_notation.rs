// question 150
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut num_stack = vec![];

    for token in tokens {
        let v;
        match &token[..] {
            "+" => {
                v = num_stack.pop().unwrap() + num_stack.pop().unwrap();
            }
            "-" => {
                let r = num_stack.pop().unwrap();
                let l = num_stack.pop().unwrap();
                v = l - r;
            }
            "*" => {
                v = num_stack.pop().unwrap() * num_stack.pop().unwrap();
            }
            "/" => {
                let r = num_stack.pop().unwrap();
                let l = num_stack.pop().unwrap();
                v = l / r;
            }
            x => v = x.parse().unwrap(),
        }
        num_stack.push(v);
    }

    num_stack.pop().unwrap()
}
