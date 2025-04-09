pub struct Polje {
    vrstica: u8,
    stolpec: u8,
    stevilo: u8,
    moznosti: Vec<u8>,
}

pub struct Suduku {
    mreza: Vec<Polje>,
    trenutno_polje: Polje,
} //naredi kot seznam seznamov

pub enum Obstoj {
    Prazno,
    Polno(u8),
}

impl Obstoj {
    fn veljavnost(&self) -> bool{
        match self {
            &Self::Prazno => true,
            &Self::Polno(n) => {
                if 0 < n && n < 10 then true else false
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
    fn prazno() -> Polje{
        Polje {
            vrstica: 0,
            stolpec: 0,
            stevilo: 0,
            moznosti: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        },
    }
}

impl Suduku {
    fn prazen() -> Suduku {
        Suduku {
            mreza: vec![],
            trenutno_polje: Polje {
                vrstica: 0,
                stolpec: 0,
                stevilo: 0,
                moznosti: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            },
        }
    }
}

fn main() {
    println!("Hello, world!");
}
