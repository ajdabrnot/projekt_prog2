//implementacije za struct Polje

use crate::logika::ugotovi_skatlo;
use crate::strukture::{Polje, Suduku};

impl Polje {
    pub fn prazno_polje(vrst: u8, stolp: u8) -> Polje {
        //ustvari prazno polje
        Polje {
            vrstica: vrst,
            stolpec: stolp,
            skatla: ugotovi_skatlo(vrst, stolp),
            stevilo: 0,
            moznosti: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        }
    }
    
    pub fn izbrisi_stevilo(&mut self) -> () {
        self.stevilo = 0
        }
    

    pub fn vpisi(&mut self, stevilo: u8) -> () {
        //če je števka veljavna izbira, jo vpiše v polje
        if self.moznosti.contains(&stevilo) {
            self.stevilo = stevilo;
            self.moznosti = vec![]
        };
    }

    pub fn ali_je_vrstica_okej(&self, suduku: &Suduku) -> bool {
        //preveri, da se števka, ki smo jo vpisali v izbrano polje, še ni pojavila v tej vrstici
        let zadovoljive = suduku.manjkajoca_v_vrstici(self.vrstica);
        return zadovoljive.contains(&self.stevilo);
        //let mut resnicnost = true;
        //for polje in suduku.mreza.clone() {
        //    if polje.vrstica == self.vrstica {
        //        if polje.stevilo == self.stevilo {
        //            resnicnost = false;
        //        } else {
        //        }
        //    } else {
        //    }
        //}
        //return self.vrstica > 0 && self.vrstica < 10 && resnicnost;
    }

    pub fn ali_je_skatla_okej(&self, suduku: &Suduku) -> bool {
        //preveri, da se števka, ki smo jo vpisali v izbrano polje, še ni pojavila v tej škatli
        let zadovoljive = suduku.manjkajoca_v_skatli(self.skatla);
        return zadovoljive.contains(&self.stevilo);
        // let mut resnicnost = true;
        // for polje in suduku.mreza.clone() {
        //     if polje.skatla == self.skatla {
        //         if polje.stevilo == self.stevilo {
        //             resnicnost = false;
        //         } else {
        //         }
        //     } else {
        //     }
        // }
        // return self.skatla > 0 && self.skatla < 10 && resnicnost;
    }
    pub fn ali_je_stolpec_okej(&self, suduku: &Suduku) -> bool {
        //preveri, da se števka, ki smo jo vpisali v izbrano polje, še ni pojavila v tem stolpcu
        let zadovoljive = suduku.manjkajoca_v_stolpcu(self.stolpec);
        return zadovoljive.contains(&self.stevilo);
        //let mut resnicnost = true;
        //for polje in suduku.mreza.clone() {
        //    if polje.stolpec == self.stolpec {
        //        if polje.stevilo == self.stevilo {
        //            resnicnost = false;
        //        } else {
        //        }
        //    } else {
        //    }
        //}
        //return self.stolpec > 0 && self.stolpec < 10 && resnicnost;
    }

    pub fn ali_je_veljavno(&self, suduku: &Suduku) -> bool {
        self.ali_je_vrstica_okej(suduku) &&
        self.ali_je_stolpec_okej(suduku) &&
        self.ali_je_skatla_okej(suduku)
    }


    pub fn ugotovi_moznosti(&mut self, suduku: &Suduku) -> () {
        //spremeni možnosti polja
        let vrstica = suduku.manjkajoca_v_vrstici(self.vrstica);
        let stolpec = suduku.manjkajoca_v_stolpcu(self.stolpec);
        let skatla = suduku.manjkajoca_v_skatli(self.skatla);
        for i in vrstica {
            if stolpec.contains(&i) && skatla.contains(&i) {
                self.moznosti.push(i)
            } else {
            }
        }
    }

    pub fn ugotovi_moznosti_in_vrni_nov_sudoku(&mut self, suduku: &Suduku) -> Suduku {
        let nov_sudoku = suduku.kopiraj_sudoku();
        let vrstica = suduku.manjkajoca_v_vrstici(self.vrstica);
        let stolpec = suduku.manjkajoca_v_stolpcu(self.stolpec);
        let skatla = suduku.manjkajoca_v_skatli(self.skatla);
        for i in vrstica {
            if stolpec.contains(&i) && skatla.contains(&i) {
                self.moznosti.push(i)
            } else {
            }
        }
        return nov_sudoku;
    }
}
