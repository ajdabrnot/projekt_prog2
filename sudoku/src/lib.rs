mod testi;

use sauron::prelude::*;
use sauron::Program;

pub mod app;
pub mod logika;
pub mod mainm;
pub mod polje;
pub mod resevanje;
pub mod sauron_funkcije;
pub mod strukture;
pub mod suduku;
pub mod se_neuporabljene_funkcije;
use crate::strukture::{App, Suduku};

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
}

//Če ne dela 0.0.0.0 spremeni v localhost

// wasm-pack build --target web --release
// basic-http-server -a 0.0.0.0:4000
// puscica gor dol za zgodovino v teminalu
