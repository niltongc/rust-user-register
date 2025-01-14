use std::io;

pub fn read_data() -> String {
    let mut data: String = String::new();
    io::stdin().read_line(&mut data).expect("Fail to read data");
    
    let trimmed = data.trim().to_string();
    if trimmed.is_empty(){
        println!("Input cannot be empty!");
        return read_data();
    }
    trimmed
}

pub fn read_data_int() -> i32 {
    let mut data: String = String::new();
    io::stdin().read_line(&mut data).expect("Fail to read data");
    data.trim()
        .parse::<i32>()
        .unwrap_or_else(|_| {
            println!("Invalid input. Please enter a valid integer:");
            read_data_int()
        })
}