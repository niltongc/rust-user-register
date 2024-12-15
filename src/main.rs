
mod screen;
mod models;

use screen::menu as menu;
use crate::models::client::Client;

fn main() {
  let clients: &mut Vec<Client> = &mut Vec::new();
  menu::show_menu(clients);
}
