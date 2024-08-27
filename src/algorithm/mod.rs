pub mod sort;
pub mod string;
pub mod bit;


#[cfg(test)]
mod test_sort {
    use rand::{Rng, thread_rng};
    use crate::algorithm::sort::*;
    fn gen_ranom_vector() -> (Vec<isize>, Vec<isize>) {
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        for _ in 0..10000 {
            let n = thread_rng().gen();
            v1.push(n);
            v2.push(n);
        }
        (v1, v2)
    }
    #[test]
    fn test_bubble_sort() {
        let (mut test, mut standard) = gen_ranom_vector();
        standard.sort();
        bubble_sort(&mut test);
        assert_eq!(test, standard);
    }
    #[test]
    fn test_select_sort() {
        let (mut test, mut standard) = gen_ranom_vector();
        standard.sort();
        select_sort(&mut test);
        assert_eq!(test, standard);
    }
    #[test]
    fn test_insert_sort() {
        let (mut test, mut standard) = gen_ranom_vector();
        standard.sort();
        insert_sort(&mut test);
        assert_eq!(test, standard);
    }
    #[test]
    fn test_heap_sort() {
        let (mut test, mut standard) = gen_ranom_vector();
        standard.sort();
        heap_sort(&mut test);
        assert_eq!(test, standard);
    }
    #[test]
    fn test_bucket_sort() {
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        let max = 10000;
        for _ in 0..max {
            let n: usize = thread_rng().gen();
            let n = n % (usize::max_value() / max);
            v1.push(n);
            v2.push(n);
        }
        bucket_sort(&mut v1);
        v2.sort();
        assert_eq!(v1, v2);
    }
    #[test]
    fn test_radix_sort() {
        let (test, standard) = gen_ranom_vector();
        let mut test: Vec<usize> = test.into_iter().map(|x| (x as usize) / 2).collect();
        let mut standard: Vec<usize> = standard.into_iter().map(|x| (x as usize) / 2).collect();
        standard.sort();
        radix_sort(&mut test);
        assert_eq!(test, standard);
    }
    #[test]
    fn test_count_sort() {
        let (test, standard) = gen_ranom_vector();
        let mut test: Vec<usize> = test.into_iter().map(|x| (x as usize) % 10000).collect();
        let mut standard: Vec<usize> = standard.into_iter().map(|x| (x as usize) % 10000).collect();
        standard.sort();
        radix_sort(&mut test);
        assert_eq!(test, standard);
    }
    #[test] 
    fn test_shell_sort() {
        let (mut test, mut standard) = gen_ranom_vector();
        standard.sort();
        shell_sort(&mut test);
        assert_eq!(test, standard);
    }
    #[test]
    fn test_merge_sort() {
        let (mut test, mut standard) = gen_ranom_vector();
        standard.sort();
        merge_sort(&mut test);
        assert_eq!(test, standard);
    }
    #[test]
    fn test_quick_sort() {
        let (mut test, mut standard) = gen_ranom_vector();
        standard.sort();
        quick_sort(&mut test);
        assert_eq!(test, standard);
    }
}

#[cfg(test)]
mod test_bit{
    use super::bit::*;
    use rand::{Rng, thread_rng};
    #[test]
    fn test_swap() {
        let mut a = 10;
        let mut b = 5;
        swap(&mut a, &mut b);
        assert_eq!(a, 5);
        assert_eq!(b, 10);
    }
    #[test]
    fn test_max() {
        let a = 50;
        let b = 25;
        assert_eq!(max(a, b), 50);
    }
    #[test]
    fn test_add() {
        for _ in 0..100 {
            let a: i64 = thread_rng().gen::<i64>() / 2;
            let b: i64 = thread_rng().gen::<i64>() / 2;
            let sum = a + b; 
            let test_sum = add(a, b);
            assert_eq!(sum, test_sum);
        }
    }
    #[test]
    fn test_sub() {
        for _ in 0..100 {
            let a: i64 = thread_rng().gen::<i64>() / 2;
            let b: i64 = thread_rng().gen::<i64>() / 2;
            let difference = a - b; 
            let test_difference = sub(a, b);
            assert_eq!(difference, test_difference);
        }
    }
    #[test]
    fn test_mul() {
        for _ in 0..100 {
            let a: i64 = thread_rng().gen::<i32>() as i64;
            let b: i64 = thread_rng().gen::<i32>() as i64;
            let product = a * b; 
            let test_product = mul(a, b);
            assert_eq!(product, test_product);
        }
    }
}
