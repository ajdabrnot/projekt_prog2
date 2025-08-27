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
//
//fn main() {
//    use sudoku::Sudoku;
//
//    // Sudokus can be created from &str's in both block or line formats or directly from bytes.
//    // here, an example in line format
//    let sudoku_line1 =
//        "...2...633....54.1..1..398........9....538....3........263..5..5.37....847...1...";
//
//    let sudoku_line =
//        "000200063300005401001003980000000090000538000030000000026300500503700008470001000";
//
//    let sudoku = Sudoku::from_str_line(sudoku_line).unwrap();
//
//    // Solve, print or convert the sudoku to another format
//    if let Some(solution) = sudoku.solution() {
//        // print the solution in line format
//        println!("{}", solution);
//
//        // or return it as a byte array
//        let cell_contents: [u8; 81] = solution.to_bytes();
//    }
//
//    //let mut sudoku = Suduku::prazen_suduku();
//    //sudoku.napolni_polja(vec![(1, 1, 1), (1, 2, 3), (1, 3, 4)]);
//    //let result = sudoku.sudoku_kot_niz_vrednosti();
//    //println!("{}", result)
//}
pub mod logika;
pub mod polje;
pub mod resevanje;
pub mod strukture;
pub mod suduku;
use crate::strukture::*;
fn main() {
    use std::time::Instant;

    let mut tezek_sudoku = Suduku::prazen_suduku();
    tezek_sudoku.napolni_polja(vec![
        (1, 4, 6),
        (1, 8, 3),
        (1, 9, 1),
        (2, 3, 5),
        (3, 2, 3),
        (3, 4, 5),
        (3, 5, 4),
        (3, 8, 2),
        (4, 7, 4),
        (5, 5, 9),
        (5, 9, 8),
        (6, 1, 1),
        (6, 2, 6),
        (6, 5, 8),
        (6, 7, 7),
        (7, 6, 5),
        (7, 7, 9),
        (7, 8, 6),
        (8, 2, 1),
        (9, 1, 4),
        (9, 4, 7),
        (9, 5, 6),
    ]);
    let mut lahek_sudoku = Suduku::prazen_suduku();
    lahek_sudoku.napolni_polja(vec![
        (1, 1, 5),
        (1, 2, 6),
        (1, 6, 7),
        (2, 2, 9),
        (2, 4, 6),
        (2, 5, 8),
        (2, 6, 3),
        (2, 9, 4),
        (3, 2, 4),
        (3, 5, 1),
        (3, 6, 5),
        (3, 8, 8),
        (3, 9, 7),
        (4, 1, 6),
        (4, 6, 2),
        (4, 8, 7),
        (4, 9, 9),
        (5, 2, 3),
        (5, 7, 1),
        (5, 9, 2),
        (6, 4, 7),
        (6, 5, 6),
        (6, 6, 4),
        (6, 8, 3),
        (7, 1, 9),
        (7, 2, 2),
        (7, 4, 8),
        (7, 6, 1),
        (8, 1, 4),
        (8, 3, 3),
        (8, 4, 2),
        (8, 5, 5),
        (8, 6, 6),
        (8, 9, 1),
        (9, 1, 8),
        (9, 5, 4),
        (9, 6, 9),
        (9, 8, 2),
    ]);

    let mut sudoku_17_namigov = Suduku::prazen_suduku();
    sudoku_17_namigov.napolni_polja(vec![
        (1, 2, 1),
        (1, 4, 6),
        (2, 1, 4),
        (2, 7, 7),
        (3, 4, 2),
        (4, 1, 7),
        (4, 5, 4),
        (5, 8, 5),
        (5, 9, 6),
        (6, 8, 2),
        (7, 1, 6),
        (7, 2, 2),
        (7, 3, 8),
        (7, 7, 3),
        (8, 4, 5),
        (8, 6, 1),
        (9, 1, 2),
    ]);

    let mut sudoku_extreme = Suduku::prazen_suduku();
    sudoku_extreme.napolni_polja(vec![
        (1, 3, 2),
        (1, 9, 5),
        (2, 1, 1),
        (2, 5, 2),
        (2, 6, 3),
        (2, 8, 8),
        (3, 1, 9),
        (3, 4, 7),
        (3, 9, 1),
        (4, 5, 3),
        (4, 8, 5),
        (4, 9, 2),
        (5, 3, 6),
        (5, 9, 8),
        (6, 2, 9),
        (6, 4, 1),
        (6, 7, 6),
        (7, 1, 7),
        (7, 3, 8),
        (7, 5, 6),
        (7, 6, 9),
        (7, 7, 5),
        (8, 8, 3),
        (9, 2, 4),
        (9, 5, 5),
        (9, 6, 7),
    ]);

    //let mut sudoku = lahek_sudoku;
    //let mut sudoku = tezek_sudoku;
    //let mut sudoku = sudoku_17_namigov;
    let mut sudoku = sudoku_extreme;
    let now = Instant::now();
    sudoku.resi_sudoku_pocasi();
    let elapsed = now.elapsed();
    println!("resi_pocasi: {:.2?}", elapsed);

    let now = Instant::now();
    sudoku.hitro_resi_sudoku_1();
    let elapsed = now.elapsed();
    println!("resi_hitro: {:.2?}", elapsed);

    let now = Instant::now();
    sudoku.ali_je_sudoku_resljiv();
    let elapsed = now.elapsed();
    println!("ali je rešljiv počasi: {:.2?}", elapsed);

    let now = Instant::now();
    sudoku.ali_je_resljiv_hitro();
    let elapsed = now.elapsed();
    println!("ali je rešljiv hitro: {:.2?}", elapsed);

    let now = Instant::now();
    sudoku.je_enolicno_resljivo_pocasi();
    let elapsed = now.elapsed();
    println!("enolično rešljiv počasi: {:.2?}", elapsed);

    let now = Instant::now();
    sudoku.je_enolicno_resljivo_hitra();
    let elapsed = now.elapsed();
    println!("enolično rešljiv hitro: {:.2?}", elapsed);
}

//časi za težek sudoku:
// resi_pocasi: 18.39s
// resi_hitro: 228.20µs
// ali je rešljiv počasi: 21.30µs
// ali je rešljiv hitro: 85.10µs
// enolično rešljiv počasi: 49.80µs
// enolično rešljiv hitro: 60.70µs

//časi za sudoku s 17 namigi:
// resi_pocasi: 15.28s
// resi_hitro: 91.00µs
// ali je rešljiv počasi: 13.30µs
// ali je rešljiv hitro: 32.60µs
// enolično rešljiv počasi: 24.50µs
// enolično rešljiv hitro: 23.70µs

//časi za (allegedly) ekstremno težek sudoku
//resi_pocasi: 263.03ms
//resi_hitro: 88.10µs
//ali je rešljiv počasi: 13.00µs
//ali je rešljiv hitro: 44.50µs
//enolično rešljiv počasi: 23.20µs
//enolično rešljiv hitro: 37.90µs

//časi za lahek sudoku:
// resi_pocasi: 1.54ms
// resi_hitro: 224.50µs
// ali je rešljiv počasi: 18.50µs
// ali je rešljiv hitro: 31.10µs
// enolično rešljiv počasi: 22.90µs
// enolično rešljiv hitro: 25.40µs
