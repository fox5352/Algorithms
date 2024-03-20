

export default function bubbleSort(arr){
  const n = arr.length;

  // loop hole list
  for (let i = 0; i < n; i++) {

    // loop from 0 to sorted chunk
    for (let j = 0; j < n - i - 1; j++) {

      // if left is bigger than right swap the values
      if (arr[j] > arr[j + 1]) {
        let temp = arr[j];
        arr[j] = arr[j + 1];
        arr[j + 1] = temp;
      }
    }
  }
}