extern crate systray;
use std::{thread, time};
use systray::{SystrayEvent,IconResource};
use std::sync::mpsc::{channel,Sender,Receiver};

pub enum ControlEvent{
    ShowIcon,
    HideIcon
}

fn create_tray(control_recviver:Receiver<ControlEvent>,tray_event_sender:Sender<SystrayEvent>) -> Result<(),String> {
    thread::spawn(move || {
        let mut tray;
        match systray::Application::new(tray_event_sender) {
            Ok(w) => tray = w,
            Err(_) => panic!("Can't create window!")
        }
        tray.set_tooltip("中文".to_string());
        loop {
            let msg;
            match control_recviver.recv() {
                Ok(m) => msg = m,
                Err(e) => {
                    println!("get a err of tray {:?}",e);
                    break;
                }
            }
            match msg {
                ControlEvent::ShowIcon =>{
                    let _ = tray.show_icon(IconResource::File("./rust.ico".to_string())).map_err(|e|{
                        println!("show icon err {:?}",e);
                    });
                    println!("i should show the icon");
                }
                ControlEvent::HideIcon =>{
                    let _ = tray.hide_icon().map_err(|e|{
                        println!("hide icon err {:?}",e);
                    });
                    println!("i should hide the icon");
                }
            }
        }
    });
    Ok(())
}

#[derive(Debug)]
pub struct TTray {
    control_sender:Sender<ControlEvent>
}
unsafe impl Sync for TTray {}

impl TTray {
    pub fn new() -> TTray {
        let (control_sender,control_recviver) = channel();
        let (tray_sender,tray_recviver) = channel();
        create_tray(control_recviver,tray_sender);
        thread::spawn(move|| {
            loop {
                match tray_recviver.recv() {
                    Ok(m) => {
                        match m {
                            SystrayEvent::LeftButtonClick => {
                                println!("left button click");
                            },
                            SystrayEvent::MenuItemClick(menu_index) => {
                            }
                        }
                    },
                    Err(e) => {
                        println!("TTray get a err of tray {:?}",e);
                    }
                }
            }
        });
        TTray{control_sender:control_sender}
    }

    pub fn show(&self){
        self.control_sender.send(ControlEvent::ShowIcon);
    }
    pub fn hide(&self){
        self.control_sender.send(ControlEvent::HideIcon);
    }
}

fn main() {
    let tray = TTray::new();
    loop {
        tray.show();
        thread::sleep(time::Duration::from_secs(5));
        tray.hide();
        thread::sleep(time::Duration::from_secs(5));
    }

}