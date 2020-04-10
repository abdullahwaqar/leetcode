class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        int answer = INT_MIN;
        int a = 0;
        for (int n : nums) {
            a += n;
            answer = max(answer, a);
            a = max(a, 0);
        }
        return answer;
    }
};