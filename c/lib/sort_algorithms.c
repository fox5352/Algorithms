#include "sort_algorithms.h"

#include <stdio.h>
#include <stdlib.h>

void bubble_sort(int *arr, int length){
  for (size_t i = 0; i < length; i++){
    // loop until sorted chunk
    for (size_t j = 0; j < length - i - 1; j++){
      // if left less then right swap up
      if (arr[j] > arr[j + 1]){
        int temp = arr[j];
        arr[j] == arr[j + 1];
        arr[j + 1] = temp;
      }
    }
  }
}

void selection_sort(int *arr, int length) {  
  // Iterate through the array
  for (int i = 0; i < length - 1; i++) {
      int min = i;
  
      // Find the minimum element in the unsorted part of the array
      for (int j = i + 1; j < length; j++) {
        // if arr index j < arr index min
        if (arr[j] < arr[min]){
          min = j;
        }
      }
      
      // Swap the found minimum element with the first element
      int temp = arr[i];
      arr[i] = arr[min];
      arr[min] = temp;
      
      // then start from next index on the next loop
  }
}

int partition(int *arr, int low, int hi){
  int pivot = arr[hi];

  int idx = low - 1;

  // weak sor list
  for (size_t i = low; i < hi; i++){
    if (arr[i] < pivot){
      idx++;
      int temp = arr[i];
      arr[i] = arr[idx];
      arr[idx] = temp;
    }
    
  }

  // swap pivots index
  idx++;
  arr[hi] = arr[idx];
  arr[idx] = pivot;
  
  return idx;
}


void quick_sort(int *arr, int low, int hi){
  if (low >= hi) return;

  int pivot = partition(arr, low, hi);

  quick_sort(arr, low, pivot - 1);
  quick_sort(arr, pivot + 1, hi);  
}