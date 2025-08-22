use crate::strukture::{Polje, Suduku};

pub fn izracunaj_indeks(vrst: usize, stolp: usize) -> usize {
    (vrst as usize) * 9 + (stolp as usize)
}

pub fn pojavitve_stevila(sudoku: &Suduku, polje: &Polje) -> Vec<(usize, usize)> {
    //vrne polja ki so v istem stolpcu/vrstici/škatli in imajo isto število
    //uporabljeno pri končnem izgledu sudokuja
    let mut odg = vec![];
    let skatla_polje = polje.ugotovi_skatlo();
    for celica in &sudoku.mreza {
        let skatla_celica = celica.ugotovi_skatlo();
        if celica.stevilo == polje.stevilo
            && celica.stevilo != 0
            && polje.stevilo != 0
            && celica.vrstica == polje.vrstica
            && celica.stolpec != polje.stolpec
        {
            odg.push((celica.vrstica as usize, celica.stolpec as usize))
        } else if celica.stevilo == polje.stevilo
            && celica.stevilo != 0
            && polje.stevilo != 0
            && celica.vrstica != polje.vrstica
            && celica.stolpec == polje.stolpec
        {
            odg.push((celica.vrstica as usize, celica.stolpec as usize))
        } else if celica.stevilo == polje.stevilo
            && celica.stevilo != 0
            && polje.stevilo != 0
            && (celica.vrstica != polje.vrstica || celica.stolpec != polje.stolpec)
            && skatla_polje == skatla_celica
        {
            odg.push((celica.vrstica as usize, celica.stolpec as usize))
        }
    }
    return odg;
}

// pub fn ugotovi_stevila_v_vrstici(vrst: u8, suduku: &Suduku) -> Vec<u8> {
//     //vrne števila, ki jih še ni v vrstici
//     let mut ze_vpisana_st = vec![];
//     let mut ni_v_vrstici = vec![];
//     for polje in &suduku.mreza {
//         if polje.vrstica == vrst {
//             match polje.stevilo {
//                 0 => {}
//                 i => ze_vpisana_st.push(i),
//             }
//         }
//     }
//     for i in 1..=9 {
//         if ze_vpisana_st.contains(&i) {
//         } else {
//             ni_v_vrstici.push(i)
//         }
//     }
//     return ni_v_vrstici;
// }
// pub fn ugotovi_stevila_v_stolpcu(stolp: u8, suduku: &Suduku) -> Vec<u8> {
//     //vrne števila, ki jih še ni v stolpcu
//     let mut ze_vpisana_st = vec![];
//     let mut ni_v_stolpcu = vec![];
//     for polje in &suduku.mreza {
//         if polje.stolpec == stolp {
//             match polje.stevilo {
//                 0 => {}
//                 i => ze_vpisana_st.push(i),
//             }
//         }
//     }
//     for i in 1..=9 {
//         if ze_vpisana_st.contains(&i) {
//         } else {
//             ni_v_stolpcu.push(i)
//         }
//     }
//     return ni_v_stolpcu;
// }
// pub fn ugotovi_stevila_v_skatli(skatla: u8, suduku: &Suduku) -> Vec<u8> {
//     //vrne števila, ki jih še ni v škatli
//     let mut ze_vpisana_st = vec![];
//     let mut ni_v_skatli = vec![];
//     for polje in &suduku.mreza {
//         if polje.ugotovi_skatlo() == skatla {
//             match polje.stevilo {
//                 0 => {}
//                 i => ze_vpisana_st.push(i),
//             }
//         }
//     }
//     for i in 1..=9 {
//         if ze_vpisana_st.contains(&i) {
//         } else {
//             ni_v_skatli.push(i)
//         }
//     }
//     return ni_v_skatli;
// }

// impl Polje {
//     pub fn ugotovi_skatlo(&self) -> u8 {
//         match self.vrstica {
//             1 | 2 | 3 => match self.stolpec {
//                 1 | 2 | 3 => 1,
//                 4 | 5 | 6 => 2,
//                 7 | 8 | 9 => 3,
//                 _ => 0,
//             },
//             4 | 5 | 6 => match self.stolpec {
//                 1 | 2 | 3 => 4,
//                 4 | 5 | 6 => 5,
//                 7 | 8 | 9 => 6,
//                 _ => 0,
//             },
//             7 | 8 | 9 => match self.stolpec {
//                 1 | 2 | 3 => 7,
//                 4 | 5 | 6 => 8,
//                 7 | 8 | 9 => 9,
//                 _ => 0,
//             },
//             _ => 0,
//         }
//     }
//     pub fn ali_je_vrstica_okej(&self, suduku: &Suduku) -> bool {
//         let mut resnicnost = true;
//         for polje in suduku.mreza.clone() {
//             if polje.vrstica == self.vrstica {
//                 if polje.stevilo == self.stevilo {
//                     resnicnost = false;
//                 } else {
//                 }
//             } else {
//             }
//         }
//         return self.vrstica > 0 && self.vrstica < 10 && resnicnost;
//     }
//     /// poj bo mogl se prevert, ce je ze ksna ista stevilka not vpisana

