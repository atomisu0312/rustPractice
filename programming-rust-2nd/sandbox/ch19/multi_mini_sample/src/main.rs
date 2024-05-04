use async_std::task;
use std::time::Duration;

async fn async_function(n: i32) -> Result<i32, ()>{
    println!("{} seconds waiting start", n);
    // 非同期の操作をここで行います。
    task::sleep(Duration::from_secs(n as u64)).await;
    println!("{} seconds have passed", n);
    Ok(n)
}

fn main() {
    let handles = vec![
        task::spawn(async_function(1)),
        task::spawn(async_function(2)),
    ];

    let results = task::block_on(async {
        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.await);
        }
        results
    });

    println!("The results are {:?}", results);
}