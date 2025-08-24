//implementacije za struct Suduku

use crate::strukture::{Polje, Resevanje, Suduku};

impl Suduku {
    pub fn prazen_suduku() -> Suduku {
        //ustvari prazen sudoku
        let mut tabela = vec![];
        for i in 1..=9 {
            for j in 1..10 {
                let polje = Polje::prazno_polje(i, j);
                tabela.push(polje);
            }
        }
        Suduku {
            mreza: tabela,
            //trenutno_polje: 0,
        }
    }

    pub fn manjkajoca_v_skatli(&self, skatla: u8) -> Vec<u8> {
        //vrne števila, ki jih še ni v škatli
        let mut ze_vpisana_st = vec![];
        let mut ni_v_skatli = vec![];
        for polje in &self.mreza {
            if polje.skatla == skatla {
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

    pub fn manjkajoca_v_stolpcu(&self, stolp: u8) -> Vec<u8> {
        //vrne števila, ki jih še ni v stolpcu
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

    pub fn manjkajoca_v_vrstici(&self, vrst: u8) -> Vec<u8> {
        //vrne števila, ki jih še ni v vrstici
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

    pub fn napolni_polje(&mut self, vrst: u8, stolp: u8, st: u8) -> () {
        let indeks = (vrst as usize - 1) * 9 + (stolp as usize) - 1;
        let mut polje = Polje::prazno_polje(vrst, stolp);
        polje.vpisi(st);
        ////let polje1 = Polje {
        ////    vrstica: vrst,
        ////    stolpec: stolp,
        ////    skatla: ugotovi_skatlo(vrst, stolp),
        ////    stevilo: st,
        ////    moznosti: vec![],
        ////};
        //let v_vrstici = polje.ali_je_vrstica_okej(self);
        //let v_stolpcu = polje.ali_je_stolpec_okej(self);
        //let v_skatli = polje.ali_je_skatla_okej(self);
        //if v_vrstici && v_stolpcu && v_skatli {
        //    self.mreza[indeks] = polje
        //} else {
        //}

        if self.ali_je_veljavno(vrst, stolp, st) {
            self.mreza[indeks] = polje
        }
        // for polje in &self.mreza {
        //     polje.ugotovi_moznosti(self)
        // }
    }

    pub fn napolni_polja(&mut self, sez: Vec<(u8, u8, u8)>) -> () {
        //napolne več polj hkrati
        for elt in sez {
            match elt {
                (a, b, c) => self.napolni_polje(a, b, c),
            }
        }
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
            //trenutno_polje: 0,
        }
    }

    pub fn ali_je_veljavno(&self, vrst: u8, stolp: u8, st: u8) -> bool {
        //preveri, ali je izbrana stevka veljavna izbira za to polje

        //let polje = Polje {
        //    vrstica: vrst,
        //    stolpec: stolp,
        //    skatla: ugotovi_skatlo(vrst, stolp),
        //    stevilo: st,
        //    moznosti: vec![],
        //};

        let mut polje = Polje::prazno_polje(vrst, stolp);
        polje.ugotovi_moznosti(self);
        return polje.moznosti.contains(&st);

        //polje.vpisi(st);
        //return polje.ali_je_vrstica_okej(self)
        //    && polje.ali_je_stolpec_okej(self)
        //    && polje.ali_je_skatla_okej(self);
    }

    fn vpisi_enolicno_dolocena_stevila(&mut self) -> () {
        //zapolni polja katerih rešitev je že enolično določena
        for mut polje in self.mreza.clone() {
            if polje.moznosti.len() == 1 {
                polje.vpisi(polje.moznosti[0]);
            }
        }
    }

    pub fn sudoku_kot_seznam_samo_vrednosti(&self) -> Vec<Vec<u8>> {
        //vrne mrežo zapisano kot
        //[[0,0,0,0,0,0,0,0,0],
        //[0,0,0,0,0,0,0,0,0],
        //[0,0,0,0,0,0,0,0,0],
        //[0,0,0,0,0,0,0,0,0],
        //[0,0,0,0,0,0,0,0,0],
        //[0,0,0,0,0,0,0,0,0],
        //[0,0,0,0,0,0,0,0,0],
        //[0,0,0,0,0,0,0,0,0],
        //[0,0,0,0,0,0,0,0,0]];
        //kjer so namesto ničel zapisane uztrezne števke
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
    //         let skatla = manjkajoca_v_skatli(polje.ugotovi_skatlo(), &self);

    //     for i in vrstica {
    //         if stolpec.contains(&i) && skatla.contains(&i) {
    //             polje.moznosti.push(i)
    //         } else {
    //         }
    //     }}

    // }
    // ne dela :(

    pub fn prazna_polja(&self) -> Vec<usize> {
        //vrne indekse praznih polj
        let mut prazna = vec![];
        for i in 0..81 {
            if self.mreza[i].stevilo == 0 {
                prazna.push(i);
            }
        }
        return prazna;
    }

    pub fn prvo_prazno_polje(&self) -> Option<usize> {
        //vrne indeks prvega praznega polja
        for i in 0..81 {
            if self.mreza[i].stevilo == 0 {
                return Some(i);
            }
        }
        return None;
    }

    pub fn resi_sudoku(&mut self) -> () {
        //self.resi_sudoku_rekurzivna()
        // for polje in &mut self.mreza {
        //     if polje.stevilo == 0 {polje.stevilo = 6}}

        let mut resen = Resevanje::nov_za_resevanje(self);
        if resen.resi() {
            self.mreza = resen.sudoku_za_resevanje.mreza.clone();
            for polje in &mut self.mreza {
                if polje.stevilo == 0 {
                    polje.stevilo = 6
                }
            }
        };
        for polje in &mut self.mreza {
            if polje.stevilo == 0 {
                polje.stevilo = 6
            }
        }
    }

    //pub fn resi_sudoku_rekurzivna_1(&mut self) -> bool {
    //    //zapolni že enolično določena polja
    //    self.vpisi_enolicno_dolocena_stevila();
    //
    //    //za večlična polja preveri možnosti od 1 do 9:
    //    for polje in &mut self.mreza { //tuki je težava da ta zanka vzame lastništvo mreže.
    //        if polje.stevilo == 0 {
    //            for st in 1..=9 {
    //                polje.stevilo = st;
    //                if self.resi_sudoku() {
    //                    return true;
    //                }
    //                polje.stevilo = 0;
    //            }
    //            return false; //je nerešljivo za to izbiro števila pri nekem prejšnem polju oz je nasploh nerešljivo?
    //        }
    //
    //    }
    //    return true;
    //    // if self.je_enolicno_resljivo() {
    //    //     self.vpisi_enolicno_dolocena_stevila();
    //    // }}
    //    // else {for polje in &mut self.mreza {
    //    //     polje.vpisi(0);
    //    // }}
    //}

    //pub fn resi_sudoku_rekurzivna(&mut self) -> bool {
    //    //zapolni že enolično določena polja
    //    self.vpisi_enolicno_dolocena_stevila();
    //
    //    let indeks = self.prvo_prazno_polje();
    //    match indeks {
    //        None => false, //(?)ni praznih polj
    //        Some(i) => {
    //            for st in &self.mreza[i].moznosti {
    //                self.mreza[i].stevilo = st;
    //                if self.resi_sudoku() {
    //                    return true;
    //                }
    //                self.mreza[i].stevilo = 0;
    //            }
    //            return false; // s temi ugibi se ne pride čez, .resi_sudoku naj odneha
    //        }
    //    };
    //    return true; //smo prišli čez vsa polja??? in je vse rešeno
    //}

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
    //fn delno_resi() -> vec![Suduku] {} //doda samo tista števila, ki so enolično določena
}
