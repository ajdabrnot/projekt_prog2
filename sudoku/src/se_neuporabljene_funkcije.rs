use crate::logika::ugotovi_skatlo;
use crate::strukture::{Polje, Suduku};

impl Polje {
    pub fn ali_je_vrstica_okej(&self, suduku: &Suduku) -> bool {
       //preveri, da se števka, ki smo jo vpisali v izbrano polje, še ni pojavila v tej vrstici
       let zadovoljive = suduku.manjkajoca_v_vrstici(self.vrstica);
       return zadovoljive.contains(&self.stevilo);
    }
    pub fn ali_je_skatla_okej(&self, suduku: &Suduku) -> bool {
       //preveri, da se števka, ki smo jo vpisali v izbrano polje, še ni pojavila v tej škatli
       let zadovoljive = suduku.manjkajoca_v_skatli(self.skatla);
       return zadovoljive.contains(&self.stevilo);
    }
     pub fn ali_je_stolpec_okej(&self, suduku: &Suduku) -> bool {
       //preveri, da se števka, ki smo jo vpisali v izbrano polje, še ni pojavila v tem stolpcu
       let zadovoljive = suduku.manjkajoca_v_stolpcu(self.stolpec);
       return zadovoljive.contains(&self.stevilo);
    }
    pub fn ali_je_veljavno(&self, suduku: &Suduku) -> bool {
       self.ali_je_vrstica_okej(suduku)
           && self.ali_je_stolpec_okej(suduku)
           && self.ali_je_skatla_okej(suduku)
    }
    
    //pub fn ugotovi_moznosti_in_vrni_nov_sudoku(&mut self, suduku: &Suduku) -> Suduku {
    //    let nov_sudoku = suduku.kopiraj_sudoku();
    //    let vrstica = suduku.manjkajoca_v_vrstici(self.vrstica);
    //    let stolpec = suduku.manjkajoca_v_stolpcu(self.stolpec);
    //    let skatla = suduku.manjkajoca_v_skatli(self.skatla);
    //    for i in vrstica {
    //        if stolpec.contains(&i) && skatla.contains(&i) {
    //            self.moznosti.push(i)
    //        } else {
    //        }
    //    }
    //    return nov_sudoku;
    //}
}

impl Suduku {

    fn vpisi_enolicno_dolocena_stevila(&mut self) -> () {
        //zapolni polja katerih rešitev je že enolično določena
        for mut polje in self.mreza.clone() {
            if polje.moznosti.len() == 1 {
                polje.vpisi_stevilo(polje.moznosti[0]);
            }
        }
    }


    //pub fn prazna_polja(&self) -> Vec<usize> {
    //    //vrne indekse praznih polj
    //    let mut prazna = vec![];
    //    for i in 0..81 {
    //        if self.mreza[i].stevilo == 0 {
    //            prazna.push(i);
    //        }
    //    }
    //    return prazna;
    //}

    //    //vrne indekse praznih polj
    //    let mut prazna = vec![];
    //    for i in 0..81 {
    //        if self.mreza[i].stevilo == 0 {
    //            prazna.push(i);
    //        }
    //    }
    //    return prazna;
    //}
}