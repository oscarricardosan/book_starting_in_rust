use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    calculation: T,
    cache_values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            cache_values: HashMap::new()
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        // let result= self.cache_values.entry(arg).or_insert((self.calculation)(arg));
        // *result

        let result= self.cache_values.get(&arg);
        match result {
            Some(value) => *value,
            None => {
                self.cache_values.insert(arg, (self.calculation)(arg));
                *self.cache_values.get(&arg).unwrap()
            }
        }

    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(60*2*3600));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a|{
        println!("entre {}", a);
        a * 2
    });

    let v1 = c.value(1);
    let v2 = c.value(2);
    c.value(1);
    c.value(22);
    c.value(22);
    c.value(22);
    c.value(22);
    c.value(22);
    c.value(22);
    c.value(22);
    c.value(3);
    c.value(3);
    c.value(3);
    c.value(3);
    c.value(3);
    c.value(3);
    c.value(3);
    c.value(4);
    c.value(1);

    println!("xxxxxxx");
    assert_eq!(v1, 2);
    assert_eq!(v2, 4);
}
