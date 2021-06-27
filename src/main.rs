use mouse_rs::Mouse;
use std::thread;

fn main() {
    println!("Hello, world!");

    while true {
        move_mouse();
        thread::sleep_ms(60000);
    }
}

fn move_mouse() {
    let mouse = Mouse::new();
    let x = mouse.get_position().unwrap().x as i32;
    let y = mouse.get_position().unwrap().y as i32;
    mouse.move_to(x + 1, y + 1).expect("Unable to move mouse");
}