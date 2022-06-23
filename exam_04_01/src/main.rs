// Bài 1: Trait

// #[derive(Debug)]
// struct Fibonacci {
//     a: u32,
//     b: u32,
// }

// fn fibonacci_numbers() -> Fibonacci {
//     Fibonacci { a: 1, b: 0 }
// }

// impl Iterator for Fibonacci {
//     type Item = u32;
//     fn next(&mut self) -> Option<Self::Item> {
//         let new_next = self.a + self.b;
//         self.b = self.a;
//         self.a = new_next;
//         Some(self.b)
//     }        
// }

// fn main(){
//     for number in fibonacci_numbers() {
//         println!("{}", number);
//     }
// }


// Bài 2: Lifetime
// Yêu cầu: Sửa lỗi Lifetime 

use std::fmt;
struct StrDisplayable(Vec<String>);

impl fmt::Display for StrDisplayable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in &self.0 {
            write!(f, "\n{}", v)?;
        }
        Ok(())
    }
}

fn main() {
    let vec: Vec<String> = vec!["a".to_string(),"bc".to_string(),"def".to_string()];
    let vec_Foo = StrDisplayable(vec);
    println!("{}",vec_Foo);
}