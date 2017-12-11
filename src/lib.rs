// Systray Lib

#[macro_use]
extern crate log;
#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(target_os = "windows")]
extern crate kernel32;
#[cfg(target_os = "windows")]
extern crate user32;
#[cfg(target_os = "windows")]
extern crate libc;
#[cfg(target_os = "linux")]
extern crate gtk;
#[cfg(target_os = "linux")]
extern crate glib;
#[cfg(target_os = "linux")]
extern crate libappindicator;

pub mod api;

use std::sync::mpsc::Sender;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum SystrayError {
    OsError(String),
    NotImplementedError,
    UnknownError,
    ShowIconWithoutSetError,
}

pub enum SystrayEvent {
    MenuItemClick(u32),
    LeftButtonClick,
    ShowIcon,
    HideIcon
}

impl std::fmt::Display for SystrayError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            &SystrayError::OsError(ref err_str) => write!(f, "OsError: {}", err_str),
            &SystrayError::NotImplementedError => write!(f, "Functionality is not implemented yet"),
            &SystrayError::UnknownError => write!(f, "Unknown error occurrred"),
            &SystrayError::ShowIconWithoutSetError => write!(f, "want show icon but icon is none"),

        }
    }
}
#[derive(Debug)]
pub enum TrayIcon {
    File(String),
    Resource(String),
}

pub struct Application {
    window: api::api::Window,
    icon: Option<TrayIcon>,
    menu_idx: u32,
    callback: HashMap<u32, Callback>,
}

type Callback = Box<(Fn(&mut Application) -> () + 'static)>;

fn make_callback<F>(f: F) -> Callback
    where F: std::ops::Fn(&mut Application) -> () + 'static {
    Box::new(f) as Callback
}

impl Application {
    pub fn new(event_tx:Sender<SystrayEvent>) -> Result<Application, SystrayError> {
        match api::api::Window::new(event_tx) {
            Ok(w) => Ok(Application {
                window: w,
                icon: None,
                menu_idx: 0,
                callback: HashMap::new(),
            }),
            Err(e) => Err(e)
        }
    }

    pub fn add_menu_item<F>(&mut self, item_name: &String, f: F) -> Result<u32, SystrayError>
        where F: std::ops::Fn(&mut Application) -> () + 'static {
        let idx = self.menu_idx;
        if let Err(e) = self.window.add_menu_entry(idx, item_name) {
            return Err(e);
        }
        self.callback.insert(idx, make_callback(f));
        self.menu_idx += 1;
        Ok(idx)
    }

    pub fn add_menu_separator(&mut self) -> Result<u32, SystrayError> {
        let idx = self.menu_idx;
        if let Err(e) = self.window.add_menu_separator(idx) {
            return Err(e);
        }
        self.menu_idx += 1;
        Ok(idx)
    }

    pub fn hide_icon(&self) -> Result<(), SystrayError> {
        self.window.delete_icon()
    }

    pub fn set_icon(&mut self, icon: TrayIcon) -> Result<(), SystrayError> {
        self.icon = Some(icon);
        Ok(())
    }

    pub fn show_icon(&self) -> Result<(), SystrayError> {
        match self.icon {
            Some(TrayIcon::File(ref icon)) => self.window.set_icon_from_file(&icon),
            Some(TrayIcon::Resource(ref icon)) => self.window.set_icon_from_resource(&icon),
            None => {
                return Err(SystrayError::ShowIconWithoutSetError)
            }
        }
    }

    pub fn set_icon_from_file(&mut self, file: String) -> Result<(), SystrayError> {
        self.icon = Some(TrayIcon::File(file));
        Ok(())
    }

    pub fn set_icon_from_resource(&mut self, resource: String) -> Result<(), SystrayError> {
        self.icon =Some(TrayIcon::Resource(resource));
        Ok(())
    }

    pub fn shutdown(&self) -> Result<(), SystrayError> {
        self.window.delete_icon()
    }

    pub fn set_tooltip(&self, tooltip: &String) -> Result<(), SystrayError> {
        self.window.set_tooltip(tooltip)
    }

    pub fn quit(&mut self) {
        self.window.quit()
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        self.shutdown().ok();
    }
}
