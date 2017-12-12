extern crate systray;
use std::{thread, time};
use systray::SystrayEvent;
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
        tray.set_icon_from_file("./resources/rust.ico".to_string()).ok();
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
                    let _ = tray.show_icon().map_err(|e|{
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

fn main() {
    let tray = TxTray::new();
    loop {
        thread::sleep(time::Duration::from_secs(13));
        let _ = tray.show();
        thread::sleep(time::Duration::from_secs(13));
        let _ = tray.hide();
    }
}
#[derive(Debug)]
struct TxTray {
    control_sender:Sender<ControlEvent>
}
impl TxTray {
    pub fn new() -> TxTray {
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
                        println!("txtray get a err of tray {:?}",e);
                    }
                }
            }
        });
        TxTray{control_sender:control_sender}
    }

    pub fn show(&self){
        self.control_sender.send(ControlEvent::ShowIcon);
    }
    pub fn hide(&self){
        self.control_sender.send(ControlEvent::HideIcon);
    }
}