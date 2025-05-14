pub mod strukture;
pub mod logika;
use crate::strukture::{Polje, Suduku};

// pub fn main() {
//     console_log::init_with_level(log::Level::Trace).unwrap();
//     console_error_panic_hook::set_once();
//     Program::mount_to_body(App::new());
//     let mut sudoku_prvi = Suduku::prazen_suduku();
//     println!("{:?}", sudoku_prvi.mreza);
//     sudoku_prvi.napolni_polje(9, 9, 1);
//     println!("TO BO NOVA MREZAAAAAA");
//     println!("{:?}", sudoku_prvi.mreza);
//     sudoku_prvi.napolni_polje(9, 8, 1);
//     println!("TO BO NOVA MREZAAAAAA");
//     println!("{:?}", sudoku_prvi.mreza);
//     let seznam_vrednosti = sudoku_prvi.sudoku_kot_seznam_samo_vrednosti();
//     println!("{:?}", seznam_vrednosti) }
