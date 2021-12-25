fn main() {
    let arr = [1, 2, 3, 4, 5, 6];

    let squared = arr.map(|x: i32| x.pow(2));
    println!("{:?}", squared);
}
