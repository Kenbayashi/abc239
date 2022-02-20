fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input)
                    .expect("input failed");

    let num = input.trim()
                   .parse::<i64>()
                   .unwrap();

    println!("{}", num.div_euclid(10));
}
