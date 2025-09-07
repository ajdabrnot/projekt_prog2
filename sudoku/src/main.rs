pub mod polje;
pub mod pomozne_funkcije;
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
    sudoku.hitro_resi_sudoku();
    let elapsed = now.elapsed();
    println!("resi_hitro: {:.2?}", elapsed);

    let now = Instant::now();
    sudoku.ali_je_sudoku_resljiv_pocasi();
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

    ////testiranje novih funkcij
    let zacetek = Instant::now();
    sudoku.manjkajoca_v_skatli(3);
    let cas_prva = zacetek.elapsed();
    println!("Funkcija manjkajoca v skatli: {:?}", cas_prva);


    let zacetek = Instant::now();
    sudoku.manjkajoca_v_vrstici(3);
    let cas_prva = zacetek.elapsed();
    println!("Funkcija manjkajoca v vrstici: {:?}", cas_prva);
    
    

    let zacetek = Instant::now();
    sudoku.manjkajoca_v_stolpcu(3);
    let cas_prva = zacetek.elapsed();
    println!("Funkcija manjkajoca v stolpcu: {:?}", cas_prva);
    
}

//časi za težek sudoku:
// resi_pocasi: 18.39s
// resi_hitro: 228.20µs
// ali je rešljiv počasi: 21.30µs
// ali je rešljiv hitro: 85.10µs
// enolično rešljiv počasi: 49.80µs
// enolično rešljiv hitro: 60.70µs
// Funkcija manjkajoca v skatli stara: 6.1µs
// Funkcija manjkajoca v skatli nova: 3.4µs
// Nova funkcija je hitrejša za 1.79x
// Funkcija manjkajoca v vrstici stara: 5.5µs
// Funkcija manjkajoca v vrstici nova: 2.7µs
// Nova funkcija je hitrejša za 2.04x
// Funkcija manjkajoca v stolpcu stara: 7.4µs
// Funkcija manjkajoca v stolpcu nova: 3.1µs
// Nova funkcija je hitrejša za 2.39x

//časi za sudoku s 17 namigi:
// resi_pocasi: 15.28s
// resi_hitro: 91.00µs
// ali je rešljiv počasi: 13.30µs
// ali je rešljiv hitro: 32.60µs
// enolično rešljiv počasi: 24.50µs
// enolično rešljiv hitro: 23.70µs
// Funkcija manjkajoca v skatli stara: 6.8µs
// Funkcija manjkajoca v skatli nova: 3.9µs
// Nova funkcija je hitrejša za 1.74x
// Funkcija manjkajoca v vrstici stara: 9.1µs
// Funkcija manjkajoca v vrstici nova: 3.1µs
// Nova funkcija je hitrejša za 2.94x
// Funkcija manjkajoca v stolpcu stara: 7.9µs
// Funkcija manjkajoca v stolpcu nova: 3µs
// Nova funkcija je hitrejša za 2.63x

//časi za (allegedly) ekstremno težek sudoku
//resi_pocasi: 263.03ms
//resi_hitro: 88.10µs
//ali je rešljiv počasi: 13.00µs
//ali je rešljiv hitro: 44.50µs
//enolično rešljiv počasi: 23.20µs
//enolično rešljiv hitro: 37.90µs
// Funkcija manjkajoca v skatli stara: 9.4µs
// Funkcija manjkajoca v skatli nova: 5.7µs
// Nova funkcija je hitrejša za 1.65x
// Funkcija manjkajoca v vrstici stara: 10.8µs
// Funkcija manjkajoca v vrstici nova: 3.3µs
// Nova funkcija je hitrejša za 3.27x
// Funkcija manjkajoca v stolpcu stara: 11.5µs
// Funkcija manjkajoca v stolpcu nova: 3.5µs
// Nova funkcija je hitrejša za 3.29x

//časi za lahek sudoku:
// resi_pocasi: 1.54ms
// resi_hitro: 224.50µs
// ali je rešljiv počasi: 18.50µs
// ali je rešljiv hitro: 31.10µs
// enolično rešljiv počasi: 22.90µs
// enolično rešljiv hitro: 25.40µs
// Funkcija manjkajoca v skatli stara: 7.6µs
// Funkcija manjkajoca v skatli nova: 3.7µs
// Nova funkcija je hitrejša za 2.05x
// Funkcija manjkajoca v vrstici stara: 6.8µs
// Funkcija manjkajoca v vrstici nova: 2.9µs
// Nova funkcija je hitrejša za 2.34x
// Funkcija manjkajoca v stolpcu stara: 6.8µs
// Funkcija manjkajoca v stolpcu nova: 3.3µs
// Nova funkcija je hitrejša za 2.06x
