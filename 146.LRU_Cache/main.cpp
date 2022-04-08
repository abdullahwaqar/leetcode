/*
Design and implement a data structure for Least Recently Used (LRU) cache. It should support the following operations: get and put.

get(key) - Get the value (will always be positive) of the key if the key exists in the cache, otherwise return -1.
put(key, value) - Set or insert the value if the key is not already present. When the cache reached its capacity, it should invalidate the least recently used item before inserting a new item.

The cache is initialized with a positive capacity.

Follow up:
Could you do both operations in O(1) time complexity?

Example:

LRUCache cache = new LRUCache( 2

cache.put(1, 1);
cache.put(2, 2);
cache.get(1);       // returns 1
cache.put(3, 3);    // evicts key 2
cache.get(2);       // returns -1 (not found)
cache.put(4, 4);    // evicts key 1
cache.get(1);       // returns -1 (not found)
cache.get(3);       // returns 3
cache.get(4);       // returns 4

*/
#include <iostream>
#include <list>
#include <unordered_map>

struct CacheNode {
        CacheNode(const int k = 0, const int v = 0): key(k), value(v) {}
        int key;
        int value;
};

class LRUCache {
    private:
        std::list<CacheNode> cacheList;
        std::unordered_map<int, std::list<CacheNode>::iterator> cacheMap;
        int capacity;
    public:
        LRUCache(int capacity);
        int get(int key);
        void put(int key, int value);
};

LRUCache::LRUCache(int capacity): capacity(capacity) {}

int LRUCache::get(int key) {
    if (cacheMap.find(key) == cacheMap.end()) return -1; //* If the key does not return -1
    auto it = cacheMap[key]; //* Get the position of the key in the cache map
    cacheList.push_front(*it); //* Push the founded value to the front of the list -> recently used value
    cacheList.erase(it); //* Remove the old value -> duplicate value that exist in the original position

    cacheMap[key] = cacheList.begin(); //* Updating the cache map to new position: begin()
    return cacheMap[key]->value; //* Return the value
}

void LRUCache::put(int key, int value) {
    //* If there is no new key in cache then insert
    //* Else just update the value and update the cache key position to the front
    if (cacheMap.find(key) == cacheMap.end()) {
        //* Insert the new cache value in fron of the list
        cacheList.push_front(CacheNode(key, value));
        cacheMap[key]->value;

        //* If max capacity reached delete the last element
        if (cacheList.size() > capacity) {
            CacheNode lastNode = cacheList.back();
            cacheMap.erase(lastNode.key);
            cacheList.pop_back();
        }
    } else {
        //* Find the position and update the value to begin()
        auto it = cacheMap[key];
        it->value = value;
        cacheList.push_front(*it);
        cacheList.erase(it); //* Remove the old value -> duplicate value that exist in the original position
        cacheMap[key] = cacheList.begin(); //* Updating the cache map to new position: begin()
    }
}

int main(int argc, char const *argv[]) {
    LRUCache cache(2);
    cache.put(1, 1);
    cache.put(2, 2);
    std::cout << cache.get(1);       // returns 1
    cache.put(3, 3);    // evicts key 2
    std::cout <<  cache.get(2);       // returns -1 (not found)
    cache.put(4, 4);    // evicts key 1
    std::cout <<  cache.get(1);       // returns -1 (not found)
    std::cout <<  cache.get(3);       // returns 3
    std::cout << cache.get(4);       // returns 4
    return 0;
}

