
#[cfg(test)]
mod tests {
    use crate::{
        logika::ugotovi_skatlo,
        sauron_funkcije::ustvari_id,
        strukture::{Polje, Suduku},
    };

    #[test]
    fn id() {
        let result = ustvari_id(2, 3);
        let iskano = "r2c3";
        assert_eq!(result, iskano)
    }

    #[test]
    fn ustvari_polje() {
        let result = Polje::prazno_polje(2, 3);
        let zeljeno = Polje {
            vrstica: 2,
            stolpec: 3,
            skatla: ugotovi_skatlo(2, 3),
            stevilo: 0,
            moznosti: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        };
        assert_eq!(result, zeljeno)
    }

    #[test]
    fn vpis_stevila() {
        let mut result = Polje::prazno_polje(3, 4);
        result.vpisi(9);
        let iskano = Polje {
            vrstica: 3,
            stolpec: 4,
            skatla: 2,
            stevilo: 9,
            moznosti: vec![],
        };
        assert_eq!(result, iskano)
    }

    #[test]
    fn izbris_stevila() {
        //ta funkcija je nekoliko vprašljiva?
        let mut result = Polje::prazno_polje(3, 4);
        result.vpisi(9);
        result.izbrisi_stevilo();
        let iskano = Polje {
            vrstica: 3,
            stolpec: 4,
            skatla: 2,
            stevilo: 0,
            moznosti: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        };
        assert_eq!(iskano, result)
    }

    #[test]
    fn ustvari_prazen_sudoku() {
        let result = Suduku::prazen_suduku();
        let tabela = vec![
            Polje::prazno_polje(1, 1),
            Polje::prazno_polje(1, 2),
            Polje::prazno_polje(1, 3),
            Polje::prazno_polje(1, 4),
            Polje::prazno_polje(1, 5),
            Polje::prazno_polje(1, 6),
            Polje::prazno_polje(1, 7),
            Polje::prazno_polje(1, 8),
            Polje::prazno_polje(1, 9),
            Polje::prazno_polje(2, 1),
            Polje::prazno_polje(2, 2),
            Polje::prazno_polje(2, 3),
            Polje::prazno_polje(2, 4),
            Polje::prazno_polje(2, 5),
            Polje::prazno_polje(2, 6),
            Polje::prazno_polje(2, 7),
            Polje::prazno_polje(2, 8),
            Polje::prazno_polje(2, 9),
            Polje::prazno_polje(3, 1),
            Polje::prazno_polje(3, 2),
            Polje::prazno_polje(3, 3),
            Polje::prazno_polje(3, 4),
            Polje::prazno_polje(3, 5),
            Polje::prazno_polje(3, 6),
            Polje::prazno_polje(3, 7),
            Polje::prazno_polje(3, 8),
            Polje::prazno_polje(3, 9),
            Polje::prazno_polje(4, 1),
            Polje::prazno_polje(4, 2),
            Polje::prazno_polje(4, 3),
            Polje::prazno_polje(4, 4),
            Polje::prazno_polje(4, 5),
            Polje::prazno_polje(4, 6),
            Polje::prazno_polje(4, 7),
            Polje::prazno_polje(4, 8),
            Polje::prazno_polje(4, 9),
            Polje::prazno_polje(5, 1),
            Polje::prazno_polje(5, 2),
            Polje::prazno_polje(5, 3),
            Polje::prazno_polje(5, 4),
            Polje::prazno_polje(5, 5),
            Polje::prazno_polje(5, 6),
            Polje::prazno_polje(5, 7),
            Polje::prazno_polje(5, 8),
            Polje::prazno_polje(5, 9),
            Polje::prazno_polje(6, 1),
            Polje::prazno_polje(6, 2),
            Polje::prazno_polje(6, 3),
            Polje::prazno_polje(6, 4),
            Polje::prazno_polje(6, 5),
            Polje::prazno_polje(6, 6),
            Polje::prazno_polje(6, 7),
            Polje::prazno_polje(6, 8),
            Polje::prazno_polje(6, 9),
            Polje::prazno_polje(7, 1),
            Polje::prazno_polje(7, 2),
            Polje::prazno_polje(7, 3),
            Polje::prazno_polje(7, 4),
            Polje::prazno_polje(7, 5),
            Polje::prazno_polje(7, 6),
            Polje::prazno_polje(7, 7),
            Polje::prazno_polje(7, 8),
            Polje::prazno_polje(7, 9),
            Polje::prazno_polje(8, 1),
            Polje::prazno_polje(8, 2),
            Polje::prazno_polje(8, 3),
            Polje::prazno_polje(8, 4),
            Polje::prazno_polje(8, 5),
            Polje::prazno_polje(8, 6),
            Polje::prazno_polje(8, 7),
            Polje::prazno_polje(8, 8),
            Polje::prazno_polje(8, 9),
            Polje::prazno_polje(9, 1),
            Polje::prazno_polje(9, 2),
            Polje::prazno_polje(9, 3),
            Polje::prazno_polje(9, 4),
            Polje::prazno_polje(9, 5),
            Polje::prazno_polje(9, 6),
            Polje::prazno_polje(9, 7),
            Polje::prazno_polje(9, 8),
            Polje::prazno_polje(9, 9),
        ];
        let iskano = Suduku { mreza: tabela };
        assert_eq!(iskano, result)
    }

