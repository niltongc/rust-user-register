use crate::screen::client_service::delete_client;
use crate::screen::client_service::list_clients;
use crate::screen::client_service::update_client;
use crate::screen::read;
use crate::screen::basic_operation::*;

use crate::models::client::Client;
use crate::screen::client_service::insert_client;

pub fn show_menu(clients: &mut Vec<Client>){

    
    loop{
        
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
            1 => insert_client(clients),
            2 => update_client(clients),
            3 => delete_client(clients),
            4 => list_clients(clients),
            0 => {
                println!("shutdown ...");
                return;
            },
            _ => println!("invalid options"),
        }

        //println!("Press key to continue ...");
        // read::read_data();
   
    }
}