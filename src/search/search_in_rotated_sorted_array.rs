// question 33

// 1. question
//
// Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.

// (i.e., [0,1,2,4,5,6,7] might become [4,5,6,7,0,1,2]).

// You are given a target value to search. If found in the array return its index, otherwise return -1.

// You may assume no duplicate exists in the array.

// Your algorithm's runtime complexity must be in the order of O(log n).

// Example 1:

// Input: nums = [4,5,6,7,0,1,2], target = 0
// Output: 4

// Example 2:

// Input: nums = [4,5,6,7,0,1,2], target = 3
// Output: -1

// 2. my solution
//
// pub fn search(nums: Vec<i32>, target: i32) -> i32 {
//     let pivot = find_pivot(&nums).unwrap_or(nums.len());

//     if let Ok(pos) = nums[0..pivot].binary_search(&target) {
//         return pos as i32;
//     }
//     if let Ok(pos) = nums[pivot..nums.len()].binary_search(&target) {
//         return (pivot + pos) as i32;
//     }
//     -1
// }

// fn find_pivot(nums: &[i32]) -> Option<usize> {
//     if nums.len() <= 1 || nums[0] < nums[nums.len() - 1] {
//         return None;
//     }

//     let mut left = 0;
//     let mut right = nums.len() - 1;

//     while left < right {
//         let mid = left + (right - left) / 2;

//         if left > 0 && nums[left] < nums[left - 1] {
//             return Some(left);
//         } else if nums[right] < nums[right - 1] {
//             return Some(right);
//         } else if nums[mid] < nums[mid - 1] {
//             return Some(mid);
//         } else if nums[mid] > nums[right] {
//             left = mid + 1;
//         } else if nums[mid] < nums[left] {
//             right = mid - 1;
//         } else {
//             unreachable!("");
//         }
//     }
//     None
// }

// 3. others solution
//
// class Solution {
// public:
//     int search(int A[], int n, int target) {
//         int lo=0,hi=n-1;
//         // find the index of the smallest value using binary search.
//         // Loop will terminate since mid < hi, and lo or hi will shrink by at least 1.
//         // Proof by contradiction that mid < hi: if mid==hi, then lo==hi and loop would have been terminated.
//         while(lo<hi){
//             int mid=(lo+hi)/2;
//             if(A[mid]>A[hi]) lo=mid+1;
//             else hi=mid;
//         }
//         // lo==hi is the index of the smallest value and also the number of places rotated.
//         int rot=lo;
//         lo=0;hi=n-1;
//         // The usual binary search and accounting for rotation.
//         while(lo<=hi){
//             int mid=(lo+hi)/2;
//             int realmid=(mid+rot)%n;
//             if(A[realmid]==target)return realmid;
//             if(A[realmid]<target)lo=mid+1;
//             else hi=mid-1;
//         }
//         return -1;
//     }
// };

// public int search(int[] nums, int target) {
//    if (nums == null || nums.length == 0) {
//        return -1;
//    }

//    /*.*/
//    int left = 0, right = nums.length - 1;
//    //when we use the condition "left <= right", we do not need to determine if nums[left] == target
//    //in outside of loop, because the jumping condition is left > right, we will have the determination
//    //condition if(target == nums[mid]) inside of loop
//    while (left <= right) {
//        //left bias
//        int mid = left + (right - left) / 2;
//        if (target == nums[mid]) {
//            return mid;
//        }
//        //if left part is monotonically increasing, or the pivot point is on the right part
//        if (nums[left] <= nums[mid]) {
//            //must use "<=" at here since we need to make sure target is in the left part,
//            //then safely drop the right part
//            if (nums[left] <= target && target < nums[mid]) {
//                right = mid - 1;
//            }
//            else {
//                //right bias
//                left = mid + 1;
//            }
//        }

//        //if right part is monotonically increasing, or the pivot point is on the left part
//        else {
//            //must use "<=" at here since we need to make sure target is in the right part,
//            //then safely drop the left part
//            if (nums[mid] < target && target <= nums[right]) {
//                left = mid + 1;
//            }
//            else {
//                right = mid - 1;
//            }
//        }
//    }
//    return -1;
//}
//
//
// int search(vector<int>& nums, int target) {
// int lo = 0, hi = nums.size();
// while (lo < hi) {
//     int mid = (lo + hi) / 2;

//     double num = (nums[mid] < nums[0]) == (target < nums[0])
//                ? nums[mid]
//                : target < nums[0] ? -INFINITY : INFINITY;

//     if (num < target)
//         lo = mid + 1;
//     else if (num > target)
//         hi = mid;
//     else
//         return mid;
// }
// return -1;
// }
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() {
        return -1;
    }
    let pivot = find_min(&nums);

    let rot = pivot;

    let mut lo = 0;
    let mut hi = nums.len();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let real_mid = (mid + rot) % nums.len();

        if nums[real_mid] == target {
            return real_mid as i32;
        } else if nums[real_mid] < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    return -1;
}

fn find_min(nums: &[i32]) -> usize {
    assert!(!nums.is_empty());
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] > nums[right] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    return left;
}
// 4. what can be improved
//
