use super::Suduku;
use crate::strukture::Resevanje;

impl<'a> Resevanje<'a> {
    pub fn nov_za_resevanje(sudoku: &'a mut Suduku) -> Self {
        Resevanje {
            sudoku_za_resevanje: sudoku,
        }
    }

    pub fn resi(&mut self, smer: bool) -> bool {
        self.resi_rekurzivno(smer)
    }

    pub fn resi_rekurzivno(&mut self, smer: bool) -> bool {
        // poisce prazne celice
        if let Some(indeks) = self.sudoku_za_resevanje.prvo_prazno_polje() {
            let mut st = 0;
            for i in 1..=9 {
                if smer {st = i} else {st = 10 - i};
                if self.sudoku_za_resevanje.mreza[indeks]
                    .ali_bi_bilo_veljavno(self.sudoku_za_resevanje, st)
                {
                    self.sudoku_za_resevanje.mreza[indeks].vpisi_stevilo(st as u8);
                    if self.resi(smer) {
                        return true;
                    }
                    self.sudoku_za_resevanje.mreza[indeks].izbrisi_stevilo()
                }
            }
            return false;
        }
        true
    }

}
