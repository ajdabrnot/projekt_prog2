#[derive(Debug, PartialEq, Clone)]
pub struct Polje {
    pub vrstica: u8,
    pub stolpec: u8,
    pub stevilo: u8,
    pub moznosti: Vec<u8>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Suduku {
    pub mreza: Vec<Polje>,
    pub trenutno_polje: usize, //kao indeks v seznamu
}

#[derive(PartialEq, Debug, Clone)]
pub enum Obstoj {
    Prazno,
    Polno(u8),
}
