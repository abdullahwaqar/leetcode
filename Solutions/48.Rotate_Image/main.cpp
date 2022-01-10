#include <iostream>
#include <vector>

std::ostream &operator<<(std::ostream &os,
                         const std::vector<std::vector<int>> &matrix) {
    for (int i = 0; i < matrix.size(); i++) {
        for (int j = 0; j < matrix.at(i).size(); j++) {
            os << matrix[i][j] << ", ";
        }
        os << std::endl;
    }
    os << std::endl;
    return os;
}

class Solution {
  public:
    void rotate(std::vector<std::vector<int>> &matrix) {
        // * nxn always true
        // std::cout << matrix;

        // * Transpose
        for (int row_idx = 0; row_idx < matrix.size(); row_idx++) {
            for (int col_idx = row_idx; col_idx < matrix.size(); col_idx++) {
                auto temporal_elem = matrix[row_idx][col_idx];
                matrix[row_idx][col_idx] = matrix[col_idx][row_idx];
                matrix[col_idx][row_idx] = temporal_elem;
            }
        }

        // * Flip horizontally
        for (int row_idx = 0; row_idx < matrix.size(); row_idx++) {
            for (int col_idx = 0; col_idx < (matrix.size() / 2); col_idx++) {
                auto temporal_elem = matrix[row_idx][col_idx];
                matrix[row_idx][col_idx] =
                    matrix[row_idx][matrix.size() - col_idx - 1];
                matrix[row_idx][matrix.size() - col_idx - 1] = temporal_elem;
            }
        }

        // std::cout << matrix;
    }
};

int main(int argc, char const *argv[]) {
    std::vector<std::vector<int>> matrix = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};

    auto solution = Solution{};

    solution.rotate(matrix);
    return 0;
}
