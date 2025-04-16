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
