use crate::strukture::{Polje, Suduku};

impl Suduku {
    pub fn prazen_suduku() -> Suduku {
        let mut tabela = vec![];
        for i in 1..=9 {
            for j in 1..10 {
                let polje = Polje::prazno_polje(i, j);
                tabela.push(polje);
            }
        }
        Suduku {
            mreza: tabela,
            trenutno_polje: 0,
        }
    }

    pub fn napolni_polje(&mut self, vrst: u8, stolp: u8, st: u8) -> () {
        let indeks = (vrst as usize - 1) * 9 + (stolp as usize) - 1;
        let polje = Polje {
            vrstica: vrst,
            stolpec: stolp,
            stevilo: st,
            moznosti: vec![],
        };
        let v_vrstici = polje.ali_je_vrstica_okej(self);
        let v_stolpcu = polje.ali_je_stolpec_okej(self);
        let v_skatli = polje.ali_je_skatla_okej(self);
        if v_vrstici && v_stolpcu && v_skatli {
            self.mreza[indeks] = polje
        } else {
        }
        // for polje in &self.mreza {
        //     polje.ugotovi_moznosti(self)
        // }
    }

    // pub fn spremeni_moznosti(&mut self) -> () {
    //     for i in 0..9 {
    //         for j in 0..9 {

    //             self.mreza[9 *  i + j].ugotovi_moznosti(self);
    //         }
    //     }
    // }
    pub fn kopiraj_sudoku(&self) -> Suduku {
        Suduku {
            mreza: self.mreza.clone(),
            trenutno_polje: 0,
        }
    }

    pub fn ali_je_veljavno(&self, vrst: u8, stolp: u8, st: u8) -> bool {
        let polje = Polje {
            vrstica: vrst,
            stolpec: stolp,
            stevilo: st,
            moznosti: vec![],
        };
        return polje.ali_je_vrstica_okej(self)
            && polje.ali_je_stolpec_okej(self)
            && polje.ali_je_skatla_okej(self);
    }

    fn vpisi_enolicno_dolocena_stevila(&mut self) -> () {
        for mut polje in self.mreza.clone() {
            if polje.moznosti.len() == 1 {
                polje.vpisi(polje.moznosti[0]);
            }
        }
    }

    pub fn sudoku_kot_seznam_samo_vrednosti(&self) -> Vec<Vec<u8>> {
        let mut nov = vec![];
        let mut manjsi_sez = vec![];
        for i in 0..81 {
            if i % 9 == 8 {
                manjsi_sez.push(self.mreza[i].stevilo);
                nov.push(manjsi_sez);
                manjsi_sez = vec![];
            } else {
                manjsi_sez.push(self.mreza[i].stevilo)
            }
        }
        return nov;
    }

    // pub fn ugotovi_moznosti_celega_sudokuja_enkrat(&mut self) -> () {
    //     //gre enkrat cez sudoku
    //     for mut polje in &self.mreza {
    //         let vrstica = ugotovi_stevila_v_vrstici(polje.vrstica, &self);
    //         let stolpec = ugotovi_stevila_v_stolpcu(polje.stolpec, &self);
    //         let skatla = ugotovi_stevila_v_skatli(polje.ugotovi_skatlo(), &self);

    //     for i in vrstica {
    //         if stolpec.contains(&i) && skatla.contains(&i) {
    //             polje.moznosti.push(i)
    //         } else {
    //         }
    //     }}

    // }
    // ne dela :(

    pub fn resi_sudoku(&mut self) -> () {
        //zelo očitno zadeva ne reši zares sudokuja, to je le začasno, da lahko usposobim gumb reši
        //to je zdej isto kot vpisi enolicno dolocena st

        //self.ugotovi_moznosti_celega_sudokuja_enkrat();
        for polje in &mut self.mreza {
            polje.ugotovi_moznosti(self);
            if polje.moznosti.len() == 1 && polje.stevilo == 0 {
                polje.vpisi(polje.moznosti[0]);
            }
        }

        // if self.je_enolicno_resljivo() {
        //     self.vpisi_enolicno_dolocena_stevila();
        // }}
        // else {for polje in &mut self.mreza {
        //     polje.vpisi(0);
        // }}
    }

    pub fn je_enolicno_resljivo(&self) -> bool {
        let mut resnicnost = true;
        for celica in &self.mreza {
            if celica.stevilo == 0 {
                if celica.moznosti.len() != 1 && celica.moznosti.len() != 0 {
                    resnicnost = false
                }
            };
            if !&celica.ali_je_vrstica_okej(&self) {
                resnicnost = false
            };
            if !&celica.ali_je_stolpec_okej(&self) {
                resnicnost = false
            };
            if !&celica.ali_je_skatla_okej(&self) {
                resnicnost = false
            };
        }
        return resnicnost;
        //dvomim da je tole čisto pravilno. potrebujem tok da neki nrdi da lahko vzpostavim sporočilo ki se prikaže, ko je zadeva enolično rešljiva
    }

    pub fn ugotovi_stevila_v_vrstici(&self, vrst: u8) -> Vec<u8> {
        let mut ze_vpisana_st = vec![];
        let mut ni_v_vrstici = vec![];
        for polje in &self.mreza {
            if polje.vrstica == vrst {
                match polje.stevilo {
                    0 => {}
                    i => ze_vpisana_st.push(i),
                }
            }
        }
        for i in 1..=9 {
            if ze_vpisana_st.contains(&i) {
            } else {
                ni_v_vrstici.push(i)
            }
        }
        return ni_v_vrstici;
    }

    pub fn ugotovi_stevila_v_stolpcu(&self, stolp: u8) -> Vec<u8> {
        let mut ze_vpisana_st = vec![];
        let mut ni_v_stolpcu = vec![];
        for polje in &self.mreza {
            if polje.stolpec == stolp {
                match polje.stevilo {
                    0 => {}
                    i => ze_vpisana_st.push(i),
                }
            }
        }
        for i in 1..=9 {
            if ze_vpisana_st.contains(&i) {
            } else {
                ni_v_stolpcu.push(i)
            }
        }
        return ni_v_stolpcu;
    }

    pub fn ugotovi_stevila_v_skatli(&self, skatla: u8) -> Vec<u8> {
        let mut ze_vpisana_st = vec![];
        let mut ni_v_skatli = vec![];
        for polje in &self.mreza {
            if polje.ugotovi_skatlo() == skatla {
                match polje.stevilo {
                    0 => {}
                    i => ze_vpisana_st.push(i),
                }
            }
        }
        for i in 1..=9 {
            if ze_vpisana_st.contains(&i) {
            } else {
                ni_v_skatli.push(i)
            }
        }
        return ni_v_skatli;
    }

    //fn delno_resi() -> vec![Suduku] {} //doda samo tista števila, ki so enolično določena
}

use std::fmt::Display;
use std::fmt::Formatter;
//use std::string;

impl Display for Suduku {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self.mreza)
    }
}

impl Display for Polje {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{} {} {}", self.vrstica, self.stolpec, self.stevilo)
    }
}
