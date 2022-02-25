use proconio::input;

fn main() {
    input! {
        point1: (i32, i32),
        point2: (i32, i32),
    }

    let knights1 = knights(point1);
    let knights2 = knights(point2);

    let ans = knights1.into_iter()
                      .map(|point| knights2.contains(&point))
                      .reduce(|a, b| a || b)
                      .unwrap();

    println!("{}", if ans {"Yes"} else {"No"});
}

fn knights(point: (i32, i32)) -> Vec<(i32, i32)> {
    let (x, y)  = point;
    let knights = vec![(1, 2), (2, 1), (-1, 2), (-2, 1), (-1, -2), (-2, -1), (-2, -1), (-1, -2)];

    knights.into_iter()
           .map(|(a, b)| (x + a, y + b))
           .collect::<Vec<(i32, i32)>>()
}
