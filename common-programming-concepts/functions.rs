fn main() {
    another_function(8);

    let x = five();

    let y = plus_one(x);

    println!("fn five() op: {x}\nfn plus_one() op: {y}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
