extern crate mod_learn;

use mod_learn::english;
use mod_learn::japanese;
use mod_learn::my_lib;
fn main() {
    println!("哈喽用英语说{}", english::greetings::hello());
    println!("哈喽用日语说{}", japanese::greetings::hello());
    my_lib::act_mod::run();
}
