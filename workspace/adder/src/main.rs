use add_one;
use add_two;
use rand;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
    println!(
        "Hello, world! {} plus two is {}!",
        num,
        add_two::add_two(num)
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(add_one::add_one(10), 11);
    }
    #[test]
    fn it_plus_2_works() {
        assert_eq!(add_two::add_two(10), 12);
    }
}