fn main() {
    struct Person { name: String, birth: i32 }

    let mut composers = Vec::new();
    composers.push(Person { name: "Palestrina".to_string(),
                            birth: 1525 });
    composers.push(Person { name: "Dowland".to_string(),
                            birth: 1563 });
    composers.push(Person { name: "Lully".to_string(),
                            birth: 1632 });

    // composersの所有権をループ内で移動させず、参照を使用してアクセスします
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }

    // composersはまだ利用可能です
    println!("Number of composers: {}", composers.len());

    // 参照は使わない
    for composer in composers {
        println!("{}, born {}", composer.name, composer.birth);
    }

    // この部分をコメントアウトしなければならない理由は次のとおり。
    // composersはループ内で所有権が移動されているため、
    let mut  primes = initialoze_primes();
    println("Primes: {:?}", primes);

}

fu initialize_primes() -> Vec<i32> {
    vec![2,3,5,7]
}

