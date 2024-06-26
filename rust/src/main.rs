use rust::tests::{test_binary_search, test_linear_search, test_bubble_sort, test_selection_sort, test_quick_sort};

fn main() {
  // testing search algorithms
  test_linear_search();
  test_binary_search();

  // testing sorting algorithms
  test_bubble_sort();
  test_selection_sort();
  test_quick_sort();

  println!("EOP");
}