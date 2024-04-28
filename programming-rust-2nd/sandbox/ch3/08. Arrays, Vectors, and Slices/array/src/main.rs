fn main() {
    let mut sieve = [true; 10000]; // ここで配列を初期化

    // エラストテネスの篩を使用して素数を見つける
    // https://algo-method.com/descriptions/64 
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    
    println!("sieve[211] = {}", sieve[211]);

    // クロージャを使用してtrueの数をカウント
    let true_count = sieve.iter().filter(|&x| *x).count();

    // trueの数を表示
    println!("Number of true values: {}", true_count);
}
