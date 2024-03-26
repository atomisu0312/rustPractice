use std::ops::Add;
use std::env;
use std::error::Error;

// 引数の解釈
fn process_args() -> Result<Vec<String>, String> {
  let args: Vec<String> = env::args().collect();

  if args.len() != 5 {
    println!("How to Use");
    println!("{} x1 y1 x2 y2\n", args[0]);
    return Err("引数の数が正しくありません。".to_string());
  }

  Ok(args)
}

#[derive(Debug)]
struct Point(i32, i32);

impl Add for Point {
  type Output = Point;

  fn add(self, rhs: Point) -> Point {
    Point(self.0 + rhs.0, self.1 + rhs.1)
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let args = process_args()?;

  // コマンドライン引数からPointインスタンスを作成
  let point1 = Point(args[1].parse().unwrap(), args[2].parse().unwrap());
  let point2 = Point(args[3].parse().unwrap(), args[4].parse().unwrap());
  // Pointインスタンスを加算
  let result = point1 + point2;

  println!("{:?}", result);  // => Point(4, 6)

  Ok(())
}