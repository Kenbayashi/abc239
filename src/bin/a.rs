use std::io;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let x = read::<f64>();

    println!("{}", (x * (12800000f64 + x)).sqrt());
}

fn read<T: FromStr>() -> T {
    io::stdin().bytes()
               .map(|c| c.unwrap() as char)
               .skip_while(|c| c.is_whitespace())
               .take_while(|c| !c.is_whitespace())
               .collect::<String>()
               .parse::<T>()
               .ok()
               .unwrap()
}
