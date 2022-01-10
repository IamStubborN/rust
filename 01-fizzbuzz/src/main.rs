use std::vec::Vec;

/*
   Напишите программу, которая выводит на экран числа от 1 до 100.
   При этом вместо чисел, кратных трем, программа должна выводить слово Fizz,
   а вместо чисел, кратных пяти — слово Buzz.
   Если число кратно пятнадцати, то программа должна выводить слово FizzBuzz.
*/

fn main() {
    const COUNT: usize = 100;
    let mut result = Vec::with_capacity(COUNT);
    for i in 1..=COUNT {
        let s = match (i % 3, i % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            _ => i.to_string(),
        };

        result.push(s);
    }

    print_result(&result);
}

fn print_result(result: &[String]) {
    for r in result {
        println!("{}", r)
    }
}
