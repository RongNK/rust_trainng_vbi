/* ====================================== BÀI SỐ 1 =================================*/
// fn main() {    
//          let x = change_value(10,&mut 20);
//          println!("{}",x)
//      }
    
    
    
//     fn change_value(input:u32, output: &mut u32) -> u32{
//         if input ==1 {
//             *output =3;
//         }
//         else {
//             *output = 4;
//         }    
//         *output
//     }


/* ====================================== BÀI SỐ 2 =================================*/
// Mục đích: giải quyết vấn đề ownership và borrowing ko dùng clone()
// Các bạn có thể sửa thêm logic để đúng với mục đichs bài này là liệt kê các số nguyên tố 
// fn main() {
//     let mut count: u32 = 1;
//     let mut num: u64 = 1;
//     let mut primes: Vec<u64> = Vec::new();

//     while count < 10 {
//         num += 1;
//         if vector_is_prime(num, &primes) {
//             count += 1;
//             primes.push(num);
//         }
//     }
//     println!("{:?}", primes);
// }

// fn vector_is_prime(num: u64, p: &Vec<u64>) -> bool {
//     for i in p {
//         if num > *i && num % i == 0 {
//             return false;
//         }
//     }

//     true
// }

/* ====================================== BÀI SỐ 3 =================================*/
// Mục đích: giải quyết vấn đề ownership and borrowing ko dùng clone()
// fn main() {
//     let mut values = vec![10, 11, 12];
//     let v = &mut values;

//     let mut max = 0;
    
//     for n in v {
//         max = std::cmp::max(max, *n);
//     }

//     println!("max is {}", max);
//     println!("Converting to percentages of maximum value...");

//     for n in &mut values {
//          (*n) = 100 * (*n) / max;
//      }
//     println!("values: {:#?}", values);
// }


/* ====================================== BÀI SỐ 4 =================================*/
// Mục đích : giải quyết vấn đề ownership và borrowing ko dùng clone()
// Logic hiện tại đang sai (cho 1 vec -> đảo chiều vector đó)
fn main(){
    let mut a: Vec<u8> = vec![1,2,3,4,5];
    a = test(&mut a);    
    println!("{:?}",a)
}

pub fn test(a: &mut Vec<u8>) -> Vec<u8> {
    let mut b:Vec<u8>  = Vec::new();
    loop {
        if a.len() == 0 { break; }
        let d = a.pop().unwrap();
        b.push(d);
    }
    b
}