/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.count += 1;
        self.items.push(value); // 元素加入堆底

        let mut idx = self.count; // 从底部开始
        while self.parent_idx(idx) > 0 { // 若存在父节点（不为0）
            let p_idx = self.parent_idx(idx);
            // 则向上堆处理（根据处理<或>区分大根或小根堆）
            if (self.comparator)(&self.items[idx], &self.items[p_idx]) {
                self.items.swap(idx, p_idx);
            }
            idx = p_idx; // 往上一层
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        if self.right_child_idx(idx) > self.count { // 右孩子不存在
            self.left_child_idx(idx) // 左孩子直接返回
        } else { // 否则比较左右孩子选值最小的一个
            let l_idx = self.left_child_idx(idx);
            let r_idx = self.right_child_idx(idx);
            if (self.comparator)(&self.items[l_idx], &self.items[r_idx]) {
                l_idx
            } else {
                r_idx
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count == 0 { // 堆为空
            return None;
        }
        // 否则弹出堆顶元素（swap_remove用最后一个填充）
        let next = Some(self.items.swap_remove(1));
        self.count -= 1;
        // 向下堆处理
        if self.count > 0 {
            let mut idx = 1;
            while self.children_present(idx) {
                let c_idx = self.smallest_child_idx(idx);
                if !(self.comparator)(&self.items[idx], &self.items[c_idx]) {
                    self.items.swap(idx, c_idx);
                }
                idx = c_idx;
            }
        }
        next
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}