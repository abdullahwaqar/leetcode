#include <math.h>
#include <stdio.h>
#include <vector>

class Solution {
  public:
    int minimumTotal(std::vector<std::vector<int>> &triangle) {
        for (int i = triangle.size() - 2; ~i; --i) {
            printf("\n");
            for (int j = triangle[i].size() - 1; ~j; --j) {
                triangle[i][j] +=
                    std::min(triangle[i + 1][j], triangle[i + 1][j + 1]);
                printf("Min: %i\n",
                       std::min(triangle[i + 1][j], triangle[i + 1][j + 1]));
            }
        }
        return triangle[0][0];
    }
};

int main(int argc, char const *argv[]) {
    std::vector<std::vector<int>> t = {{2}, {3, 4}, {6, 5, 7}, {4, 1, 8, 3}};
    Solution().minimumTotal(t);
    return 0;
}
