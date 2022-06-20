use std::collections::HashMap;
use std::io;

struct School<T>{
    students: HashMap<String, T>
}

impl<T: std::cmp::Ord + Copy> School<T>{
    fn new() -> Self{
        Self{
            students: HashMap::new()
        }
    }
    fn add(&mut self,name: String, point: T){
        self.students.insert(name, point);
    }
    fn point_list(&self) -> Vec<T>{
        let mut tmp_vec = Vec::new();
        // Add all point of students         
        for val in self.students.values() {
            tmp_vec.push(*val)
        }            
        // Sort vector
        tmp_vec.sort();
        // Remove duplicate
        tmp_vec.dedup();
        tmp_vec
    }
    fn find_student(&self, point: T) -> Vec<String>{
        let mut tmp_list: Vec<String> =  Vec::new();
        for (key, value) in &self.students {
            if *value == point{
                tmp_list.push(key.clone())
            }
        }
        // Sort vector
        tmp_list.sort();
        // Remove duplicate
        tmp_list.dedup();
        tmp_list
    }
}


fn main() {
    // Case 01
    let mut student_list = School::new();

    // Case 02
    student_list.add("Lee".to_string(), "Rong");    
    student_list.add("Nancy".to_string(), "Rong");    
    student_list.add("Pim".to_string(),"Ran");           
    println!("Liệt kê các điểm số hiện tại mà trường đã cập nhập {:?}",student_list.point_list());

    // Case 03
    let mut input_text = String::new();
    println!("Input Point to search:");
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    println!("Liệt kê danh sách các học sinh có cùng tham số value {}: {:?}", trimmed, student_list.find_student(trimmed));
}

