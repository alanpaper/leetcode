use std::{collections::{HashMap, VecDeque}};


#[derive(Debug)]
struct LFUCache {
    size: i32,
    queue: VecDeque<i32>,
    cache: HashMap<i32, i32>,
    cache_key: HashMap<i32, usize>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {

    fn new(capacity: i32) -> Self {
        LFUCache { size: capacity, queue: VecDeque::new(), cache: HashMap::new(), cache_key: HashMap::new() }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        println!("{}, self.queue = {:#?}",key, self);
        if let Some(&index) = self.cache_key.get(&key) {
            if let Some(&x) = self.queue.get(index as usize) {
                if x == key {
                    self.queue.remove(index as usize);
                }
            }
            self.queue.push_back(key);
            for i in 0..self.queue.len() {
                self.cache_key.insert(self.queue[i], i);
            }
        }
        println!("{:?} = {:?}", key, self.queue);
        if let Some(&val) = self.cache.get(&key) {
            return val
        }
        return -1
    }
    
    fn put(&mut self, key: i32, value: i32) {
        
        if let Some(&index) = self.cache_key.get(&key) {
            if let Some(&x) = self.queue.get(index as usize) {
                if x == key {
                    self.queue.remove(index as usize);
                }
            }
            self.queue.push_back(key);
            for i in 0..self.queue.len() {
                self.cache_key.insert(self.queue[i], i);
            }
        } else {
            if self.size as usize == self.queue.len() {
                println!("{}={}, self.queue = {:?}",key, value, self.queue);
                if let Some(first) = self.queue.pop_front() {
                    self.cache.remove(&first);
                    self.queue.push_back(key);
                    for i in 0..self.queue.len() {
                        self.cache_key.insert(self.queue[i], i);
                    }
                }
                println!("{}={}, self.queue = {:?}",key, value, self.queue);
            } else {
                self.queue.push_back(key);
                for i in 0..self.queue.len() {
                    self.cache_key.insert(self.queue[i], i);
                }
            }
        }
        self.cache.insert(key, value);
    }
}



#[test]
fn test() {
    let mut lfu = LFUCache::new(2);
    lfu.put(1, 1);
    lfu.put(2, 2);
    lfu.get(1);
    lfu.put(3,3);
    lfu.get(2);
    lfu.get(3);
    lfu.put(4,4);
    lfu.get(1);
    lfu.get(3);
    lfu.get(4);
    assert_eq!(1,2);
}
