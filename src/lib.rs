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

pub enum SystrayEvent{
    MenuItemClick(u32),
    LeftButtonClick,
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

#[derive(Clone,Debug,PartialEq)]
pub enum IconResource {
    File(String),
    Resource(String),
}

#[derive(Clone,PartialEq)]
pub enum IconStatus {
    SHOW,
    HIDE,
}

#[derive(Clone)]
pub struct TrayIcon {
    pub status: IconStatus,
    pub resource: IconResource
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

    pub fn hide_icon(&mut self) -> Result<(), SystrayError> {
        self.window.delete_icon()?;
        if let Some(ref mut icon) = self.icon {
            icon.status = IconStatus::HIDE;
        };
        Ok(())
    }

    pub fn show_icon(&mut self,icon:IconResource) -> Result<(), SystrayError> {
        match self.icon {
            Some(ref mut exist_icon) => {
                if exist_icon.status == IconStatus::HIDE || exist_icon.resource != icon {
                    match icon.clone() {
                        IconResource::File(f) => {
                            self.window.set_icon_from_file(&f.clone())?;
                        }
                        IconResource::Resource(r) => {
                            self.window.set_icon_from_resource(&r.clone())?;
                        }
                    }

                    exist_icon.status = IconStatus::SHOW;
                }
            },
            None => {
                match icon.clone() {
                    IconResource::File(f) => {
                        self.window.set_icon_from_file(&f.clone())?;
                    }
                    IconResource::Resource(r) => {
                        self.window.set_icon_from_resource(&r.clone())?;
                    }
                }
                self.icon = Some(TrayIcon {
                    resource:icon.clone(),
                    status:IconStatus::SHOW
                })
            }
        };
        Ok(())
    }

    pub fn set_tooltip(&self, tooltip: &String) -> Result<(), SystrayError> {
        self.window.set_tooltip(tooltip)
    }

    pub fn quit(&mut self) {
        let _ = self.hide_icon();
        self.window.quit()
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        self.quit();
    }
}