    #[test]
    fn napolnevanje_polja() {
        let mut result = Suduku::prazen_suduku();
        result.napolni_polje(1, 1, 1);
        let mut polje = Polje::prazno_polje(1, 1);
        polje.vpisi(1);
        let tabela = vec![
            polje,
            Polje::prazno_polje(1, 2),
            Polje::prazno_polje(1, 3),
            Polje::prazno_polje(1, 4),
            Polje::prazno_polje(1, 5),
            Polje::prazno_polje(1, 6),
            Polje::prazno_polje(1, 7),
            Polje::prazno_polje(1, 8),
            Polje::prazno_polje(1, 9),
            Polje::prazno_polje(2, 1),
            Polje::prazno_polje(2, 2),
            Polje::prazno_polje(2, 3),
            Polje::prazno_polje(2, 4),
            Polje::prazno_polje(2, 5),
            Polje::prazno_polje(2, 6),
            Polje::prazno_polje(2, 7),
            Polje::prazno_polje(2, 8),
            Polje::prazno_polje(2, 9),
            Polje::prazno_polje(3, 1),
            Polje::prazno_polje(3, 2),
            Polje::prazno_polje(3, 3),
            Polje::prazno_polje(3, 4),
            Polje::prazno_polje(3, 5),
            Polje::prazno_polje(3, 6),
            Polje::prazno_polje(3, 7),
            Polje::prazno_polje(3, 8),
            Polje::prazno_polje(3, 9),
            Polje::prazno_polje(4, 1),
            Polje::prazno_polje(4, 2),
            Polje::prazno_polje(4, 3),
            Polje::prazno_polje(4, 4),
            Polje::prazno_polje(4, 5),
            Polje::prazno_polje(4, 6),
            Polje::prazno_polje(4, 7),
            Polje::prazno_polje(4, 8),
            Polje::prazno_polje(4, 9),
            Polje::prazno_polje(5, 1),
            Polje::prazno_polje(5, 2),
            Polje::prazno_polje(5, 3),
            Polje::prazno_polje(5, 4),
            Polje::prazno_polje(5, 5),
            Polje::prazno_polje(5, 6),
            Polje::prazno_polje(5, 7),
            Polje::prazno_polje(5, 8),
            Polje::prazno_polje(5, 9),
            Polje::prazno_polje(6, 1),
            Polje::prazno_polje(6, 2),
            Polje::prazno_polje(6, 3),
            Polje::prazno_polje(6, 4),
            Polje::prazno_polje(6, 5),
            Polje::prazno_polje(6, 6),
            Polje::prazno_polje(6, 7),
            Polje::prazno_polje(6, 8),
            Polje::prazno_polje(6, 9),
            Polje::prazno_polje(7, 1),
            Polje::prazno_polje(7, 2),
            Polje::prazno_polje(7, 3),
            Polje::prazno_polje(7, 4),
            Polje::prazno_polje(7, 5),
            Polje::prazno_polje(7, 6),
            Polje::prazno_polje(7, 7),
            Polje::prazno_polje(7, 8),
            Polje::prazno_polje(7, 9),
            Polje::prazno_polje(8, 1),
            Polje::prazno_polje(8, 2),
            Polje::prazno_polje(8, 3),
            Polje::prazno_polje(8, 4),
            Polje::prazno_polje(8, 5),
            Polje::prazno_polje(8, 6),
            Polje::prazno_polje(8, 7),
            Polje::prazno_polje(8, 8),
            Polje::prazno_polje(8, 9),
            Polje::prazno_polje(9, 1),
            Polje::prazno_polje(9, 2),
            Polje::prazno_polje(9, 3),
            Polje::prazno_polje(9, 4),
            Polje::prazno_polje(9, 5),
            Polje::prazno_polje(9, 6),
            Polje::prazno_polje(9, 7),
            Polje::prazno_polje(9, 8),
            Polje::prazno_polje(9, 9),
        ];
        let iskano = Suduku { mreza: tabela };
        assert_eq!(iskano, result)
    }

    #[test]
    fn sez_praznih_polj() {
        let mut sudoku = Suduku::prazen_suduku();
        sudoku.napolni_polja(vec![(1, 1, 1), (1, 2, 3), (1, 3, 4)]);
        let result = sudoku.prazna_polja();
        let mut iskano = vec![];
        for i in 3..81 {
            iskano.push(i)
        }
        assert_eq!(iskano, result)
    }

