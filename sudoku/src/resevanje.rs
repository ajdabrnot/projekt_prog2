
use super::Suduku;
use crate::strukture::Resevanje;

impl<'a> Resevanje<'a> {
    pub fn nov_za_resevanje(sudoku: &'a mut Suduku) -> Self {
        Resevanje {
            sudoku_za_resevanje: sudoku,
        }
    }

    pub fn resi(&mut self) -> bool {
        self.resi_rekurzivno()
    }

    pub fn resi_rekurzivno(&mut self) -> bool {
        // poisce prazne celice
        if let Some(indeks) = self.sudoku_za_resevanje.prvo_prazno_polje() {
            for st in 1..=9 {
                //self.sudoku_za_resevanje.mreza[indeks].vpisi(st as u8);
                if self.sudoku_za_resevanje.mreza[indeks].ali_bi_bilo_veljavno(self.sudoku_za_resevanje, st)
                {
                    self.sudoku_za_resevanje.mreza[indeks].vpisi(st as u8);
                    if self.resi() {
                        return true;
                    }
                    self.sudoku_za_resevanje.mreza[indeks].izbrisi_stevilo()
                    
                }
                //else {self.sudoku_za_resevanje.mreza[indeks].izbrisi_stevilo()}
            }
            return false;
        }
        true
    }
    
    pub fn resi_se_enkrat(&mut self) -> bool {
        self.resi_rekurzivno_se_enkrat()
    }

    pub fn resi_rekurzivno_se_enkrat(&mut self) -> bool {
        // poisce prazne celice
        if let Some(indeks) = self.sudoku_za_resevanje.prvo_prazno_polje() {
            for st in (1..=9).rev() {
                //self.sudoku_za_resevanje.mreza[indeks].vpisi(st as u8);
                if self.sudoku_za_resevanje.mreza[indeks].ali_bi_bilo_veljavno(self.sudoku_za_resevanje, st)
                {
                    self.sudoku_za_resevanje.mreza[indeks].vpisi(st as u8);
                    if self.resi_se_enkrat() {
                        return true;
                    }
                    self.sudoku_za_resevanje.mreza[indeks].izbrisi_stevilo()
                    
                }
                //else {self.sudoku_za_resevanje.mreza[indeks].izbrisi_stevilo()}
            }
            return false;
        }
        true
    }

}
