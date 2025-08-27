use crate::logika::*;
use crate::strukture::{App, Msg, Suduku};
use sauron::html::text;
use sauron::prelude::*;
use sauron::Node;
use wasm_bindgen::JsCast;
use web_sys::window;

pub fn poklici_shrani_pdf() {
    let window = window().unwrap();
    let func = js_sys::Reflect::get(&window, &JsValue::from_str("shraniSudokuKotPDF")).unwrap();

    let func = func.dyn_into::<js_sys::Function>().unwrap();
    func.call0(&JsValue::NULL).unwrap();
}

pub fn izpise_navodila(app: &mut App, p_n: bool) -> () {
    if p_n {
        app.prikaz_navodil = "vidna".to_string()
    } else {
        app.prikaz_navodil = "nevidna".to_string()
    }
}
//
//pub fn ali_je_enolicno_resljiv1(app: &App) -> &str {
//    if app.mreza.je_enolicno_resljivo() {
//        "Sudoku JE enolično rešljiv :D"
//    } else {
//        "Sudoku NI enolično rešljiv :("
//    }
//}
pub fn ali_je_sploh_oz_enolicno_resljiv(app: &App) -> &str {
    //if app.mreza.ali_je_sudoku_resljiv() {
    if app.mreza.ali_je_resljiv_hitro() {
        //if app.mreza.je_enolicno_resljivo_pocasi() {
        if app.mreza.je_enolicno_resljivo_hitra() {
            "Sudoku JE enolično rešljiv :D"
        } else {
            "Sudoku NI enolično rešljiv :("
        }
    } else {
        "Sudoku NI več rešljiv. >:| Izbriši kakšno od vpisanih števil in poskusi znova! "
    }
}

//pub fn ali_je_sploh_resljiv(app: &App) -> &str {
//    if app.mreza.ali_je_sudoku_resljiv() {
//        "Obstaja vsaj ena rešitev za sudoku :D"
//    } else {
//        "Sudoku NI več rešljiv. Izbriši kakšno od vpisanih števil in poskusi s kakšno drugo! :("
//    }
//}

pub fn izpisi_eno_vrstico_polj(sudoku: &Suduku, vrstica: usize) -> Node<Msg> {
    //izpiše eno vrstico sudokuja po poljih
    let mut sez: std::vec::Vec<sauron::Node<Msg>> = vec![];
    for i in 0..9 {
        sez.push(td(
            [r#class("celica"), r#id(ustvari_id(vrstica, i))],
            [div(
                [r#class("celica_1")],
                [text(izpisi_stevilo_polja(sudoku, vrstica, i))],
            )],
        ));
    }
    div([], [tr([], sez)])
}

pub fn izpisi_stevilo_polja(sudoku: &Suduku, vrstica: usize, i: usize) -> &str {
    let st = sudoku.sudoku_kot_seznam_samo_vrednosti()[vrstica][i];
    match st {
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        _ => "",
    }
}

pub fn izpisi_vse_vrstice_polj(sudoku: &Suduku) -> Node<Msg> {
    //izpiše cel sudoku po poljih
    let mut sez: std::vec::Vec<sauron::Node<Msg>> = vec![];
    for i in 0..9 {
        sez.push(tr([], [izpisi_eno_vrstico_polj(sudoku, i)]))
    }
    div([], sez)
}

pub fn ustvari_id(vrstica: usize, stolpec: usize) -> String {
    //naredi string, ki je uporabljen kot id za celice sudukuja
    let id = format!("r{}c{}", vrstica.to_string(), stolpec.to_string());
    return id;
}

pub fn sudoku_inputi(sudoku: &App) -> Node<Msg> {
    //izpiše suduku kot tabelo input polj, če se dve številki ponovita, ju obarva.
    let mut sez: std::vec::Vec<sauron::Node<Msg>> = vec![];
    let mut ponovitve = vec![];
    for j in 0..9 {
        for i in 0..9 {
            ponovitve.extend(ponovitve_stevila(
                &sudoku.mreza,
                &sudoku.mreza.mreza[izracunaj_indeks(j, i)],
            ));
        }
    }
    for j in 0..9 {
        let mut sez1 = vec![];
        for i in 0..9 {
            if ponovitve.contains(&(j + 1, i + 1)) {
                sez1 = ustvari_input_polje(sez1, "ponovitev".to_string(), sudoku, j, i);
            } else {
                sez1 = ustvari_input_polje(sez1, "dobro".to_string(), sudoku, j, i);
            }
        }
        sez.push(tr([], sez1));
    }
    div([], sez)
}

pub fn ustvari_input_polje(
    mut sez: Vec<Node<Msg>>,
    razred: String,
    sudoku: &App,
    j: usize,
    i: usize,
) -> Vec<Node<Msg>> {
    //ustvari j-i-to input polje. dovoli vpis črk, a se te ne vpišejo v dejanski sudoku
    sez.push(td(
        [r#id(ustvari_id(j, i))],
        [input(
            [
                r#min(1),
                r#max(9),
                r#type("text"),
                r#maxlength("1"),
                //r#placeholder(sudoku.mreza.mreza[izracunaj_indeks(j, i)].stevilo),
                r#class(razred),
                on_input(move |event: InputEvent| {
                    if vec![
                        "0".to_string(),
                        "1".to_string(),
                        "2".to_string(),
                        "3".to_string(),
                        "4".to_string(),
                        "5".to_string(),
                        "6".to_string(),
                        "7".to_string(),
                        "8".to_string(),
                        "9".to_string(),
                        "Backspace".to_string(),
                    ]
                    .contains(&event.value())
                    {
                        Msg::Polje(j, i, event.value().parse().unwrap())
                    } else {
                        Msg::Polje(j, i, 0)
                    }
                }),
            ],
            [],
        )],
    ));
    return sez;
}
