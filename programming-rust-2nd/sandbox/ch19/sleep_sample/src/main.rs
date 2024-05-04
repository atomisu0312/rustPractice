use std::time::Duration;
use async_std::task;

async fn async_function(n: i32) -> Result<i32, ()>{
    println!("{} seconds waiting start", n);
    // 非同期の操作をここで行います。
    task::sleep(Duration::from_secs(n as u64)).await;
    println!("{} seconds have passed", n);
    Ok(n)
}


fn main() {
    println!("Start");

    async {async_function(3).await};
    println!("Waited!");
}
