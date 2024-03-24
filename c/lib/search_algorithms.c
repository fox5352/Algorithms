#include "search_algorithms.h"

int binary_search(int arr[], int low, int high, int target){
  
  while (low <= high){
    /* code */
    int mid = (low + high) / 2;

    if (arr[mid] == target){
      return mid;
    }else if (arr[mid] < target){
      low = mid + 1;
    }else{
      high = mid - 1;
    }
  }

  return -1;
}

int linear_search(int arr[], int length, int target){
  for (int i = 0; i < length; i++){
    if (arr[i] == target){
      return i;
    }
  }

  return -1;
}
