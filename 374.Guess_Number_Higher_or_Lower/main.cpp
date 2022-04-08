/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is lower than the guess number
 *			      1 if num is higher than the guess number
 *               otherwise return 0
 * int guess(int num);
 */

class Solution {
  public:
    int guessNumber(int n) {
        long long int left = 0;
        long long int right = n;
        long long int ans = 0;
        while (left <= right) {
            long long int middle = (left + right) / 2;

            if (guess(middle) == -1) {
                right = middle - 1;
            } else if (guess(middle) == 0) {
                return middle;
            } else {
                left = middle + 1;
            }
        }
        return ans;
    }
};