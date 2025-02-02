
use crate::models::client::Client;
use crate::screen::basic_operation::clear_screen;
use crate::screen::read::{read_data, read_data_int};
use crate::screen::basic_operation::wait_time;

pub fn insert_client(clients: &mut Vec<Client>){
    clear_screen();
    let mut client: Client = Client::default();

    client.id = clients.len() + 1;

    insert_client_data(&mut client);

    clients.push(client);

    clear_screen();
    println!("Registered successfully");
    wait_time(1);
}

fn insert_client_data(client: &mut Client) {
    println!("insert name");
    client.name = read_data();

    println!("insert cpf");
    client.cpf = read_data();
    
    println!("insert address");
    client.address = read_data();
}

pub fn update_client(clients: &mut Vec<Client>){
    clear_screen();

    if no_clients(clients){
        return;
    }

    let id = get_id();

    if let Some((index, client)) = get_client_by_id(clients, &id){
        println!("{}", "-".to_string().repeat(40));
        println!("Updating client");
        println!("{}", "-".to_string().repeat(40));
        show_client(client);
        println!("{}", "-".to_string().repeat(40));
        insert_client_data(&mut clients[index]);
        clear_screen();
        println!("Update OK")

    } else {
        clear_screen();
        println!("Client not found")
    }
}

fn get_client_by_id<'a>(clients: &'a Vec<Client>, id: &i32) -> Option<(usize, &'a Client)> {
    clients
        .iter()
        .enumerate()
        .find(|(_, client)| client.id == *id as usize)
}


fn get_id() -> i32 {
    clear_screen();

    println!("insert client id");

    read_data_int()
}

pub fn list_clients(clients: &mut Vec<Client>){
    clear_screen();

    if no_clients(clients){
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

fn show_client(client: &Client){
    println!("\
    ID: {}\n\
    Name {}\n\
    CPF: {}\n\
    Address: {}
    ", client.id, client.name, client.cpf, client.address)
}

fn no_clients(clients: &mut Vec<Client>) -> bool{
    if clients.len() == 0 {
        println!("No registers found");
        wait_time(1);
        return true;
    }
    return false;
}

pub fn delete_client(clients: &mut Vec<Client>){
    clear_screen();

    if no_clients(clients){
        return;
    }

    let id = get_id();

    if let Some((index, client)) = get_client_by_id(clients, &id){
        println!("{}", "-".to_string().repeat(40));
        println!("Are you sure to delete client?");
        println!("{}", "-".to_string().repeat(40));
        show_client(client);
        println!("{}", "-".to_string().repeat(40));
        println!("y - YES\nn - NO");
        let choosed = read_data();
        if choosed == "y"{
            clients.remove(index);
            clear_screen();
            println!("Client deleted")
        } else {
            clear_screen();
            println!("Not deleted")
        }
        

    } else {
        clear_screen();
        println!("Client not found")
    }
}