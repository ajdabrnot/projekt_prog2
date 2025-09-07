//implementacije za struct Polje
use crate::pomozne_funkcije::*;
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
        //izbrise stevko, resetira možnosti
        self.stevilo = 0;
        self.moznosti = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    }

    pub fn vpisi_stevilo(&mut self, stevilo: u8) -> () {
        //če je števka veljavna izbira, jo vpiše v polje
        if self.moznosti.contains(&stevilo) {
            self.stevilo = stevilo;
            self.moznosti = vec![]
        };
    }

    pub fn ali_bi_bila_st_veljavna_v_vrstici(&self, suduku: &Suduku, st: u8) -> bool {
        //preveri, da se števka, ki jo želimo vpisati v izbrano polje, še ni pojavila v tej vrstici
        let zadovoljive = suduku.manjkajoca_v_vrstici(self.vrstica);
        return zadovoljive.contains(&st);
    }

    pub fn ali_bi_bila_st_veljavna_v_skatli(&self, suduku: &Suduku, st: u8) -> bool {
        //preveri, da se števka, ki jo želimo vpisati v izbrano polje, še ni pojavila v tej škatli
        let zadovoljive = suduku.manjkajoca_v_skatli(self.skatla);
        return zadovoljive.contains(&st);
    }

    pub fn ali_bi_bila_st_veljavna_v_stolpcu(&self, suduku: &Suduku, st: u8) -> bool {
        //preveri, da se števka, ki jo želimo vpisati v izbrano polje, še ni pojavila v tem stolpcu
        let zadovoljive = suduku.manjkajoca_v_stolpcu(self.stolpec);
        return zadovoljive.contains(&st);
    }

    pub fn ali_bi_bilo_veljavno(&self, suduku: &Suduku, st: u8) -> bool {
        self.ali_bi_bila_st_veljavna_v_vrstici(suduku, st)
            && self.ali_bi_bila_st_veljavna_v_stolpcu(suduku, st)
            && self.ali_bi_bila_st_veljavna_v_skatli(suduku, st)
    }
    pub fn ugotovi_moznosti(&mut self, suduku: &Suduku) -> () {
        //spremeni možnosti polja
        self.moznosti = vec![];
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
}
