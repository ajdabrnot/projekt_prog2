//pub mod strukture;
//// pub mod logika;
//use crate::strukture::{Polje, Suduku};

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

// sudoku::strukture::Suduku;

// fn main() {
//     use sudoku::Sudoku;

//     // Sudokus can be created from &str's in both block or line formats or directly from bytes.
//     // here, an example in line format
//     let sudoku_line1 =
//         "...2...633....54.1..1..398........9....538....3........263..5..5.37....847...1...";

//     let sudoku_line =
//         "000200063300005401001003980000000090000538000030000000026300500503700008470001000";

//     let sudoku = Sudoku::from_str_line(sudoku_line).unwrap();

//     // Solve, print or convert the sudoku to another format
//     if let Some(solution) = sudoku.solution() {
//         // print the solution in line format
//         println!("{}", solution);

//         // or return it as a byte array
//         let cell_contents: [u8; 81] = solution.to_bytes();
//     }

// }
