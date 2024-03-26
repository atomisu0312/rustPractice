fn rebind() {
  let sum = 0;
  for i in 0..10 {
      // 新しい束縛を作っているので上の束縛には影響がない。
      let sum = sum + i;
  }
  println!("{}", sum); // => 0
}

fn reassign() {
  let mut sum = 0;
  for i in 0..10 {
      // 上の束縛の値を書き換える。
      sum = sum + i;
  }
  println!("{}", sum); // => 45
}

fn main() {
  rebind();
  reassign();
}