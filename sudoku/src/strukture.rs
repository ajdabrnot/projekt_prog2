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
}

#[derive(Debug, Clone)]
pub enum Msg {
    Polje(usize, usize, usize),
    Resi,
    NavodilaOn,
    NavodilaOff,
    ShraniPdf,
    KorakNazaj,
    KorakNaprej,
    ZacniZnova,
    Barve,
}

pub struct App {
    pub stevilo: usize,
    pub vrstica: usize,
    pub stolpec: usize,
    pub mreza: Suduku,
    pub prikaz_navodil: String,
    pub stare_mreze: Vec<Suduku>,
    pub trenutna_mreza: usize,
    pub barvne_sheme: Vec<String>,
    pub trenutna_barvna_shema: usize,
}

pub struct Resevanje<'a> {
    pub sudoku_za_resevanje: &'a mut Suduku,
}
