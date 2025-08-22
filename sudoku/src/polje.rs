impl Polje {
    pub fn ugotovi_skatlo(&self) -> u8 {
        match self.vrstica {
            1 | 2 | 3 => match self.stolpec {
                1 | 2 | 3 => 1,
                4 | 5 | 6 => 2,
                7 | 8 | 9 => 3,
                _ => 0,
            },
            4 | 5 | 6 => match self.stolpec {
                1 | 2 | 3 => 4,
                4 | 5 | 6 => 5,
                7 | 8 | 9 => 6,
                _ => 0,
            },
            7 | 8 | 9 => match self.stolpec {
                1 | 2 | 3 => 7,
                4 | 5 | 6 => 8,
                7 | 8 | 9 => 9,
                _ => 0,
            },
            _ => 0,
        }
    }
    pub fn ali_je_vrstica_okej(&self, suduku: &Suduku) -> bool {
        let mut resnicnost = true;
        for polje in suduku.mreza.clone() {
            if polje.vrstica == self.vrstica {
                if polje.stevilo == self.stevilo {
                    resnicnost = false;
                } else {
                }
            } else {
            }
        }
        return self.vrstica > 0 && self.vrstica < 10 && resnicnost;
    }
    /// poj bo mogl se prevert, ce je ze ksna ista stevilka not vpisana

    pub fn ali_je_skatla_okej(&self, suduku: &Suduku) -> bool {
        let mut resnicnost = true;
        for polje in suduku.mreza.clone() {
            if polje.ugotovi_skatlo() == self.ugotovi_skatlo() {
                if polje.stevilo == self.stevilo {
                    resnicnost = false;
                } else {
                }
            } else {
            }
        }
        return self.ugotovi_skatlo() > 0 && self.ugotovi_skatlo() < 10 && resnicnost;
    }
    pub fn ali_je_stolpec_okej(&self, suduku: &Suduku) -> bool {
        let mut resnicnost = true;
        for polje in suduku.mreza.clone() {
            if polje.stolpec == self.stolpec {
                if polje.stevilo == self.stevilo {
                    resnicnost = false;
                } else {
                }
            } else {
            }
        }
        return self.stolpec > 0 && self.stolpec < 10 && resnicnost;
    }
    pub fn ugotovi_moznosti(&mut self, suduku: &mut Suduku) -> () {
        let vrstica = ugotovi_stevila_v_vrstici(self.vrstica, &suduku);
        let stolpec = ugotovi_stevila_v_stolpcu(self.stolpec, &suduku);
        let skatla = ugotovi_stevila_v_skatli(self.ugotovi_skatlo(), &suduku);
        for i in vrstica {
            if stolpec.contains(&i) && skatla.contains(&i) {
                self.moznosti.push(i)
            } else {
            }
        }
    }
    pub fn ugotovi_moznosti_in_vrni_nov_sudoku(&mut self, suduku: Suduku) -> Suduku {
        let nov_sudoku = suduku.kopiraj_sudoku();
        let vrstica = ugotovi_stevila_v_vrstici(self.vrstica, &nov_sudoku);
        let stolpec = ugotovi_stevila_v_stolpcu(self.stolpec, &nov_sudoku);
        let skatla = ugotovi_stevila_v_skatli(self.ugotovi_skatlo(), &nov_sudoku);
        for i in vrstica {
            if stolpec.contains(&i) && skatla.contains(&i) {
                self.moznosti.push(i)
            } else {
            }
        }
        return nov_sudoku;
    }

    pub fn vpisi(&mut self, stevilo: u8) -> () {
        if self.moznosti.contains(&stevilo) {
            self.stevilo = stevilo;
            self.moznosti = vec![]
        };
    }

    pub fn prazno_polje(vrst: u8, stolp: u8) -> Polje {
        Polje {
            vrstica: vrst,
            stolpec: stolp,
            stevilo: 0,
            moznosti: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        }
    }
}
