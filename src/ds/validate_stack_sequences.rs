
// question 946
pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    let mut stack = Vec::new();

    let mut push_idx = 0;
    for i in 0..popped.len() {
        let out_v = popped[i];
        while stack.len() == 0 || *stack.last().unwrap() != out_v {
            if push_idx == pushed.len() {
                return false;
            }

            stack.push(pushed[push_idx]);
            push_idx += 1;
        }

        stack.pop();
    }

    true
}

// pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
//     let mut map = std::collections::HashMap::new();
//     for (i, v) in pushed.iter().enumerate() {
//         map.insert(*v, i);
//     }

//     for i in 0..popped.len() {
//         let idx = map.get(&popped[i]).unwrap();
//         let mut prev = idx;
//         for j in i + 1..popped.len() {
//             let curr = map.get(&popped[j]).unwrap();
//             if curr > idx {
//                 continue;
//             }

//             if curr > prev {
//                 return false;
//             }

//             prev = curr;
//         }
//     }
//     true
// }
