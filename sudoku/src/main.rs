pub struct Polje {
    vrstica: u8,
    stolpec: u8,
    stevilo: Obstoj,
    moznosti: Vec<u8>,
}

pub struct Suduku {
    mreza: Vec<Polje>,
    trenutno_polje: usize, //kao indeks v seznamu
} 

pub enum Obstoj {
    Prazno,
    Polno(u8),
}


impl Obstoj {
    fn veljavnost(&self) -> bool{
        match self {
            &Self::Prazno => true,
            &Self::Polno(n) => {
                n > 0 && n < 10
            }
        }
    }
}

impl Polje {
    fn ugotovi_skatlo(&self) -> u8 {
        42
    }
    fn ali_je_vrstica_okej(&self, suduku: Suduku) -> bool {
        false
    }
    fn ali_je_skatla_okej(&self, suduku: Suduku) -> bool {
        false
    }
    fn ali_je_stolpec_okej(&self, suduku: Suduku) -> bool {
        false
    }
    pub fn prazno_polje(vrst: u8, stolp: u8) -> Polje{
        Polje {
            vrstica: vrst,
            stolpec: stolp,
            stevilo: Obstoj::Prazno,
            moznosti: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        }
    }
}

impl Suduku {
    fn prazen_suduku() -> Suduku {
        let mut tabela = vec![];
        for i in  1..=9 {
            for j in 1..10 {
                let polje = Polje::prazno_polje(i, j);
                tabela.push(polje);
            }
        }
        Suduku {
            mreza: tabela,
            trenutno_polje: 0
        }
    }

    fn napolni_polje(&mut self, vrst: u8, stolp: u8, st: u8) -> () {
        let indeks = (vrst as usize) * 9 + (stolp as usize);
        let polje = Polje {
            vrstica: vrst,
            stolpec: stolp,
            stevilo: Obstoj::Polno(st),
            moznosti: vec![],
        };
        self.mreza[indeks] = polje;
    } 
}

fn main() {
    println!("Hello, world!");
}
