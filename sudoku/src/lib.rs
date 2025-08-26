mod testi;

use sauron::html::text;
use sauron::prelude::*;
use sauron::{node, Cmd, Component, Node, Program};

pub mod app;
pub mod logika;
pub mod mainm;
pub mod polje;
pub mod resevanje;
pub mod sauron_funkcije;
pub mod strukture;
pub mod suduku;
use crate::logika::pojavitve_stevila;
use crate::strukture::{App, Msg, Polje, Suduku};

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
}

//Če ne dela 0.0.0.0 spremeni v localhost

//zakaj dovoli dve isti števili v enem stolpcu???
//x-wing preveri
//škatle v debele obrobe css

// wasm-pack build --target web --release
// basic-http-server -a 0.0.0.0:4000
// puscica gor dol za zgodovino v teminalu
