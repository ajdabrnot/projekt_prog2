#[derive(Debug, PartialEq, Clone)]
pub struct Polje {
    pub vrstica: u8,
    pub stolpec: u8,
    pub skatla: u8,
    pub stevilo: u8,
    pub moznosti: Vec<u8>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Suduku {
    pub mreza: Vec<Polje>,
    //pub trenutno_polje: usize, //kao indeks v seznamu
}

pub enum Msg {
    Polje(usize, usize, usize),
    Resi,
    NavodilaOn,
    NavodilaOff,
}

pub struct App {
    pub stevilo: usize,
    pub vrstica: usize,
    pub stolpec: usize,
    pub mreza: Suduku,
    pub prikaz_navodil: String,
}

pub struct Resevanje<'a> {
     //tale 'a vrjetno nrdi da se poj zbrise ane????
    pub sudoku_za_resevanje: &'a mut Suduku
}
