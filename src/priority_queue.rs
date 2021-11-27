use std::collections::HashMap;

// minheap
#[derive(Debug)]
pub struct PriorityQueue<P, V> {
    elems: Vec<(P,V)>,
    value_map: HashMap<V, usize>,
}

impl<P: PartialOrd + Copy + std::fmt::Debug, V: Copy + Eq + std::hash::Hash+ std::fmt::Debug> PriorityQueue<P, V> {
    pub fn new() -> PriorityQueue<P, V> {
        PriorityQueue {
            elems: Vec::new(),
            value_map: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.elems.len()
    }

    fn swap(&mut self, i: usize, j: usize) {
        let tmp = self.elems[i];
        self.elems[i] = self.elems[j];
        self.elems[j] = tmp;
        
        self.value_map.insert(self.elems[j].1, j);
        self.value_map.insert(self.elems[i].1, i);
    }

    fn upheap(&mut self, mut i: usize) {
        while i > 0 {
            let parent = self.elems[i/2];
            if parent.0 > self.elems[i].0 {
                self.swap(i, i/2);
                i /= 2;
            } else {
                return
            }
        }
    }

    fn downheap(&mut self, mut i: usize) {
        loop {
            // No children
            if i*2 + 1 >= self.elems.len() {
                return;
            }
            // no right child - just try left child
            if i*2 + 2 >= self.elems.len() {
                if self.elems[i].0 > self.elems[i*2+1].0 {
                    self.swap(i, i*2+1);
                    i = i * 2 + 1;
                    continue;
                } else {
                    return;
                }
            }

            // both children
            let left_greatest = self.elems[i*2+1].0 >= self.elems[i*2+2].0;
            let greater_than_left = self.elems[i].0 > self.elems[i*2+1].0;
            let greater_than_right = self.elems[i].0 > self.elems[i*2+2].0;

            let swap_left = (greater_than_left && greater_than_right && !left_greatest) || (greater_than_left && !greater_than_right);
            
            if swap_left {
                self.swap(i, i*2+1);
                i = i * 2 + 1;
                continue;
            }

            let swap_right = (greater_than_left && greater_than_right && left_greatest) || (greater_than_right && !greater_than_left);

            if swap_right {
                self.swap(i, i*2+2);
                i = i * 2 + 2;
                continue;
            }

            return;
        }
    }

    fn update_priority(&mut self, elem_idx: usize, new_K: P) {
        let old_K = self.elems[elem_idx].0;
        self.elems[elem_idx].0 = new_K;
        if new_K > old_K {
            self.downheap(elem_idx);
        } else {
            self.upheap(elem_idx);
        }
    }

    fn insert(&mut self, priority: P, value: V) {
        self.elems.push((priority, value));
        self.value_map.insert(value, self.elems.len()-1);
        self.upheap(self.elems.len()-1);
    }

    // either inserts or update priority
    // ah how to update priority. look up, if decreasing, upheap, if increasing, downheap? Seems ok
    pub fn set(&mut self, priority: P, value: V) {
        if let Some(idx) = self.value_map.get(&value) {
            self.update_priority(*idx, priority);
        } else {
            self.insert(priority, value);
        }
    }

    pub fn remove_min(&mut self) -> Option<V> {
        if self.elems.len() == 0 {
            return None;
        }

        let min = self.elems[0];
        self.swap(0, self.elems.len()-1);
        self.elems.truncate(self.elems.len()-1);
        self.downheap(0);

        self.value_map.remove(&min.1);
        return Some(min.1);
    }

    pub fn remove(&mut self, value: V) {
        if let Some(idx) = self.value_map.get(&value).map(|u| *u) {
            self.value_map.remove(&value);
            self.swap(idx, self.len());
            self.elems.truncate(self.elems.len()-1);
            self.downheap(idx);
        };
    }
}

// can have a hashmap to index P indexes for O(1) decrease prioritys as well
#[test]
fn pq_test_ooo() {
    let mut pq = PriorityQueue::new();
    pq.set(1, "aaa");
    pq.set(2, "bbb");
    pq.set(3, "ccc");
    pq.set(4, "ddd");
    pq.set(5, "eee");
    pq.set(6, "fff");
    println!("{:?}", pq);
    assert_eq!(pq.remove_min().unwrap(), "aaa");
    assert_eq!(pq.remove_min().unwrap(), "bbb");
    assert_eq!(pq.remove_min().unwrap(), "ccc");
    assert_eq!(pq.remove_min().unwrap(), "ddd");
    assert_eq!(pq.remove_min().unwrap(), "eee");
    assert_eq!(pq.remove_min().unwrap(), "fff");
    assert_eq!(pq.remove_min(), None);



}

#[test]
fn pq_test_increase_key() {
    let mut pq = PriorityQueue::new();
    pq.set(0, "fff");
    pq.set(1, "aaa");
    pq.set(2, "bbb");
    pq.set(3, "ccc");
    pq.set(4, "ddd");
    pq.set(5, "eee");
    pq.set(6, "fff");
    assert_eq!(pq.remove_min().unwrap(), "aaa");
    assert_eq!(pq.remove_min().unwrap(), "bbb");
    assert_eq!(pq.remove_min().unwrap(), "ccc");
    assert_eq!(pq.remove_min().unwrap(), "ddd");
    assert_eq!(pq.remove_min().unwrap(), "eee");
    assert_eq!(pq.remove_min().unwrap(), "fff");
    assert_eq!(pq.remove_min(), None);
}

#[test]
fn pq_test_decrease_key() {
    let mut pq = PriorityQueue::new();
    pq.set(10, "fff");
    pq.set(1, "aaa");
    pq.set(2, "bbb");
    pq.set(3, "ccc");
    pq.set(4, "ddd");
    pq.set(5, "eee");
    pq.set(6, "fff");
    assert_eq!(pq.remove_min().unwrap(), "aaa");
    assert_eq!(pq.remove_min().unwrap(), "bbb");
    assert_eq!(pq.remove_min().unwrap(), "ccc");
    assert_eq!(pq.remove_min().unwrap(), "ddd");
    assert_eq!(pq.remove_min().unwrap(), "eee");
    assert_eq!(pq.remove_min().unwrap(), "fff");
    assert_eq!(pq.remove_min(), None);
}
#[test]
fn pq_test() {
    let mut pq = PriorityQueue::new();
    pq.set(10, "hello");
    pq.set(9, "asdasd");
    pq.set(15, "asdasd");
    pq.set(5, "spaget");
    pq.set(99, "hhh");
    pq.set(11, "aaa");
    //pq.set(2, "hhh");
    //pq.set(6, "hhh");


    assert_eq!(pq.remove_min().unwrap(), "spaget");
    assert_eq!(pq.remove_min().unwrap(), "hello");
    assert_eq!(pq.remove_min().unwrap(), "aaa");
    assert_eq!(pq.remove_min().unwrap(), "asdasd");
    assert_eq!(pq.remove_min().unwrap(), "hhh");
    assert_eq!(pq.remove_min(), None);
}