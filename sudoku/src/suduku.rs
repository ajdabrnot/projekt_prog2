//implementacije za struct Suduku

use crate::{pomozne_funkcije::povezi_skatlo_z_indeksi, strukture::{Polje, Resevanje, Suduku}};

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
        Suduku { mreza: tabela }
    }

    // pub fn manjkajoca_v_skatli(&self, skatla: u8) -> Vec<u8> {
    //     //vrne števila, ki jih še ni v škatli
    //     let mut ze_vpisana_st = vec![];
    //     let mut ni_v_skatli = vec![];
    //     for polje in &self.mreza {
    //         if polje.skatla == skatla {
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

    pub fn manjkajoca_v_skatli_boljsi_nacin(&self, skatla: u8) -> Vec<u8> {
        //vrne števila, ki jih še ni v škatli
        let mut ze_vpisana_st = vec![];
        let mut ni_v_skatli = vec![];
        let seznam_indeksov = povezi_skatlo_z_indeksi(skatla);
        for i in seznam_indeksov {
            match self.mreza[i as usize].stevilo {
                0 => {}
                i => ze_vpisana_st.push(i),
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
    

    // pub fn manjkajoca_v_stolpcu(&self, stolp: u8) -> Vec<u8> {
    //     //vrne števila, ki jih še ni v stolpcu
    //     let mut ze_vpisana_st = vec![];
    //     let mut ni_v_stolpcu = vec![];
    //     for polje in &self.mreza {
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

    pub fn manjkajoca_v_stolpcu_boljsi_nacin(&self, stolp: u8) -> Vec<u8> {
        //vrne števila, ki jih še ni v stolpcu
        let mut ze_vpisana_st = vec![];
        let mut ni_v_stolpcu = vec![];
        for i in 0..9 {
            match self.mreza[i * 9 + stolp as usize - 1].stevilo {
                0 => {}
                i => ze_vpisana_st.push(i),
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


    // pub fn manjkajoca_v_vrstici(&self, vrst: u8) -> Vec<u8> {
    //     //vrne števila, ki jih še ni v vrstici
    //     let mut ze_vpisana_st = vec![];
    //     let mut ni_v_vrstici = vec![];
    //     for polje in &self.mreza {
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

    pub fn manjkajoca_v_vrstici_boljsi_nacin(&self, vrst: u8) -> Vec<u8> {
        //vrne števila, ki jih še ni v vrstici
        let mut ze_vpisana_st = vec![];
        let mut ni_v_vrstici = vec![];
        let indeks_zacetka_vrstice = (vrst - 1) * 9;
        let indeks_konca_vrstice = (vrst - 1) * 9 + 9;
        for i in indeks_zacetka_vrstice..indeks_konca_vrstice {
            match self.mreza[i as usize].stevilo {
                0 => {}
                i => ze_vpisana_st.push(i),
            }
        }
        for st in 1..=9 {
            if ze_vpisana_st.contains(&st) {
            } else {
                ni_v_vrstici.push(st)
            }
        }
        return ni_v_vrstici;
    }

    pub fn ali_je_veljavno(&self, vrst: u8, stolp: u8, st: u8) -> bool {
        //preveri, ali je izbrana stevka veljavna izbira za to polje
        let mut polje = Polje::prazno_polje(vrst, stolp);
        polje.ugotovi_moznosti(self);
        return polje.moznosti.contains(&st);
    }

    pub fn napolni_polje(&mut self, vrst: u8, stolp: u8, st: u8) -> () {
        let indeks = (vrst as usize - 1) * 9 + (stolp as usize) - 1;
        let mut polje = Polje::prazno_polje(vrst, stolp);
        polje.vpisi_stevilo(st);

        if self.ali_je_veljavno(vrst, stolp, st) {
            self.mreza[indeks] = polje
        }
    }

    pub fn napolni_polja(&mut self, sez: Vec<(u8, u8, u8)>) -> () {
        //napolne več polj hkrati (predvsem koristno za teste)
        for elt in sez {
            match elt {
                (a, b, c) => self.napolni_polje(a, b, c),
            }
        }
    }

    pub fn kopiraj_sudoku(&self) -> Suduku {
        Suduku {
            mreza: self.mreza.clone(),
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

    pub fn sudoku_kot_niz_vrednosti(&self) -> String {
        //vrne sudoku zapisan kot
        //'000000000000000000000000000000000000000000000000000000000000000000000000000000000'
        //kjer so namestno ničel zapisane uztrezne števke
        let mut niz = "".to_string();
        for i in 0..81 {
            match self.mreza[i].stevilo {
                1 => niz.push('1'),
                2 => niz.push('2'),
                3 => niz.push('3'),
                4 => niz.push('4'),
                5 => niz.push('5'),
                6 => niz.push('6'),
                7 => niz.push('7'),
                8 => niz.push('8'),
                9 => niz.push('9'),
                0 => niz.push('0'),

                _a => niz.push('a'),
            }
        }
        return niz;
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

    pub fn hitro_resi_sudoku(&mut self) -> () {
        use sudoku::Sudoku;
        let stevila = self.sudoku_kot_niz_vrednosti();
        let sudoku = Sudoku::from_str_line(&stevila).unwrap();

        match sudoku.solution() {
            Some(resitev) => {
                let mut niz = format!("{}", resitev);
                for i in 0..81 {
                    match niz.pop() {
                        Some('1') => self.mreza[80 - i].vpisi_stevilo(1),
                        Some('2') => self.mreza[80 - i].vpisi_stevilo(2),
                        Some('3') => self.mreza[80 - i].vpisi_stevilo(3),
                        Some('4') => self.mreza[80 - i].vpisi_stevilo(4),
                        Some('5') => self.mreza[80 - i].vpisi_stevilo(5),
                        Some('6') => self.mreza[80 - i].vpisi_stevilo(6),
                        Some('7') => self.mreza[80 - i].vpisi_stevilo(7),
                        Some('8') => self.mreza[80 - i].vpisi_stevilo(8),
                        Some('9') => self.mreza[80 - i].vpisi_stevilo(9),
                        Some(_a) => {}

                        None => {}
                    }
                }
            }
            None => {}
        }
    }

    pub fn resi_sudoku_pocasi(&mut self) -> () {
        let mut resen = Resevanje::nov_za_resevanje(self);
        if resen.resi(true) {
            self.mreza = resen.sudoku_za_resevanje.mreza.clone();
        };
    }

    pub fn je_enolicno_resljivo_hitra(&self) -> bool {
        use sudoku::Sudoku;
        let stevila = self.sudoku_kot_niz_vrednosti();
        let sudoku = Sudoku::from_str_line(&stevila).unwrap();
        return sudoku.is_uniquely_solvable();
    }

    // pub fn je_enolicno_resljivo_pocasi(&self) -> bool {
    //     let mut kopirani_sudoku_prvic = self.kopiraj_sudoku();
    //     let mut kopirani_sudoku_drugic = self.kopiraj_sudoku();
    //     let mut resen_prvic = Resevanje::nov_za_resevanje(&mut kopirani_sudoku_prvic);
    //     let mut resen_drugic = Resevanje::nov_za_resevanje(&mut kopirani_sudoku_drugic);
    //     if resen_prvic.resi(true) && resen_drugic.resi_se_enkrat() {
    //         return resen_prvic.sudoku_za_resevanje.mreza == resen_drugic.sudoku_za_resevanje.mreza;
    //     }
    //     return false;
    // }
    pub fn je_enolicno_resljivo_pocasi(&self) -> bool {
        let mut kopirani_sudoku_prvic = self.kopiraj_sudoku();
        let mut kopirani_sudoku_drugic = self.kopiraj_sudoku();
        let mut resen_prvic = Resevanje::nov_za_resevanje(&mut kopirani_sudoku_prvic);
        let mut resen_drugic = Resevanje::nov_za_resevanje(&mut kopirani_sudoku_drugic);
        if resen_prvic.resi(true) && resen_drugic.resi(false) {
            return resen_prvic.sudoku_za_resevanje.mreza == resen_drugic.sudoku_za_resevanje.mreza;
        }
        return false;
    }

    pub fn ali_je_resljiv_hitro(&self) -> bool {
        use sudoku::Sudoku;
        let stevila = self.sudoku_kot_niz_vrednosti();
        let sudoku = Sudoku::from_str_line(&stevila).unwrap();
        match sudoku.some_solution() {
            Some(_a) => true,
            None => false,
        }
    }

    pub fn ali_je_sudoku_resljiv_pocasi(&self) -> bool {
        let mut kopirani_sudoku = self.kopiraj_sudoku();
        let mut resen = Resevanje::nov_za_resevanje(&mut kopirani_sudoku);
        return resen.resi(true);
    }
}
