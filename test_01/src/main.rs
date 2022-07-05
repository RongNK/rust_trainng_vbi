use std::{fs::File, io::ErrorKind};

fn main() {
    let str_file_path = "target/debug/b.txt";
    let _f = File::open(str_file_path);
    let _f = match _f {
        Ok(fl) => fl,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(str_file_path) {
                Ok(new_file) => new_file,
                Err(error) => panic!("Lỗi khi tạo file! {}", error)
            }
            _ => {
                panic!("Chịu rồi!")
            }
        }
    };
}
pub fn test_fn(input: u32) -> u32{
    input + 10
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(test_fn(result), 14);
    }
}
