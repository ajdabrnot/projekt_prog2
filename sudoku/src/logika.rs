use crate::strukture::{Polje, Obstoj, Suduku};

impl Obstoj {
    pub fn veljavnost(&self) -> bool {
        match self {
            &Self::Prazno => true,
            &Self::Polno(n) => n > 0 && n < 10,
        }
    }
}

impl Polje {
    pub fn ugotovi_skatlo(&self) -> u8 {
        match self.vrstica {
            1 | 2 | 3 => match self.stolpec {
                1 | 2 | 3 => 1,
                4 | 5 | 6 => 2,
                7 | 8 | 9 => 3,
                _ => 0
            },
            4 | 5 | 6 => match self.stolpec {
                1 | 2 | 3 => 4,
                4 | 5 | 6 => 5,
                7 | 8 | 9 => 6,
                _ => 0
            },
            7 | 8 | 9 => match self.stolpec {
                1 | 2 | 3 => 7,
                4 | 5 | 6 => 8,
                7 | 8 | 9 => 9,
                _ => 0
            },
            _ => 0
        }
    }
    pub fn ali_je_vrstica_okej(&self, suduku: Suduku) -> bool {
        return self.vrstica > 0 && self.vrstica < 10
    } 
        /// poj bo mogl se prevert, ce je ze ksna ista stevilka not vpisana
        
    pub fn ali_je_skatla_okej(&self, suduku: Suduku) -> bool {
        return self.ugotovi_skatlo() > 0 && self.ugotovi_skatlo() < 10
    }
    pub fn ali_je_stolpec_okej(&self, suduku: Suduku) -> bool {
        return self.stolpec > 0 && self.stolpec < 10 
    }
    pub fn ugotovi_stevilo_moznosti(&self, suduku: Suduku) -> u32 { 3
    }

    pub fn ugotovi_stevila_v_vrstici(vrst: u8, suduku: Suduku) -> Vec<u8> {
        let mut ze_vpisana_st = vec![];
        let mut ni_v_vrstici = vec![];
        for polje in &suduku.mreza {
            if polje.vrstica == vrst {
                match polje.stevilo {
                    Obstoj::Polno(i) => ze_vpisana_st.push(i),
                    Obstoj::Prazno => {}
                }
            }
        };
        for i in 1..=9 {
            if ze_vpisana_st.contains(&i) {} 
            else {ni_v_vrstici.push(i)}
        };
        return ni_v_vrstici
    }
    pub fn ugotovi_stevila_v_stolpcu(stolp: u8, suduku: Suduku) -> Vec<u8> {
        let mut ze_vpisana_st = vec![];
        let mut ni_v_stolpcu = vec![];
        for polje in &suduku.mreza {
            if polje.stolpec == stolp {
                match polje.stevilo {
                    Obstoj::Polno(i) => ze_vpisana_st.push(i),
                    Obstoj::Prazno => {}
                }
            }
        };
        for i in 1..=9 {
            if ze_vpisana_st.contains(&i) {} 
            else {ni_v_stolpcu.push(i)}
        };
        return ni_v_stolpcu
    }
    pub fn ugotovi_stevila_v_skatli(skatla: u8, suduku: Suduku) -> Vec<u8> {
        let mut ze_vpisana_st = vec![];
        let mut ni_v_skatli = vec![];
        for polje in &suduku.mreza {
            if polje.ugotovi_skatlo() == skatla {
                match polje.stevilo {
                    Obstoj::Polno(i) => ze_vpisana_st.push(i),
                    Obstoj::Prazno => {}
                }
            }
        };
        for i in 1..=9 {
            if ze_vpisana_st.contains(&i) {} 
            else {ni_v_skatli.push(i)}
        };
        return ni_v_skatli
    }

    pub fn vpisi(&mut self, stevilo: u8) -> () {
        if self.moznosti.contains(&stevilo) {
            self.stevilo = Obstoj::Polno(stevilo)
        };
        
    }

    pub fn prazno_polje(vrst: u8, stolp: u8) -> Polje {
        Polje {
            vrstica: vrst,
            stolpec: stolp,
            stevilo: Obstoj::Prazno,
            moznosti: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        }
    }
}

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
        let indeks = (vrst as usize) * 9 + (stolp as usize);
        let polje = Polje {
            vrstica: vrst,
            stolpec: stolp,
            stevilo: Obstoj::Polno(st),
            moznosti: vec![],
        };
        self.mreza[indeks] = polje;
    }

    //fn delno_resi() -> vec![Suduku] {} //doda samo tista števila, ki so enolično določena
}