    #[test]
    fn prvo_prazno_polje_sudokuja() {
        let mut sudoku = Suduku::prazen_suduku();
        sudoku.napolni_polja(vec![(1, 1, 1), (1, 2, 3), (1, 3, 4)]);
        let result = sudoku.prvo_prazno_polje();
        let iskano = Some(3);
        assert_eq!(iskano, result)
    }

    #[test]
    fn manjkajoca_stevila() {
        let mut sudoku = Suduku::prazen_suduku();
        sudoku.napolni_polja(vec![
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
        let result_skatla = sudoku.manjkajoca_v_skatli(8);
        let iskano_skatla = vec![3, 7];
        let result_stolpec = sudoku.manjkajoca_v_stolpcu(6);
        let iskano_stolpec = vec![8];
        let result_vrstica = sudoku.manjkajoca_v_vrstici(2);
        let iskano_vrstica = vec![1, 2, 5, 7];
        let result = (result_skatla, result_stolpec, result_vrstica);
        let iskano = (iskano_skatla, iskano_stolpec, iskano_vrstica);
        assert_eq!(result, iskano)
    }

    #[test]
    fn ali_je_vrsticaskatlapolje_okej() {
        //preverjeno za true in false variante
        let mut polje = Polje::prazno_polje(1, 9);
        polje.vpisi(4);
        let mut sudoku = Suduku::prazen_suduku();
        sudoku.napolni_polja(vec![
            (1, 1, 5),
            (1, 2, 6),
            (1, 6, 7),
            (2, 9, 4),
            (3, 8, 8),
            (3, 9, 7),
            (4, 1, 6),
            (4, 6, 2),
            (4, 8, 7),
            (4, 9, 9),
            (5, 9, 2),
            (8, 9, 1),
        ]);
        let result_vrstica = polje.ali_je_vrstica_okej(&sudoku);
        let result_stolpec = polje.ali_je_stolpec_okej(&sudoku);
        let result_skatla = polje.ali_je_skatla_okej(&sudoku);
        let iskano_vrstica = true;
        let iskano_stolpec = false;
        let iskano_skatla = false;
        let result = (result_skatla, result_stolpec, result_vrstica);
        let iskano = (iskano_skatla, iskano_stolpec, iskano_vrstica);
        assert_eq!(result, iskano)
    }

    #[test]
    fn veljavnost_polja() {
        let mut polje = Polje::prazno_polje(1, 9);
        polje.vpisi(4);
        let mut sudoku = Suduku::prazen_suduku();
        sudoku.napolni_polja(vec![
            (1, 1, 5),
            (1, 2, 6),
            (1, 6, 7),
            (2, 9, 4),
            (3, 8, 8),
            (3, 9, 7),
            (4, 1, 6),
            (4, 6, 2),
            (4, 8, 7),
            (4, 9, 9),
            (5, 9, 2),
            (8, 9, 1),
        ]);
        let result = polje.ali_je_veljavno(&sudoku);
        let iskano = false;
        assert_eq!(iskano, result)
    }

    #[test]
    fn ugotavljanje_moznosti_polja() {
        let mut polje = Polje::prazno_polje(1, 9);
        let mut sudoku = Suduku::prazen_suduku();
        sudoku.napolni_polja(vec![
            (1, 1, 5),
            (1, 2, 6),
            (1, 6, 7),
            (2, 9, 4),
            //(3, 8, 8),
            (3, 9, 7),
            (4, 1, 6),
            (4, 6, 2),
            (4, 8, 7),
            (4, 9, 9),
            (5, 9, 2),
            (8, 9, 1),
        ]);
        polje.ugotovi_moznosti(&sudoku);
        let result = polje.moznosti;
        let iskano = vec![3, 8];
        assert_eq!(result, iskano)
    }

    #[test]
    fn veljavna_napolnitev_polja_false() {
        let mut sudoku = Suduku::prazen_suduku();
        sudoku.napolni_polja(vec![(1, 1, 5), (1, 2, 6), (1, 6, 7)]);
        let result = sudoku.ali_je_veljavno(1, 3, 7);
        let iskano = false;
        assert_eq!(iskano, result)
    }
    #[test]
    fn veljavna_napolnitev_polja_true() {
        let mut sudoku = Suduku::prazen_suduku();
        sudoku.napolni_polja(vec![(1, 1, 5), (1, 2, 6), (1, 6, 7)]);
        let result = sudoku.ali_je_veljavno(1, 3, 2);
        let iskano = true;
        assert_eq!(iskano, result)
    }

    #[test]
    fn dodaj_stevilo_v_prazen_sudoku() {
        let mut sudoku_prvi = Suduku::prazen_suduku();
        sudoku_prvi.napolni_polje(9, 9, 1);
        sudoku_prvi.napolni_polje(9, 8, 1);
        let seznam_vrednosti = sudoku_prvi.sudoku_kot_seznam_samo_vrednosti();
        let rezultat = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 1],
        ];
        assert_eq!(seznam_vrednosti, rezultat)
    }
}
