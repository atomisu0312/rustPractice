fn main() {
    let mut primes = initialize_primes();

    println!("Primes: {:?}", primes);

    primes = add_elements(primes);
    println!("Primes after adding: {:?}", primes);

    let pixel_buffer = new_pixel_buffer(10, 10);
    println!("Pixel buffer: {:?}", pixel_buffer);

    let mut pal = create_palindrome();
    println!("Palindrome: {:?}", pal);

    let v: Vec<i32> = collect_range();
    println!("Collected range: {:?}", v);

    let mut palindrome = create_palindrome_vec();
    palindrome.reverse();
    println!("Reversed palindrome: {:?}", palindrome);

    let mut v = create_vec_with_capacity();
    println!("Vec with capacity: {:?}", v);

    v.push(1);
    v.push(2);
    println!("Vec after pushing 1 and 2: {:?}", v);

    v.push(3);
    println!("Vec after pushing 3: {:?}", v);
    println!("Capacity is now {}", v.capacity());

    let mut v_origin = vec![10, 20, 30, 40, 50];
    let mut v = insert_and_remove_elements(v_origin);
    println!("Vec after insert and remove: {:?}", v);

    let mut v = pop_elements();
    println!("Vec after popping: {:?}", v);

    iterate_over_vector();
}

fn initialize_primes() -> Vec<i32> {
    vec![2, 3, 5, 7]
}

fn add_elements(mut primes: Vec<i32>) -> Vec<i32> {
    primes.push(11);
    primes.push(13);
    primes
}

fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}

fn create_palindrome() -> Vec<String> {
    vec!["step".to_string(), "on".to_string(), "no".to_string(), "pets".to_string()]
}

fn collect_range() -> Vec<i32> {
    (0..5).collect()
}

fn create_palindrome_vec() -> Vec<String> {
    vec!["a man".to_string(), "a plan".to_string(), "a canal".to_string(), "panama".to_string()]
}

fn create_vec_with_capacity() -> Vec<i32> {
    Vec::with_capacity(2)
}

fn insert_and_remove_elements(mut v: Vec<i32>) -> Vec<i32> {
    //let mut v = vec![10, 20, 30, 40, 50];
    v.insert(3, 35);
    v.remove(1);
    v
}

fn pop_elements() -> Vec<String> {
    let mut v = vec!["Snow Puff".to_string(), "Glass Gem".to_string()];
    v
}

fn iterate_over_vector() {
    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!("{}: {}", l,
                 if l.len() % 2 == 0 {
                     "functional"
                 } else {
                     "imperative"
                 });
    }
}
