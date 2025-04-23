
#[derive(Debug, PartialEq)]
pub struct Polje {
    pub vrstica: u8,
    pub stolpec: u8,
    pub stevilo: Obstoj,
    pub moznosti: Vec<u8>,
}

pub struct Suduku {
    pub mreza: Vec<Polje>,
    pub trenutno_polje: usize, //kao indeks v seznamu
}

#[derive(PartialEq, Debug)]
pub enum Obstoj {
    Prazno,
    Polno(u8),
}