//     pub fn ali_je_skatla_okej(&self, suduku: &Suduku) -> bool {
//         let mut resnicnost = true;
//         for polje in suduku.mreza.clone() {
//             if polje.ugotovi_skatlo() == self.ugotovi_skatlo() {
//                 if polje.stevilo == self.stevilo {
//                     resnicnost = false;
//                 } else {
//                 }
//             } else {
//             }
//         }
//         return self.ugotovi_skatlo() > 0 && self.ugotovi_skatlo() < 10 && resnicnost;
//     }
//     pub fn ali_je_stolpec_okej(&self, suduku: &Suduku) -> bool {
//         let mut resnicnost = true;
//         for polje in suduku.mreza.clone() {
//             if polje.stolpec == self.stolpec {
//                 if polje.stevilo == self.stevilo {
//                     resnicnost = false;
//                 } else {
//                 }
//             } else {
//             }
//         }
//         return self.stolpec > 0 && self.stolpec < 10 && resnicnost;
//     }
//     pub fn ugotovi_moznosti(&mut self, suduku: &mut Suduku) -> () {
//         let vrstica = ugotovi_stevila_v_vrstici(self.vrstica, &suduku);
//         let stolpec = ugotovi_stevila_v_stolpcu(self.stolpec, &suduku);
//         let skatla = ugotovi_stevila_v_skatli(self.ugotovi_skatlo(), &suduku);
//         for i in vrstica {
//             if stolpec.contains(&i) && skatla.contains(&i) {
//                 self.moznosti.push(i)
//             } else {
//             }
//         }
//     }
//     pub fn ugotovi_moznosti_in_vrni_nov_sudoku(&mut self, suduku: Suduku) -> Suduku {
//         let nov_sudoku = suduku.kopiraj_sudoku();
//         let vrstica = ugotovi_stevila_v_vrstici(self.vrstica, &nov_sudoku);
//         let stolpec = ugotovi_stevila_v_stolpcu(self.stolpec, &nov_sudoku);
//         let skatla = ugotovi_stevila_v_skatli(self.ugotovi_skatlo(), &nov_sudoku);
//         for i in vrstica {
//             if stolpec.contains(&i) && skatla.contains(&i) {
//                 self.moznosti.push(i)
//             } else {
//             }
//         }
//         return nov_sudoku;
//     }

//     pub fn vpisi(&mut self, stevilo: u8) -> () {
//         if self.moznosti.contains(&stevilo) {
//             self.stevilo = stevilo;
//             self.moznosti = vec![]
//         };
//     }

//     pub fn prazno_polje(vrst: u8, stolp: u8) -> Polje {
//         Polje {
//             vrstica: vrst,
//             stolpec: stolp,
//             stevilo: 0,
//             moznosti: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
//         }
//     }
// }

// impl Suduku {
//     pub fn prazen_suduku() -> Suduku {
//         let mut tabela = vec![];
//         for i in 1..=9 {
//             for j in 1..10 {
//                 let polje = Polje::prazno_polje(i, j);
//                 tabela.push(polje);
//             }
//         }
//         Suduku {
//             mreza: tabela,
//             //trenutno_polje: 0,
//         }
//     }

//     pub fn napolni_polje(&mut self, vrst: u8, stolp: u8, st: u8) -> () {
//         let indeks = (vrst as usize - 1) * 9 + (stolp as usize) - 1;
//         let polje = Polje {
//             vrstica: vrst,
//             stolpec: stolp,
//             stevilo: st,
//             moznosti: vec![],
//         };
//         let v_vrstici = polje.ali_je_vrstica_okej(self);
//         let v_stolpcu = polje.ali_je_stolpec_okej(self);
//         let v_skatli = polje.ali_je_skatla_okej(self);
//         if v_vrstici && v_stolpcu && v_skatli {
//             self.mreza[indeks] = polje
//         } else {
//         }
//         // for polje in &self.mreza {
//         //     polje.ugotovi_moznosti(self)
//         // }
//     }

//     // pub fn spremeni_moznosti(&mut self) -> () {
//     //     for i in 0..9 {
//     //         for j in 0..9 {

//     //             self.mreza[9 *  i + j].ugotovi_moznosti(self);
//     //         }
//     //     }
//     // }
//     pub fn kopiraj_sudoku(&self) -> Suduku {
//         Suduku {
//             mreza: self.mreza.clone(),
//             //trenutno_polje: 0,
//         }
//     }

//     pub fn ali_je_veljavno(&self, vrst: u8, stolp: u8, st: u8) -> bool {
//         let polje = Polje {
//             vrstica: vrst,
//             stolpec: stolp,
//             stevilo: st,
//             moznosti: vec![],
//         };
//         return polje.ali_je_vrstica_okej(self)
//             && polje.ali_je_stolpec_okej(self)
//             && polje.ali_je_skatla_okej(self);
//     }

//     fn vpisi_enolicno_dolocena_stevila(&mut self) -> () {
//         for mut polje in self.mreza.clone() {
//             if polje.moznosti.len() == 1 {
//                 polje.vpisi(polje.moznosti[0]);
//             }
//         }
//     }

