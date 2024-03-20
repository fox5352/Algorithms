import { binarySearch ,linearSearch} from "./search_algorithms/index.js";
import { bubbleSort, mergeSort, quickSort }  from "./sorting_algorithms/index.js"

function test_binarySearch(sortedArray){
  let target = sortedArray[sortedArray.length - 1]
  
  const start = new Date().getMilliseconds();

  let result = binarySearch(sortedArray, target);

  const end = new Date().getMilliseconds();
  console.log('binary search took '+ (end - start) +'ms');
}

function test_linearSearch(sortedArray){
  let target = sortedArray[sortedArray.length - 1]
  
  const start = new Date().getMilliseconds();

  let result = linearSearch(sortedArray, target);

  const end = new Date().getMilliseconds();
  console.log('linear search took '+ (end - start) +'ms');
}

function test_bubbleSort(unsortedList){
  const start = new Date().getMilliseconds();
  
  bubbleSort(unsortedList);

  const end = new Date().getMilliseconds();
  console.log('bubble sort took '+ (end - start) +'ms');
}

function test_mergeSort(unsortedList) {
  const start = new Date().getMilliseconds();

  mergeSort(unsortedList);

  const end = new Date().getMilliseconds();
  console.log('merge sort took '+ (end - start) +'ms');
}

function test_quickSort(unsortedList) {
  const start = new Date().getMilliseconds();

  quickSort(unsortedList);

  const end = new Date().getMilliseconds();
  console.log('quick sort took '+ (end - start) +'ms');
}

(function main() {
  const sortedList = [];
  const sortCap = 1_000_000;
  for (let i = 0; i < sortCap; i++) {
    // Push the current value of i (incremented by 1) into the array
    sortedList.push(i + 1);
  }
  
  let unsortedList = [];  
  const scrambleList = (unsortedList) => {
    const unsortedCap = 10_000
    for (let index = 0; index < unsortedCap; index++) {
      // push random number into the array
      unsortedList.push(Math.floor(Math.random() * unsortedCap));
    }
  }

  // search algorithms
  console.log('search algorithms');
  test_binarySearch(sortedList);
  test_linearSearch(sortedList);

  console.log('');
  // sort algorithms
  console.log('sort algorithms');

  scrambleList(unsortedList)
  test_mergeSort(unsortedList);

  scrambleList(unsortedList)
  test_quickSort(unsortedList);

  scrambleList(unsortedList)
  test_bubbleSort(unsortedList);
  
})()
