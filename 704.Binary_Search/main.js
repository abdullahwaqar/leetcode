/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var search = function(nums, target) {
    let low = 0;
    let high = nums.length;
    while (low < high) {
        const mid = parseInt((low + high) / 2);
        if (nums[mid] === target) return mid; //* if target is at mid (parent node) return
        if (nums[mid] < target) {
            low = mid + 1; //* go to the left side
        } else {
            high = mid - 1; //* go to the high side
        }
    }
    if (nums[high] === target) return high;
    return -1;

};

console.log(search([-1,0,3,5,9,12 ], 9));
