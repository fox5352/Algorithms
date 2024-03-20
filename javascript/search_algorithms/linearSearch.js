/**
 * Performs a linear search on a sorted array
 * @param {Array} array a array to search doesn't need to be sorted
 * @param {*} target The value to search for
 * @returns {number} The index of the target value, or -1 if not found
 */

export default function linearSearch(array, target){
  for(let i = 0; i < array.length; i++){
    if(array[i] === target){
      return i;
    }
  }
  return -1;
}