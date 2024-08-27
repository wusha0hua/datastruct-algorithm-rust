pub mod heap;

#[cfg(test)]
mod test_heap {
    use crate::datastruct::heap::heap::Heap;
    use rand::{Rng, thread_rng};
    #[test]
    fn test_min_heap() {
        let mut heap = Heap::<isize>::new(); 
        let mut test = Vec::new();
        for _ in 0..10000 {
            let n = thread_rng().gen();
            test.push(n);
            heap.push(n);
        }
        let mut heap_vec = Vec::new();
        while let Some(n) = heap.pop() {
            heap_vec.push(n);
        }
        test.sort();
        assert_eq!(heap_vec, test);
    }
    #[test]
    fn test_max_heap() {
        let mut heap = Heap::<isize>::new(); 
        heap.set_heap_order_max();
        let mut test = Vec::new();
        for _ in 0..10000 {
            let n = thread_rng().gen();
            test.push(n);
            heap.push(n);
        }
        let mut heap_vec = Vec::new();
        while let Some(n) = heap.pop() {
            heap_vec.push(n);
        }
        test.sort();
        test.reverse();
        assert_eq!(heap_vec, test);
    }

}
