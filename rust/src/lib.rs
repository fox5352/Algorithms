pub mod search_algorithms {
    pub fn linear_search(array: &[i32], target: i32) -> i32 {
        for i in 0..array.len() {
            if array[i] == target {
                return i as i32;
            }
        }
        return -1;
    }

    pub fn binary_search(sorted_array: &[i32], target: i32) -> i32 {
        let mut left = 0;
        let mut right = sorted_array.len() - 1;

        // loop while left < right
        while left <= right {
            let mid = left + (right - left) / 2;

            // if mid is target then return mid
            if sorted_array[mid] == target {
                return mid as i32;
            } // if mid is less than target left is mid
            else if sorted_array[mid] < target {
                left = mid + 1;
            } // if mid is greater than target right is mid
            else {
                right = mid - 1;
            }
        }

        // not found
        return -1;
    }
}

pub mod sorting_algorithms {
    pub fn bubble_sort(array: &mut [i32]) {
        // loop list
        for i in 0..array.len() {
            // loop from 0 to sorted chunk
            for j in 0..(array.len() - i - 1) {
                if array[j] > array[j + 1] {
                    let temp = array[j];
                    array[j] = array[j + 1];
                    array[j + 1] = temp;
                }
            }
        }
    }

    pub fn selection_sort(array: &mut [i32]) {
        // loop array
        for i in 0..(array.len())-1 {
            let mut min = i;

            // Find the minimum element in the unsorted part of the array
            for j in (i+1)..array.len() {
                // if arr index j < arr index min
                if  array[j] < array[min]{
                    min = j;
                }
            }
      
            // Swap the found minimum element with the first element
            let temp = array[i];
            array[i] = array[min];
            array[min] = temp
            // then start from next index on the next loop
        }
    }

    fn partition(array: &mut [i32], lo: i32, hi: i32) -> i32 {
        let pivot = array[hi as usize];

        let mut idx = lo - 1;

        // weak sort
        for i in lo..hi {
            if array[i as usize] < pivot {
                idx += 1;
                let temp = array[i as usize];
                array[i as usize] = array[idx as usize];
                array[idx as usize] = temp;
            }
        }

        // swap pivot index
        idx += 1;
        array[hi as usize] = array[idx as usize];
        array[idx as usize] = pivot;

        return idx;
    }

    fn qs(array: &mut [i32], lo: i32, hi: i32) {
        if lo >= hi  {
            return;
        }

        let pivot = partition(array, lo, hi);

        qs(array, lo, pivot - 1);
        qs(array, pivot + 1, hi);
    }

    pub fn quick_sort(array: &mut [i32]){
        qs(array, 0, (array.len() as i32) - 1);
    }

}

pub mod tests {
    use super::*;

    // create a wrapper function called timer
    fn timer<F>(func: F, name: &str)
        where F: FnOnce(){
        let start = std::time::Instant::now();
        func();
        let end = std::time::Instant::now();
        // print time in seconds
        println!("{} took: {:.6}s", name, end.duration_since(start).as_secs_f64());

    }

    pub fn test_linear_search() {
        let array: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        
        timer(|| {
            assert_eq!(search_algorithms::linear_search(&array, 4), 3);
            assert_eq!(search_algorithms::linear_search(&array, 5), 4);
            assert_eq!(search_algorithms::linear_search(&array, 6), 5);
            assert_eq!(search_algorithms::linear_search(&array, 7), 6);
            assert_eq!(search_algorithms::linear_search(&array, 8), 7);
            assert_eq!(search_algorithms::linear_search(&array, 9), 8);
            assert_eq!(search_algorithms::linear_search(&array, 10), 9);
        }, "linear search")
    }

    pub fn test_binary_search() {
        let array: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        
        timer(|| {
            assert_eq!(search_algorithms::binary_search(&array, 4), 3);
            assert_eq!(search_algorithms::binary_search(&array, 5), 4);
            assert_eq!(search_algorithms::binary_search(&array, 6), 5);
            assert_eq!(search_algorithms::binary_search(&array, 7), 6);
            assert_eq!(search_algorithms::binary_search(&array, 8), 7);
            assert_eq!(search_algorithms::binary_search(&array, 9), 8);
        }, "binary search")
    }

    pub fn test_bubble_sort() {
        // unsorted list from 0 to 20
        let mut array: Vec<i32> = vec![];
        // reverse loop
        for i in (0..20).rev(){
            array.push(i);
        }

        timer(|| {
            sorting_algorithms::bubble_sort(&mut array);
        }, "bubble sort");

        for i in 0..array.len() {
            assert_eq!(array[i], i as i32);
        }
    }

    pub fn test_selection_sort() {
        // unsorted list from 0 to 20
        let mut array: Vec<i32> = vec![];
        // reverse loop
        for i in (0..20).rev(){
            array.push(i);
        }

        timer(|| {
            sorting_algorithms::selection_sort(&mut array);
        }, "selection sort");

        for i in 0..array.len() {
            assert_eq!(array[i], i as i32);
        }
    }

    pub fn test_quick_sort() {
        // unsorted list from 0 to 20
        let mut array: Vec<i32> = vec![];
        // reverse loop
        for i in (0..20).rev(){
            array.push(i);
        }

        timer(|| {
            sorting_algorithms::quick_sort(&mut array);
        }, "quick sort");

        for i in 0..array.len() {
            assert_eq!(array[i], i as i32);
        }
    }

}
