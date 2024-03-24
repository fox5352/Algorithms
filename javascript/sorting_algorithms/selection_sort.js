
export default function selectionSort(array) {
  // loop array
  for (let i = 0; i < array.length - 1; i++) {
    let min = i
    // Find the minimum element in the unsorted part of the array
    for (let j = i + 1; j < array.length; j++) {
      // if array j is less than array min
      if (array[j] < array[min]) {
        min = j;
      }
    }
    // Swap the found minimum element with the first element
    const temp = array[i];
    array[i] = array[min];
    array[min] = temp

    // next iteration
  }
}