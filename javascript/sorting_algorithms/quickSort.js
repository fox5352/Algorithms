

export default function quickSort(arr) {
  if (arr.length <= 1) {
    return arr
  }

  const pivot = arr[Math.floor(arr.length/2)];
  const left = [];
  const right = [];

  for (let i = 0; i < arr.length - 1; i++) {
        if (arr[i] < pivot) {
          left.push(arr[i]);
        }else {
          right.push(arr[i]);
        }
  }

  return [...quickSort(left), pivot,...quickSort(right)]
}