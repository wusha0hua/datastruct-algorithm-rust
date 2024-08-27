pub fn bubble_sort<T: PartialOrd>(data: &mut Vec<T>) {
    for i in 0..data.len() - 1 {
        for j in i + 1..data.len() {
            if data[i] > data[j] {
                data.swap(i, j);
            }
        }
    }
}

pub fn select_sort<T: PartialOrd>(data: &mut Vec<T>) {
    for i in 0..data.len() - 1 {
        let mut min_index = i;
        for j in i + 1..data.len() {
            if data[j] < data[min_index] {
                min_index = j;
            }
        }
        data.swap(i, min_index);
    }
}

pub fn insert_sort<T: PartialOrd>(data: &mut Vec<T>) {
    for i in 1..data.len() {
        for j in (1..=i).rev() {
            if data[j] < data[j - 1] {
                data.swap(j, j - 1);
            } 
        }
    }
}

pub fn heap_sort<T: PartialOrd>(data: &mut Vec<T>) {
    use crate::datastruct::heap::heap::Heap;
    let mut heap = Heap::new();
    while let Some(n) = data.pop() {
        heap.push(n);
    } 
    while let Some(n) = heap.pop() {
        data.push(n);
    }
}

pub fn bucket_sort(data: &mut Vec<usize>) {
    let len = data.len();
    let max = match data.iter().max() {
        Some(n) => *n,
        None => return,
    };
    let mut buckets = vec![vec![]; len + 1];
    while let Some(n) = data.pop() {
        let index = n * len / max;
        buckets[index].push(n);
    }
    for bucket in buckets.iter_mut() {
        bucket.sort();
    }
    for bucket in buckets {
        for n in bucket {
            data.push(n);
        }
    }
}

pub fn radix_sort(data: &mut Vec<usize>) {
    let mut buckets = vec![vec![]; 10];
    let mut max = match data.iter().max() {
        Some(n) => *n,
        None => return,
    };
    let mut divisor = 1;
    while max != 0 {
        while let Some(n) = data.pop() {
            buckets[n / divisor % 10].push(n);
        } 
        for bucket in buckets.iter_mut() {
            while let Some(n) = bucket.pop() {
                data.push(n);
            }
        }
        divisor = divisor * 10;
        max = max / 10;
    }
}

pub fn count_sort(data: &mut Vec<usize>) {
    let min = match data.iter().min() {
        Some(&n) => n,
        None => return,
    };
    let max = match data.iter().max() {
        Some(&n) => n,
        None => return,
    };
    let len = max - min + 1;
    let mut counts = vec![0; len];
    let mut tmp = Vec::new();
    for n in data.iter() {
        tmp.push(*n);
        counts[*n - min] += 1; 
    }
    for i in 1..counts.len() {
        counts[i] = counts[i] + counts[i - 1];
    }
    for n in tmp {
        data[counts[n - min] - 1] = n;
    }
}

pub fn shell_sort<T: PartialOrd + Copy>(data: &mut Vec<T>) {
    let len = data.len();
    let mut step = data.len();
    while step > 1 {
        step /= 2;
        for i in 0..len - step {
            let mut j = i;
            let tmp = data[j + step];
            while j as isize >= 0 {
                if tmp < data[j] {
                    data[j + step] = data[j];
                    j = (j as isize - step as isize) as usize;
                } else {
                    break;
                }
            }
            data[(j as isize + step as isize) as usize] = tmp;
        }
    }
}

pub fn merge_sort<T: PartialOrd + Copy>(data: &mut Vec<T>) {
    fn recursive_merge_sort<T: PartialOrd + Copy>(data: &mut Vec<T>, temp: &mut Vec<T>, left: usize, right: usize) {
        if right - left <= 1 {
            return;
        }
        let mid = (right - left) / 2 + left;
        recursive_merge_sort(data, temp, left, mid);
        recursive_merge_sort(data, temp, mid, right);
        let mut left_pointer = left;
        let mut right_pointer = mid;
        let mut pointer = left;
        while left_pointer < mid && right_pointer < right {
            if data[left_pointer] > data[right_pointer] {
                temp[pointer] = data[right_pointer];
                right_pointer += 1;
            } else {
                temp[pointer] = data[left_pointer];
                left_pointer += 1;
            }
            pointer += 1;
        }
        while left_pointer < mid {
            temp[pointer] = data[left_pointer];
            left_pointer += 1;
            pointer += 1;
        }
        while right_pointer < right {
            temp[pointer] = data[right_pointer];
            right_pointer += 1;
            pointer += 1;
        }
        pointer = left;
        while pointer < right {
            data[pointer] = temp[pointer];
            pointer += 1;
        }
    }
    if data.len() == 0 {return;}
    let mut temp = vec![data[0]; data.len()];
    recursive_merge_sort(data, &mut temp, 0, data.len());
}

pub fn quick_sort<T: PartialOrd + Copy>(data: &mut Vec<T>) {
    fn recursive_quick_sort<T: PartialOrd + Copy>(data: &mut Vec<T>, left: usize, right: usize) {
        if right - left <= 1 {return;}
        let mut left_pointer = left;
        let mut right_pointer = right - 1;
        let pivot = data[left_pointer];
        while left_pointer < right_pointer {
            while left_pointer < right_pointer && data[right_pointer] >= pivot {
                right_pointer -= 1;
            }
            if left_pointer < right_pointer {
                data.swap(left_pointer, right_pointer);
            }
            while left_pointer < right_pointer && data[left_pointer] <= pivot {
                left_pointer += 1;
            }
            if left_pointer < right_pointer {
                data.swap(left_pointer, right_pointer);
            }
        }
        data[left_pointer] = pivot;
        recursive_quick_sort(data, left, left_pointer);
        recursive_quick_sort(data, left_pointer + 1, right);
    }
    recursive_quick_sort(data, 0, data.len());
}
