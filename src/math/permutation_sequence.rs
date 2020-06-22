// question 60

// 1. question
// The set [1,2,3,...,n] contains a total of n! unique permutations.

// By listing and labeling all of the permutations in order, we get the following sequence for n = 3:

//     "123"
//     "132"
//     "213"
//     "231"
//     "312"
//     "321"

// Given n and k, return the kth permutation sequence.

// 2. My solution
pub fn get_permutation(n: i32, k: i32) -> String {
    let mut result = String::new();
    let mut vals: Vec<i32> = (1..n + 1).collect();

    let c = k;
    for _ in 1..=n {
        let s = get_kth(&mut vals, c);
        result.push_str(&s);
    }

    result
}

fn get_kth(vals: &mut Vec<i32>, k: i32) -> String {
    if vals.len() == 1 {
        return vals.remove(0).to_string();
    }

    if vals.len() == 2 {
        if k % 2 == 0 {
            return vals.remove(1).to_string();
        }
        return vals.remove(0).to_string();
    }

    let group_num = fac(vals.len() - 1);
    let index = (k - 1) as usize / group_num % vals.len();

    vals.remove(index).to_string()
}

fn fac(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    n * fac(n - 1)
}

// 3. Other's solution
// public class Solution {
//     public String getPermutation(int n, int k) {
//         StringBuilder sb = new StringBuilder();
//         ArrayList<Integer> num = new ArrayList<Integer>();
//         int fact = 1;
//         for (int i = 1; i <= n; i++) {
//             fact *= i;
//             num.add(i);
//         }
//         for (int i = 0, l = k - 1; i < n; i++) {
//             fact /= (n - i);
//             int index = (l / fact);
//             sb.append(num.remove(index));
//             l -= index * fact;
//         }
//         return sb.toString();
//     }
// }

// string getPermutation(int n, int k) {
//     int i,j,f=1;
//     // left part of s is partially formed permutation, right part is the leftover chars.
//     string s(n,'0');
//     for(i=1;i<=n;i++){
//         f*=i;
//         s[i-1]+=i; // make s become 1234...n
//     }
//     for(i=0,k--;i<n;i++){
//         f/=n-i;
//         j=i+k/f; // calculate index of char to put at s[i]
//         char c=s[j];
//         // remove c by shifting to cover up (adjust the right part).
//         for(;j>i;j--)
//             s[j]=s[j-1];
//         k%=f;
//         s[i]=c;
//     }
//     return s;
// }

// impl Solution {
//     pub fn get_permutation(n: i32, mut k: i32) -> String {
//         // FACTORIAL of numbers from 0! to 9!
//         const FACTORIAL: [i32; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
//         let mut numbers = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];
//         let mut sequence = String::new();
//         // necessary
//         k = k - 1;

//         for i in 0..n {
//             let idx = (n - 1 - i) as usize;
//             let div = k / FACTORIAL[idx];

//             k = k % FACTORIAL[idx];
//             sequence.push(numbers.remove(div as usize));
//         }

//         sequence
//     }
// }

// 4. What can be improved
//  1) 计算 fac 可以不用函数
//  2) 算第 n 个的 index 的时候不必特殊处理 1 和 2

// 5. improve again
pub fn get_permutation_improved(n: i32, k: i32) -> String {
    let mut k = k - 1;
    let mut result = String::with_capacity(n as usize);

    let mut fact = (1..=n).fold(1, |acc, x| acc * x);
    let mut vals: Vec<i32> = (1..=n).collect();

    for i in 0..n {
        fact /= n - i;
        let index = k / fact;
        result.push_str(&vals.remove(index as usize).to_string());

        k = k % fact;
    }

    result
}

// 6. thinking
//  1) 观察规律来获得思路
//  2) 对自己的思路一定要有清晰的认知, 不要依靠 debug 来帮助自己完善思路,
//     而是应该直接考虑清楚其中的逻辑
//  3) 积累 index 从 0 或 从 1 开始的区别和经验
//      这里算第 k 个计算 index 先转换为从 0 开始的一般没什么问题
