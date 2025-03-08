fn main() {
    let mut numbers = vec![1, 2, 3];
    let target = 6;

    for num in numbers.iter() {
        if *num == target {
            println!("{}", num);
            break;
        }
    }
}
