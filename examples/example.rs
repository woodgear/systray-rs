extern crate systray;
use std::{thread, time};
use systray::SystrayEvent;
use std::sync::mpsc::{channel,Sender};
fn get_reciver() -> Result<Sender<SystrayEvent>,String> {
    let (event_tx, event_rx) = channel();
    let event_tx_clone = event_tx.clone();
    thread::spawn(move || {
        let mut tray;
        match systray::Application::new(event_tx_clone) {
            Ok(w) => tray = w,
            Err(_) => panic!("Can't create window!")
        }
        tray.set_icon_from_file("./resources/rust.ico".to_string()).ok();
        tray.add_menu_item(&"Print a thing".to_string(), |_| {
            println!("Printing a thing!");
        }).ok();
        tray.add_menu_item(&"Add Menu Item".to_string(), |window| {
            window.add_menu_item(&"Interior item".to_string(), |_| {
                println!("what");
            }).ok();
            window.add_menu_separator().ok();
        }).ok();
        tray.add_menu_separator().ok();
        tray.add_menu_item(&"Quit".to_string(), |window| {
            window.quit();
        }).ok();
        let _ = tray.set_tooltip(&"test tips".to_string());

        loop {
            println!("in loop");
            let msg;
            match event_rx.recv() {
                Ok(m) => msg = m,
                Err(e) => {
                    println!("get a err of tray {:?}",e);
                    break;
                }
            }
            match msg {
                SystrayEvent::ShowIcon =>{
                    let _ = tray.show_icon().map_err(|e|{
                        println!("show icon err {:?}",e);
                    });
                    println!("i should show the icon");
                }
                SystrayEvent::HideIcon =>{
                    let _ = tray.hide_icon().map_err(|e|{
                        println!("hide icon err {:?}",e);
                    });
                    println!("i should hide the icon");
                }
                SystrayEvent::LeftButtonClick =>{
                    println!("recv LeftButtonClick");
                }
                SystrayEvent::MenuItemClick(menu_index) =>{
                    println!("recv MenuItemClick {}",menu_index);
                }
            }
        }
    });
    return Ok(event_tx.clone());
}

fn main() {
    let sender = get_reciver().unwrap();
    loop {
        thread::sleep(time::Duration::from_secs(3));
        let _ = sender.send(SystrayEvent::ShowIcon);
        thread::sleep(time::Duration::from_secs(3));
        let _ = sender.send(SystrayEvent::HideIcon);
    }
}