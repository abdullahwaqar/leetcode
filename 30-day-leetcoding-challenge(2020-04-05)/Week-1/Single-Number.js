/* *
Given a non-empty array of integers, every element appears twice except for one. Find that single one.

Note:

Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?

Example 1:

Input: [2,2,1]
Output: 1

Example 2:

Input: [4,1,2,1,2]
Output: 4
*/

/**
 * @param {number[]} nums
 * @return {number}
 */
var singleNumber = function(nums) {
    var seen = {};
    nums.forEach(n => {
        if (seen[n]) {
            seen[n] += 1; 
        } else {
            seen[n] = 1;
        }
    });
    for (var key in seen) {
        if (seen[key] === 1) return key
    }
};

console.log(singleNumber([4,1,2,1,2]));