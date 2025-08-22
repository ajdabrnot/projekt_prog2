pub mod app;
pub mod strukture;
use crate::strukture::App;
use sauron::prelude::*;
use sauron::Program;

// ctrl + k + c ti zakomentira vse kar oznacis
// ctr + k ctrl u odkomentira
// ctrl k s bliznice
// ctrl c prekine terminal

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
