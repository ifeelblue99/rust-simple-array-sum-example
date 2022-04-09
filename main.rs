fn main() {
  let arr: [i8;5] = [1, 2, 3, 7, 9];

  let result: i8 = sum_of_elements(arr);
  println!("{:?} - sum -> {}", arr, result);
}

fn sum_of_elements(arr: [i8;5]) -> i8{
  let mut sum: i8 = 0;
  for num in arr.iter(){
    sum+=num;
  }
  sum
}
