use crate::strukture::{Polje, Suduku};

pub fn izracunaj_indeks(vrst: usize, stolp: usize) -> usize {
    (vrst as usize) * 9 + (stolp as usize)
}

pub fn ustvari_id(vrstica: usize, stolpec: usize) -> String {
    //naredi string, ki je uporabljen kot id za celice sudukuja
    let id = format!("r{}c{}", vrstica.to_string(), stolpec.to_string());
    return id;
}
pub fn pojavitve_stevila(sudoku: &Suduku, polje: &Polje) -> Vec<(usize, usize)> {
    //vrne polja ki so v istem stolpcu/vrstici/škatli in imajo isto število
    let mut odg = vec![];
    let skatla_polje = polje.ugotovi_skatlo();
    for celica in &sudoku.mreza {
        let skatla_celica = celica.ugotovi_skatlo();
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
