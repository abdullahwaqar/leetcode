/*
You have a RecentCounter class which counts the number of recent requests within
a certain time frame.

Implement the RecentCounter class:

    RecentCounter() Initializes the counter with zero recent requests.
    int ping(int t) Adds a new request at time t, where t represents some time
in milliseconds, and returns the number of requests that has happened in the
past 3000 milliseconds (including the new request). Specifically, return the
number of requests that have happened in the inclusive range [t - 3000, t].

It is guaranteed that every call to ping uses a strictly larger value of t than
the previous call.



Constraints:

    1 <= t <= 104
    Each test case will call ping with strictly increasing values of t.
    At most 104 calls will be made to ping.

*/

#include <iostream>
#include <vector>

class RecentCounter {
  public:
    std::vector<long> queue;
    RecentCounter() {}

    /**
     * We need to store only the t, that occurs 3000 milliseconds ago until
     * now, which means the previous stored t should be greater or equal to
     * (current t-3000). Since each test case will call ping with strictly
     * increasing values of t, the stored elements in the array is in
     * ascending order. We add the new t to the end and maintain the front
     * of the array within the time frame of t-3000, when it’s not
     * satisfied, we remove the front element.
     */
    int ping(int t) {
        queue.push_back(t);
        while (queue[0] < t - 3000) {
            queue.erase(queue.begin());
        }
        return queue.size();
    }
};

/**
 * Your RecentCounter object will be instantiated and called as such:
 * RecentCounter* obj = new RecentCounter();
 * int param_1 = obj->ping(t);
 */