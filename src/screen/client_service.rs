
use crate::models::client::Client;
use crate::screen::basic_operation::clear_screen;
use crate::screen::read::read_data;
use crate::screen::basic_operation::wait_time;

pub fn insert_client(clients: &mut Vec<Client>){
    clear_screen();
    let mut client: Client = Client::default();

    client.id = clients.len() + 1;

    println!("insert name");
    client.name = read_data();

    println!("insert cpf");
    client.cpf = read_data();
    
    println!("insert address");
    client.address = read_data();

    clients.push(client);

    clear_screen();
    println!("Registered successfully");
    wait_time(1);
}