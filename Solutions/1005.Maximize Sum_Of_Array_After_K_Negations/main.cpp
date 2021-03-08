#include <algorithm>
#include <iostream>
#include <numeric>
#include <vector>

class Solution {
  public:
    int largestSumAfterKNegations(std::vector<int> &A, int K) {
        std::sort(A.begin(), A.end());
        int left = 0;
        int size = A.size();

        while (K--) {
            if (A[left] < 0) {
                A[left] = -A[left];
                if (left < size && A[left] > A[left + 1]) {
                    left += 1;
                }
            } else {
                A[left] = -A[left];
            }
        }

        int sum = 0;
        sum = std::accumulate(A.begin(), A.end(), sum);
        return sum;
    }
};

int main(int argc, char const *argv[]) {
    std::vector<int> arr{2, -3, -1, 5, -4};
    auto sol = Solution();
    auto r = sol.largestSumAfterKNegations(arr, 2);
    std::cout << r;
    return 0;
}
