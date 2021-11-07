//If we want to bring all public items defined in a path into scope,
use std::collections::*;


// Complete module paths
use std::cmp::Ordering;
use std::io;
// -- Es lo msmo que lo siguiente solo que anidado--

use std::{cmp::Ordering, io};


use std::io;
use std::io::Write;
// ==>
use std::io::{self, Write};

use std::collections::HashMap;

use std::fmt;

use std::fmt::Result;
use std::io::Result as IoResult;

// fn function3() -> Result {
//     // --snip--
// }
//
// fn function4() -> IoResult<()> {
//     // --snip--
// }
//
// fn function1() -> fmt::Result {
//     // --snip--
// }
//
// fn function2() -> io::Result<()> {
//     // --snip--
// }

use rand::Rng;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    print!("{:?} \n", map);

    let secret_number = rand::thread_rng().gen_range(1..101);
    print!("{} \n\n", secret_number);

}