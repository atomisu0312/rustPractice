use std::ops::AddAssign;

// 型 T は Clone トレイト（T.clone()）と AddAssign<T> トレイト（S+=T）を
// 実装している必要があります。
fn sum<T: Clone + AddAssign<T>>(v: &Vec<T>, init: T) -> T {
  let mut result = init;
  for x in v {
    result += x.clone();
  }
  result
}

fn main() {
  let v = vec![10, 20, 30, 40];
  // sum<i32, i32> を呼び出します。
  println!("{}", sum(&v, 0));  // => 100

  let v = vec![0.01, 0.02, 0.03, 0.04];
  // sum<f64, f64> を呼び出します。
  println!("{}", sum(&v, 0.0));  // => 0.1
}