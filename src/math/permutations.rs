// question 46

// 1. question
// Given a collection of distinct integers, return all possible permutations.

// Example:

// Input: [1,2,3]
// Output:
// [
//  [1,2,3],
//  [1,3,2],
//  [2,1,3],
//  [2,3,1],
//  [3,1,2],
//  [3,2,1]
//]

// 2. my solution
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let fac = (1..=nums.len()).fold(1, |acc, x| acc * x);

    (1..=fac)
        .map(|_| {
            let permutation = nums.clone();
            next_permutation(&mut nums);
            permutation
        })
        .collect()
}

pub fn next_permutation(nums: &mut Vec<i32>) {
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
    while nums[un_order] >= nums[i] {
        i -= 1;
    }

    nums.swap(un_order, i);
    nums[un_order + 1..].reverse();
}

// 3. other's solution

// backtracking
//
// public List<List<Integer>> permute(int[] nums) {
//    List<List<Integer>> list = new ArrayList<>();
//    // Arrays.sort(nums); // not necessary
//    backtrack(list, new ArrayList<>(), nums);
//    return list;
// }

// private void backtrack(List<List<Integer>> list, List<Integer> tempList, int [] nums){
//    if(tempList.size() == nums.length){
//       list.add(new ArrayList<>(tempList));
//    } else{
//       for(int i = 0; i < nums.length; i++){
//          if(tempList.contains(nums[i])) continue; // element already exists, skip
//          tempList.add(nums[i]);
//          backtrack(list, tempList, nums);
//          tempList.remove(tempList.size() - 1);
//       }
//    }
// }

// DFS
//
// def permute(self, nums):
//     res = []
//     self.dfs(nums, [], res)
//     return res
//
// def dfs(self, nums, path, res):
//     if not nums:
//         res.append(path)
//         # return # backtracking
//     for i in xrange(len(nums)):
//         self.dfs(nums[:i]+nums[i+1:], path+[nums[i]], res)

// dfs(nums = [1, 2, 3] , path = [] , result = [] )
// |____ dfs(nums = [2, 3] , path = [1] , result = [] )
// |      |___dfs(nums = [3] , path = [1, 2] , result = [] )
// |      |    |___dfs(nums = [] , path = [1, 2, 3] , result = [[1, 2, 3]] ) # added a new permutation to the result
// |      |___dfs(nums = [2] , path = [1, 3] , result = [[1, 2, 3]] )
// |           |___dfs(nums = [] , path = [1, 3, 2] , result = [[1, 2, 3], [1, 3, 2]] ) # added a new permutation to the result
// |____ dfs(nums = [1, 3] , path = [2] , result = [[1, 2, 3], [1, 3, 2]] )
// |      |___dfs(nums = [3] , path = [2, 1] , result = [[1, 2, 3], [1, 3, 2]] )
// |      |    |___dfs(nums = [] , path = [2, 1, 3] , result = [[1, 2, 3], [1, 3, 2], [2, 1, 3]] ) # added a new permutation to the result
// |      |___dfs(nums = [1] , path = [2, 3] , result = [[1, 2, 3], [1, 3, 2], [2, 1, 3]] )
// |           |___dfs(nums = [] , path = [2, 3, 1] , result = [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1]] ) # added a new permutation to the result
// |____ dfs(nums = [1, 2] , path = [3] , result = [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1]] )
//        |___dfs(nums = [2] , path = [3, 1] , result = [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1]] )
//        |    |___dfs(nums = [] , path = [3, 1, 2] , result = [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2]] ) # added a new permutation to the result
//        |___dfs(nums = [1] , path = [3, 2] , result = [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2]] )
//             |___dfs(nums = [] , path = [3, 2, 1] , result = [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]] ) # added a new permutation to the result

// void dfs(int pos, vector<int> &num, vector<vector<int>> &result){
//     if(pos == num.size()){
//         result.push_back(num);
//     }
//     else{
//         for(int i=pos; i<num.size(); i++){
//             swap(num[i], num[pos]);
//             dfs(pos+1, num, result);
//             swap(num[i], num[pos]);
//         }
//     }
// }

// 4. what can be improved
//   1) 使用不同的方法(回溯, dfs 等)

// 5. improved again
//
// 注意这种交换 DFS 对有序 nums
// 可以保证集合正确, 不能保证顺序正确(对大于 num.len() / 2 的部分)
//
// pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
//     let mut nums = nums;
//     let mut ans: Vec<Vec<i32>> = Vec::new();

//     if nums.len() == 0 {
//         return ans;
//     }

//     dfs(&mut nums, &mut ans, 0);
//     ans
// }

// fn dfs(nums: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, current: usize) {
//     assert!(nums.len() > 0);
//     if current == nums.len() {
//         result.push(nums.clone());
//     }

//     for i in current..nums.len() {
//         nums.swap(i, current);
//         dfs(nums, result, current + 1);
//         nums.swap(i, current);
//     }
// }

// 6. thinking
//   1) 问自己, 本质上要的是什么, 一般来说都有常见模型
//   2) 掌握常见模型
