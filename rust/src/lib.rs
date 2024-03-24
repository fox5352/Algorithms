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

}

pub mod tests {
    use super::*;

    pub fn test_linear_search() {
        let array: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // start timer
        let start = std::time::Instant::now();        

        // do tests
        assert_eq!(search_algorithms::linear_search(&array, 4), 3);
        assert_eq!(search_algorithms::linear_search(&array, 5), 4);
        assert_eq!(search_algorithms::linear_search(&array, 6), 5);
        assert_eq!(search_algorithms::linear_search(&array, 7), 6);
        assert_eq!(search_algorithms::linear_search(&array, 8), 7);
        assert_eq!(search_algorithms::linear_search(&array, 9), 8);
        assert_eq!(search_algorithms::linear_search(&array, 10), 9);

        // stop timer
        let end = std::time::Instant::now();
        let duration = end.duration_since(start);
        println!("Linear search time: {:?}", duration);    
    }

    pub fn test_binary_search() {
        let array: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // start timer
        let start = std::time::Instant::now();        

        // do tests
        assert_eq!(search_algorithms::binary_search(&array, 4), 3);
        assert_eq!(search_algorithms::binary_search(&array, 5), 4);
        assert_eq!(search_algorithms::binary_search(&array, 6), 5);
        assert_eq!(search_algorithms::binary_search(&array, 7), 6);
        assert_eq!(search_algorithms::binary_search(&array, 8), 7);
        assert_eq!(search_algorithms::binary_search(&array, 9), 8);

        // stop timer
        let end = std::time::Instant::now();
        let duration = end.duration_since(start);
        println!("Binary search time: {:?}", duration);
    }

    pub fn test_bubble_sort() {
        // unsorted list from 0 to 20
        let mut array: Vec<i32> = vec![];
        // reverse loop
        for i in (0..20).rev(){
            array.push(i);
        }

        // start timer
        let start = std::time::Instant::now();        

        sorting_algorithms::bubble_sort(&mut array);

        // stop timer
        let end = std::time::Instant::now();
        let duration = end.duration_since(start);
        println!("Bubble sort time: {:?}", duration);


        for i in 0..array.len() {
            assert_eq!(array[i], i as i32);
        }
    }

}
