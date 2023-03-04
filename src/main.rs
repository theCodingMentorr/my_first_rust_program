
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main () {
    let x = add(1, 1);
    let _y = add(3, 0);
    let _z = add(x, 1); 

    let _life = 42;

    println!("The Meaning of Life is: {:?}", _life);

}
