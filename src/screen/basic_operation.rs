use std::time::Duration;
use std::thread::sleep;

pub fn clear_screen(){
    clearscreen::clear().expect("Fail to clear the screen")
}

pub fn wait_time(wtime: u64){
    sleep(Duration::from_secs(wtime))
}