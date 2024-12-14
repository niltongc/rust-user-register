use crate::screen::read;
use crate::screen::basic_operation::*;

pub fn show_menu(){

    
    loop{
        clear_screen();
        println!("\
            ========== Menu ==========\n\
            Choose options:\n\n\
            1 - Register client\n\
            2 - Update client\n\
            3 - Remove client\n\
            4 - List clients\n\
            0 - Logout\n\
        ");

        let option: i32 = read::read_data_int();
        match option {
            1 => println!("Option 1"),
            2 => println!("Option 2"),
            3 => println!("Option 3"),
            4 => println!("Option 4"),
            0 => {
                println!("shutdown ...");
                return;
            },
            _ => println!("invalid options"),
        }

        println!("Press key to continue ...");
        // read::read_data();
        wait_time(2);
    }
}