use crate::strukture::{Polje, Obstoj, Suduku};
use crate::logika;

#[cfg(test)]
mod tests {
    use crate::testi::Polje;
    use crate::testi::Obstoj;

    #[test]
    fn ustvari_polje() {
        let result = Polje::prazno_polje(2, 3);
    let zeljeno = Polje {
        vrstica: 2,
        stolpec: 3,
        stevilo: 0,
        moznosti: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
    };
    assert_eq!(result, zeljeno)
    }
   
}



