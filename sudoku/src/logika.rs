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

pub fn pojavitve_stevila(sudoku: &Suduku, polje: &Polje) -> Vec<(usize, usize)> {
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
//use std::string;

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
