fn main() {
    
    let point = Box::new((0.5, 0.625));
    {
        let label = format!("{:?}", point);
        println!("{}", label);
    }
    println!("{:?}", point);
}
