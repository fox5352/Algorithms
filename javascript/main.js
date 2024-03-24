import { binarySearch ,linearSearch} from "./search_algorithms/index.js";
import { bubbleSort, mergeSort, quickSort, selectionSort }  from "./sorting_algorithms/index.js"


function timer_func(func, name){
  const start = Date.now();
  func();
  const end = Date.now();
  const elapsedSeconds = (end - start) / 1000;
  console.log(`${name} took: ${elapsedSeconds.toFixed(6)}s`);
}

//assert function exit if false
function assert_eq(a, b) {
  if (a!== b) {
    throw new Error(`Assertion failed: ${a}!= ${b}`);
  }
}

function test_binarySearch(sortedList) {
  timer_func(()=>{
    assert_eq(binarySearch(sortedList, 4), 3);
    assert_eq(binarySearch(sortedList, 5), 4);
    assert_eq(binarySearch(sortedList, 6), 5);
    assert_eq(binarySearch(sortedList, 7), 6);
    assert_eq(binarySearch(sortedList, 8), 7);
    assert_eq(binarySearch(sortedList, 9), 8);
    assert_eq(binarySearch(sortedList, 10), 9);
    
  }, "Binary search")
}

function test_linearSearch(sortedList) {
  timer_func(()=>{
    assert_eq(linearSearch(sortedList, 4), 3);
    assert_eq(linearSearch(sortedList, 5), 4);
    assert_eq(linearSearch(sortedList, 6), 5);
    assert_eq(linearSearch(sortedList, 7), 6);
    assert_eq(linearSearch(sortedList, 8), 7);
    assert_eq(linearSearch(sortedList, 9), 8);
    assert_eq(linearSearch(sortedList, 10), 9);
    
  }, "Linear search")
}

function test_bubbleSort(unsortedList) {
  timer_func(()=>{
    bubbleSort(unsortedList);
  }, "Bubble sort")
}

function test_selectionSort(unsortedList) {
  timer_func(()=>{
    selectionSort(unsortedList);
  }, "Selection sort")
}

(function main() {
  const sortedList = [];
  const sortCap = 10;
  for (let i = 0; i < sortCap; i++) {
    // Push the current value of i (incremented by 1) into the array
    sortedList.push(i + 1);
  }
  
  let unsortedList = [];
  const scrambleList = (unsortedList) => {
    const unsortedCap = 20
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
  test_bubbleSort(unsortedList);

  scrambleList(unsortedList)
  test_selectionSort(unsortedList);

  // scrambleList(unsortedList)
  // test_mergeSort(unsortedList);

  // scrambleList(unsortedList)
  // test_quickSort(unsortedList);
  
})()
