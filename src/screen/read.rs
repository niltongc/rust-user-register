use std::io;

pub fn read_data() -> String {
    let mut data: String = String::new();
    io::stdin().read_line(&mut data).expect("Fail to read data");
    data.trim().to_string()
}

pub fn read_data_int() -> i32 {
    let mut data: String = String::new();
    io::stdin().read_line(&mut data).expect("Fail to read data");
    data.trim().parse().expect("Error to convert in int")}