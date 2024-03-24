#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <time.h>

// --------------- search algorithms --------------------
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

// --------------- sort algorithms --------------------
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

// --------------- test functions --------------------
void test_binary_search(){
  int arr[10];
  for (size_t i = 0; i < 10; i++){
    arr[i] = i+1;
  }
  int low = 0;
  int high = (sizeof(arr) / sizeof(arr[0]));

  clock_t start = clock();

  assert(binary_search(arr, low, high, 5) == 4);
  assert(binary_search(arr, low, high, 6) == 5);
  assert(binary_search(arr, low, high, 7) == 6);
  assert(binary_search(arr, low, high, 8) == 7);
  assert(binary_search(arr, low, high, 9) == 8);
  assert(binary_search(arr, low, high, 10) == 9);

  clock_t end = clock();
  printf("%s: %fs\n", "binary search", (double)(end - start) / CLOCKS_PER_SEC);
}

void test_linear_search(){
  int arr[10];
  for (size_t i = 0; i < 10; i++){
    arr[i] = i+1;
  }
  int length = sizeof(arr) / sizeof(arr[0]);

  // start timer
  clock_t start = clock();
 
  assert(linear_search(arr, length, 5) == 4);
  assert(linear_search(arr, length, 6) == 5);
  assert(linear_search(arr, length, 7) == 6);
  assert(linear_search(arr, length, 8) == 7);
  assert(linear_search(arr, length, 9) == 8);
  assert(linear_search(arr, length, 10) == 9);


  // stop timer
  clock_t end = clock();
  printf("%s: %fs\n", "liner search", (double)(end - start) / CLOCKS_PER_SEC);
}

void test_bubble_sort(){
  int arr[20];
  for (size_t i = 20; i > 0 ; --i){
    arr[i-1] = i;
  }
  int length = sizeof(arr) / sizeof(arr[0]);

  // start timer
  clock_t start = clock();

  bubble_sort(arr, length);

  // stop timer
  clock_t end = clock();
  printf("%s: %fs\n", "bubble sort", (double)(end - start) / CLOCKS_PER_SEC);

  for (size_t i = 0; i < 20; i++){
    assert(arr[i] - 1 == i);
  }
  
}

// --------------- main function --------------------
int main(int argc, char **argv){
  // searching algorithms
  test_linear_search();
  test_binary_search();

  // sorting algorithms
  test_bubble_sort();

  printf("EOP\n");
  return 0;
}