class Solution {
public:
    bool isPowerOfTwo(int n) {
          if (n<=0) return false;
        
        //keep on looping while bit shifting n to the right, if n is 1 then it is a power of 2
        //if n is odd at any point in this loop then n is not a power of 2 since a power of 2
        //represented in binary should have only 1 "1" digit in the entire number.
        while (n!=1) {
            if (n%2==1) return false;
            n>>=1;
        }
        return true;
    }
};