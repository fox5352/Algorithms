/**
 * Performs a binary search on a sorted array
 * @param {Array} sortedArray The sorted array to search
 * @param {*} target The value to search for
 * @returns {number} The index of the target value, or -1 if not found
 */

export default function binarySearch(sortedArray, target) {
  let left = 0;
  let right = sortedArray.length - 1;

  while (left <= right) {
    const mid = Math.floor((left + right) / 2);

    if (sortedArray[mid] === target) {
      return mid; // Target found, return its index
    } else if (sortedArray[mid] < target) {
      left = mid + 1; // Target is in the right half
    } else {
      right = mid - 1; // Target is in the left half
    }
  }

  return -1; // Target not found
}