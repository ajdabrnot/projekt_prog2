use crate::strukture::{Polje, Obstoj, Suduku};
use crate::logika;



#[cfg(test)]
mod tests {
    use crate::testi::{Polje, Obstoj, Suduku};

    #[test]
    fn ustvari_polje() {
        let result = Polje::prazno_polje(2, 3);
    let zeljeno = Polje {
        vrstica: 2,
        stolpec: 3,
        stevilo: 0,
        moznosti: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
    };
    assert_eq!(result, zeljeno)
    }

    #[test]
    fn dodaj_stevilo_v_prazen_sudoku() {
        let mut sudoku_prvi = Suduku::prazen_suduku();
        sudoku_prvi.napolni_polje(9, 9, 1);
        sudoku_prvi.napolni_polje(9, 8, 1);
        let seznam_vrednosti = sudoku_prvi.sudoku_kot_seznam_samo_vrednosti();
        let rezultat = [[0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 1]];
        assert_eq!(seznam_vrednosti, rezultat)
    }
   
}



