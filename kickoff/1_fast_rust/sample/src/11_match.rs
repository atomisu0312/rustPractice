use std::env;

fn exam1(a: isize) {
  match a {
      // リテラルへのマッチ
      0 => println!("0"),
      // レンジパターンが書ける。
      1..=10 => println!("small number"),
      // 変数パターンで受けると残りの可能性すべてを受け、変数をその値に束縛する。
      n => println!("big number: {}", n),
  }
}

fn exam2(a: isize, b: isize) {
  match (a, b) {
      // タプルパターンでタプルを分解できる。さらにパターンの入れ子もできる。
      (0,0) => println!("all zero"),
      // 部分的に変数パターンを使うこともできる。
      // この例では、第２引数が0..=10の範囲に収まっていれば、こちらが実行される
      (f, 0..=10) => println!("int 1st: {} with small number", f),
      
      // if文でマッチング条件を表現することもできる
      (a_val,b_val) if a_val >= 0 && a_val <= 10 => println!("int 2nd: {} with small number", b_val),
      
      // もちろん丸ごと変数で受け取ることもできる。
      // 値を特に使わないのであれば特別なパターン`_`を使うことで無視できる。
      _ => println!("other tuple"),
  }
}

fn main() {
  // 引数をベクトルして受け取る
  let args: Vec<String> = env::args().collect();
  // how to USE
  if args.len() != 4 {
    println!("How to Use is");
    println!("{} a b c", args[0]);
    println!("a: an arg of exam1");
    println!("b, c: args of exam2");
    return;
  }
  // 引数に対して型の変換を行っている
  let a: isize = args[1].parse().expect("Failed to parse first argument");
  let b: isize = args[2].parse().expect("Failed to parse second argument");
  let c: isize = args[3].parse().expect("Failed to parse second argument");
  
  // 関数をバインドしている 
  let f: fn(isize) = exam1;
  f(a);
  let g: fn(isize, isize) = exam2;
  g(b, c);
}