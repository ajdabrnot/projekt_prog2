use crate::strukture::{Polje, Suduku};

pub fn izracunaj_indeks(vrst: usize, stolp: usize) -> usize {
    //vrne indeks polja v mreži
    (vrst as usize) * 9 + (stolp as usize)
}

pub fn ugotovi_skatlo(vrst: u8, stolp: u8) -> u8 {
    //izračuna škatlo glede na vrstico in stolpec
    match vrst {
        1 | 2 | 3 => match stolp {
            1 | 2 | 3 => 1,
            4 | 5 | 6 => 2,
            7 | 8 | 9 => 3,
            _ => 0,
        },
        4 | 5 | 6 => match stolp {
            1 | 2 | 3 => 4,
            4 | 5 | 6 => 5,
            7 | 8 | 9 => 6,
            _ => 0,
        },
        7 | 8 | 9 => match stolp {
            1 | 2 | 3 => 7,
            4 | 5 | 6 => 8,
            7 | 8 | 9 => 9,
            _ => 0,
        },
        _ => 0,
    }
}
pub fn povezi_skatlo_z_indeksi(skatla: u8) -> Vec<u8> {
    match skatla {
        1 => [0,1,2,9,10,11,18,19,20].to_vec(),
        2 => [3,4,5,12,13,14,21,22,23].to_vec(),
        3 => [6,7,8,15,16,17,24,25,26].to_vec(),
        4 => [27,28,29,36,37,38,45,46,47].to_vec(),
        5 => [30,31,32,39,40,41,48,49,50].to_vec(),
        6 => [33,34,35,42,43,44,51,52,53].to_vec(),
        7 => [54,55,56,63,64,65,72,73,74].to_vec(),
        8 => [57,58,59,66,67,68,75,76,77].to_vec(),
        9 => [60,61,62,69,70,71,78,79,80].to_vec(),
        _ => [].to_vec()
    }
}

pub fn ponovitve_stevila(sudoku: &Suduku, polje: &Polje) -> Vec<(usize, usize)> {
    //vrne polja ki so v istem stolpcu/vrstici/škatli in imajo isto število
    //uporabljeno pri končnem izgledu sudokuja (da se celice s ponovitvami obarvajo)
    let mut odg = vec![];
    let skatla_polje = polje.skatla;
    for celica in &sudoku.mreza {
        let skatla_celica = celica.skatla;
        if celica.stevilo == polje.stevilo
            && celica.stevilo != 0
            && polje.stevilo != 0
            && celica.vrstica == polje.vrstica
            && celica.stolpec != polje.stolpec
        {
            odg.push((celica.vrstica as usize, celica.stolpec as usize))
        } else if celica.stevilo == polje.stevilo
            && celica.stevilo != 0
            && polje.stevilo != 0
            && celica.vrstica != polje.vrstica
            && celica.stolpec == polje.stolpec
        {
            odg.push((celica.vrstica as usize, celica.stolpec as usize))
        } else if celica.stevilo == polje.stevilo
            && celica.stevilo != 0
            && polje.stevilo != 0
            && (celica.vrstica != polje.vrstica || celica.stolpec != polje.stolpec)
            && skatla_polje == skatla_celica
        {
            odg.push((celica.vrstica as usize, celica.stolpec as usize))
        }
    }
    return odg;
}

use std::fmt::Display;
use std::fmt::Formatter;

impl Display for Suduku {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self.mreza)
    }
}

impl Display for Polje {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{} {} {}", self.vrstica, self.stolpec, self.stevilo)
    }
}
