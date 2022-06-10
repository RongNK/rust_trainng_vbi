fn main() {
    let array_root:[i32;10] = [12,3,6,23,78,23,5,12,4,1];
    let array_test:[i32;3] = [1,4,5];

    println!("Root array: {:?}",array_root);
    println!("Test array: {:?}",array_test);
    let mut tmp_index_01 = 0;

    // How to resolve this exam? 
    // These steps are:
    // 1. Convert root and test array to Strings
    // 2. Check:  root string contains test string
    // 3. Show the result
    let mut str_root = "".to_string();
    let mut str_test = "".to_string();
    while tmp_index_01 < array_test.len(){
        str_test.push_str("_");
        str_test.push_str(&array_test[tmp_index_01].to_string());
        tmp_index_01 += 1;
    }
    tmp_index_01 = 0;
    while tmp_index_01 < array_root.len(){
        str_root.push_str("_");
        str_root.push_str(&array_root[tmp_index_01].to_string());
        tmp_index_01 += 1;
    }
    if str_root.contains(&str_test){
        println!("The test array is a child of root array!")
    }else{
        println!("The test array is not a child of root array!")
    }
}