//     pub fn sudoku_kot_seznam_samo_vrednosti(&self) -> Vec<Vec<u8>> {
//         let mut nov = vec![];
//         let mut manjsi_sez = vec![];
//         for i in 0..81 {
//             if i % 9 == 8 {
//                 manjsi_sez.push(self.mreza[i].stevilo);
//                 nov.push(manjsi_sez);
//                 manjsi_sez = vec![];
//             } else {
//                 manjsi_sez.push(self.mreza[i].stevilo)
//             }
//         }
//         return nov;
//     }

//     // pub fn ugotovi_moznosti_celega_sudokuja_enkrat(&mut self) -> () {
//     //     //gre enkrat cez sudoku
//     //     for mut polje in &self.mreza {
//     //         let vrstica = ugotovi_stevila_v_vrstici(polje.vrstica, &self);
//     //         let stolpec = ugotovi_stevila_v_stolpcu(polje.stolpec, &self);
//     //         let skatla = ugotovi_stevila_v_skatli(polje.ugotovi_skatlo(), &self);

//     //     for i in vrstica {
//     //         if stolpec.contains(&i) && skatla.contains(&i) {
//     //             polje.moznosti.push(i)
//     //         } else {
//     //         }
//     //     }}

//     // }
//     // ne dela :(

//     pub fn prazna_polja(&self) -> Vec<usize> {
//         //vrne indekse praznih polj
//         let mut prazna = vec![];
//         for i in 0..81 {
//             if self.mreza[i].stevilo == 0 {
//                 prazna.push(i);
//             }
//         }
//         return prazna;
//     }

//     pub fn prvo_prazno_polje(&self) -> Option<usize> {
//         //vrne indeks prvega praznega polja
//         for i in 0..81 {
//             if self.mreza[i].stevilo == 0 {
//                 return Some(i);
//             }
//         }
//         return None;
//     }

//     pub fn resi_sudoku(&mut self) -> () {
//         //self.resi_sudoku_rekurzivna()
//         for polje in &mut self.mreza {
//             polje.stevilo = 6
//         }
//     }

//     //pub fn resi_sudoku_rekurzivna_1(&mut self) -> bool {
//     //    //zapolni že enolično določena polja
//     //    self.vpisi_enolicno_dolocena_stevila();
//     //
//     //    //za večlična polja preveri možnosti od 1 do 9:
//     //    for polje in &mut self.mreza { //tuki je težava da ta zanka vzame lastništvo mreže.
//     //        if polje.stevilo == 0 {
//     //            for st in 1..=9 {
//     //                polje.stevilo = st;
//     //                if self.resi_sudoku() {
//     //                    return true;
//     //                }
//     //                polje.stevilo = 0;
//     //            }
//     //            return false; //je nerešljivo za to izbiro števila pri nekem prejšnem polju oz je nasploh nerešljivo?
//     //        }
//     //
//     //    }
//     //    return true;
//     //    // if self.je_enolicno_resljivo() {
//     //    //     self.vpisi_enolicno_dolocena_stevila();
//     //    // }}
//     //    // else {for polje in &mut self.mreza {
//     //    //     polje.vpisi(0);
//     //    // }}
//     //}

//     //pub fn resi_sudoku_rekurzivna(&mut self) -> bool {
//     //    //zapolni že enolično določena polja
//     //    self.vpisi_enolicno_dolocena_stevila();
//     //
//     //    let indeks = self.prvo_prazno_polje();
//     //    match indeks {
//     //        None => false, //(?)ni praznih polj
//     //        Some(i) => {
//     //            for st in &self.mreza[i].moznosti {
//     //                self.mreza[i].stevilo = st;
//     //                if self.resi_sudoku() {
//     //                    return true;
//     //                }
//     //                self.mreza[i].stevilo = 0;
//     //            }
//     //            return false; // s temi ugibi se ne pride čez, .resi_sudoku naj odneha
//     //        }
//     //    };
//     //    return true; //smo prišli čez vsa polja??? in je vse rešeno
//     //}

//     pub fn je_enolicno_resljivo(&self) -> bool {
//         let mut resnicnost = true;
//         for celica in &self.mreza {
//             if celica.stevilo == 0 {
//                 if celica.moznosti.len() != 1 && celica.moznosti.len() != 0 {
//                     resnicnost = false
//                 }
//             };
//             if !&celica.ali_je_vrstica_okej(&self) {
//                 resnicnost = false
//             };
//             if !&celica.ali_je_stolpec_okej(&self) {
//                 resnicnost = false
//             };
//             if !&celica.ali_je_skatla_okej(&self) {
//                 resnicnost = false
//             };
//         }
//         return resnicnost;
//         //dvomim da je tole čisto pravilno. potrebujem tok da neki nrdi da lahko vzpostavim sporočilo ki se prikaže, ko je zadeva enolično rešljiva
//     }
//     //fn delno_resi() -> vec![Suduku] {} //doda samo tista števila, ki so enolično določena
// }

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
