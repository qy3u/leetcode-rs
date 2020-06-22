// Question 31

// 1. question
// Implement next permutation, which rearranges numbers into the lexicographically next greater permutation of numbers.

// If such arrangement is not possible, it must rearrange it as the lowest possible order (ie, sorted in ascending order).
//
// The replacement must be in-place and use only constant extra memory.
//
// Here are some examples. Inputs are in the left-hand column and its corresponding outputs are in the right-hand column.
//
// 1,2,3 → 1,3,2
// 3,2,1 → 1,2,3
// 1,1,5 → 1,5,1
//

// 2. my solution
pub fn next_permutation(nums: &mut Vec<i32>) {
    if nums.len() <= 1 {
        return;
    }

    let len = nums.len();
    let mut un_order = nums.len();

    for i in (0..=len - 2).rev() {
        if nums[i] < nums[i + 1] {
            un_order = i;
            break;
        }
    }

    if un_order == nums.len() {
        nums.reverse();
        return;
    }

    for i in (un_order + 1..=nums.len() - 1).rev() {
        if nums[i] > nums[un_order] {
            nums.swap(i, un_order);
            nums[un_order + 1..len].reverse();
            return;
        }
    }
}

// 3. others solution
// void nextPermutation(vector<int> &num) {
//     if(num.size()<=1) return;
//     int i=num.size()-1,j;
//     for(j=num.size()-2; j>=0; j--){
//         if(num[j]<num[j+1]) break;
//     }
//     if(j>=0){
//         while(num[i]<=num[j]) i--;
//         swap(num[i], num[j]);
//     }
//     reverse(num.begin()+j+1, num.end());
// }

// void nextPermutation(vector<int> &num) {
//     if(num.size()<=1) return;
//     int i=num.size()-1,j;
//     for(j=num.size()-2; j>=0; j--){
//         if(num[j]<num[j+1]) break;
//     }
//     if(j>=0){
//         while(num[i]<=num[j]) i--;
//         swap(num[i], num[j]);
//     }
//     reverse(num.begin()+j+1, num.end());
// }

// 4. what can be improved
//  1) 可以考虑现有的库函数来实现部分逻辑
//  2) 最后的 reverse 逻辑

// 5. improved again

pub fn next_permutation_again(nums: &mut Vec<i32>) {
    if nums.len() <= 1 {
        return;
    }

    let mut un_order = nums.len();
    for i in (0..=nums.len() - 2).rev() {
        if nums[i] < nums[i + 1] {
            un_order = i;
            break;
        }
    }

    if un_order == nums.len() {
        nums.reverse();
        return;
    }

    let mut i = nums.len() - 1;
    while nums[i] <= nums[un_order] {
        i -= 1;
    }
    nums.swap(un_order, i);
    nums[un_order + 1..].reverse();
}

// 6. thinking
//
//  1) 必须要对算法清楚, 不能有一点马虎
