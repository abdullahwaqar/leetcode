/**
Given an array of distinct integers candidates and a target integer target,
return a list of all unique combinations of candidates where the chosen numbers
sum to target. You may return the combinations in any order.

The same number may be chosen from candidates an unlimited number of times. Two
combinations are unique if the frequency of at least one of the chosen numbers
is different.

Constraints:

    1 <= candidates.length <= 30
    1 <= candidates[i] <= 200
    All elements of candidates are distinct.
    1 <= target <= 500

*/

#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
  public:
    vector<vector<int>> combinationSum(vector<int> &candidates, int target) {
        vector<vector<int>> res;
        vector<int> path;
        sort(candidates.begin(), candidates.end());
        dfs(candidates, target, 0, res, path);
        return res;
    }

  private:
    void dfs(const vector<int> &candidates, int target, int idx,
             vector<vector<int>> &res, vector<int> &path) {
        if (target == 0) {
            res.push_back(path);
            return;
        }

        for (int i = idx; i < candidates.size(); ++i) {
            if (candidates[i] > target)
                return;

            path.push_back(candidates[i]);
            dfs(candidates, target - candidates[i], i, res, path);
            path.pop_back();
        }
    }
};