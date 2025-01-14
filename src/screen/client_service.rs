
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

pub fn list_clients(clients: &mut Vec<Client>){
    clear_screen();

    if clients.len() == 0 {
        println!("No registers found");
        wait_time(1);
        return;
    }

    println!("{}", "-".to_string().repeat(40));

    for client in clients{
        show_client(client);
        println!("{}", "-".to_string().repeat(40));
    }

    println!("Press Enter to continue");
    read_data();
}

fn show_client(client: &mut Client){
    println!("\
    ID: {}\n\
    Name {}\n\
    CPF: {}\n\
    Address: {}
    ", client.id, client.name, client.cpf, client.address)
}