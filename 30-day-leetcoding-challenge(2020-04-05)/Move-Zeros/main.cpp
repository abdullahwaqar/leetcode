class Solution {
public:
    void moveZeroes(vector<int>& nums) {
        int n = nums.size();
        int next = 0;
        for (int x: nums) {
            if (x != 0) {
                nums[next] = x;
                next++; 
            }
        }
        for (int i = 0; i < n; i++) {
            nums[i] = 0;
        }
    }
};