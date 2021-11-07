use std::thread;
use std::time::Duration;

fn main() {
    /*let v1 = vec![1, 2, 3];
    let mut count:u32 = 0;

    loop{
        count+=1;
        println!("{}", count);
    }*/

    let v1 = vec![1, 2, 3];

    let total: i32 = v1.iter().sum();
    println!("Sum: {}", total);

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
        // thread::sleep(Duration::from_secs(1));
    }

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("Got: {:?}", v1);

}
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
