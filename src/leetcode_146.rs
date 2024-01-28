use std::collections::HashMap;
use std::ptr;

struct LRUCache {
    container: HashMap<i32, *mut Node>,
    head: *mut Node,
    tail: *mut Node,
    len: usize,
    capacity: i32,
}

struct Node {
    key: i32,
    val: i32,
    pre: *mut Node,
    next: *mut Node,
}

impl Node {
    fn new(key: i32, val: i32) -> Self {
        Self {
            key,
            val,
            pre: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            len: 0,
            capacity,
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            container: HashMap::with_capacity(capacity as usize),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.container.remove(&key) {
            //move ptr to first
            Some(ptr) => {
                self.move_to_first(ptr);
                self.container.insert(key, ptr);
                unsafe { (*ptr).val }
            }
            None => -1,
        }
    }

    fn move_to_first(&mut self, ptr: *mut Node) {
        self.remove(ptr);
        self.add_first(ptr);
    }

    fn remove_last(&mut self) -> *mut Node {
        let last = self.tail;
        assert!(!last.is_null());
        unsafe {
            if (*last).pre.is_null() {
                self.head = ptr::null_mut();
                self.tail = ptr::null_mut();
            } else {
                self.tail = (*last).pre;
                (*self.tail).next = ptr::null_mut();
                (*last).pre = ptr::null_mut();
            }
        }
        last
    }

    fn remove_first(&mut self) {
        let first = self.head;
        assert!(!first.is_null());
        unsafe {
            if (*first).next.is_null() {
                self.head = ptr::null_mut();
                self.tail = ptr::null_mut();
            } else {
                self.head = (*first).next;
                (*self.head).pre = ptr::null_mut();
                (*first).next = ptr::null_mut();
            }
        }
    }

    fn add_first(&mut self, ptr: *mut Node) {
        let first = self.head;
        if first.is_null() {
            self.head = ptr;
            self.tail = ptr;
        } else {
            unsafe {
                (*first).pre = ptr;
                (*ptr).next = first;
                (*ptr).pre = ptr::null_mut();
            }
            self.head = ptr;
        }
    }

    fn remove(&mut self, ptr: *mut Node) {
        if ptr == self.head {
            self.remove_first();
        } else if ptr == self.tail {
            self.remove_last();
        } else {
            unsafe {
                (*(*ptr).pre).next = (*ptr).next;
                (*(*ptr).next).pre = (*ptr).pre;
                (*ptr).pre = ptr::null_mut();
                (*ptr).next = ptr::null_mut();
            }
        }
    }
    fn put(&mut self, key: i32, value: i32) {
        match self.len {
            0 => {
                let node = Node::new(key, value);
                let ptr = Box::into_raw(Box::new(node));
                self.head = ptr;
                self.tail = ptr;
                self.container.insert(key, ptr);
                self.len += 1;
            }
            len => {
                match self.container.remove(&key) {
                    //move ptr to first if exist
                    Some(ptr) => {
                        unsafe { (*ptr).val = value };
                        self.move_to_first(ptr);
                        self.container.insert(key, ptr);
                    }
                    None => {
                        //add first
                        let ptr = Box::into_raw(Box::new(Node::new(key, value)));
                        self.add_first(ptr);
                        self.container.insert(key, ptr);
                        //remove last if full
                        if len == self.capacity as usize {
                            let last = self.remove_last();
                            self.container.remove(unsafe { &(*last).key });
                        } else {
                            self.len += 1;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode_146::LRUCache;

    #[test]
    fn test() {
        let mut lRUCache = LRUCache::new(3);
        lRUCache.put(1, 1); // 缓存是 {1=1}
        lRUCache.put(2, 2); // 缓存是 {1=1}
        lRUCache.put(3, 3); // 缓存是 {1=1}
        lRUCache.put(4, 4); // 缓存是 {1=1}

        println!("{}", lRUCache.get(4)); // 返回 1
        println!("{}", lRUCache.get(3)); // 返回 1
        println!("{}", lRUCache.get(2)); // 返回 1
        println!("{}", lRUCache.get(1)); // 返回 1
        lRUCache.put(5, 5); // 缓存是 {1=1}

        println!("{}", lRUCache.get(1)); // 返回 1
        println!("{}", lRUCache.get(2)); // 返回 1
        println!("{}", lRUCache.get(3)); // 返回 1
        println!("{}", lRUCache.get(4));
        println!("{}", lRUCache.get(5));
    }
}
