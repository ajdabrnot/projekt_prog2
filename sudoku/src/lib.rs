//pub mod grafika;

mod testi;

use sauron::html::text;
use sauron::prelude::*;
use sauron::{node, Cmd, Component, Node, Program};

pub mod logika;
pub mod strukture;

use crate::strukture::{Polje, Suduku};

pub enum Msg {
    Stevka(usize),
    Vrstica(usize),
    Stolpec(usize),
    Polje(usize, usize, usize),
}

pub struct App {
    stevilo: usize,
    vrstica: usize,
    stolpec: usize,
    mreza: Suduku,
}

impl App {
    pub fn new() -> App {
        App {
            stevilo: 0,
            vrstica: 0,
            stolpec: 0,
            mreza: Suduku::prazen_suduku(),
        }
    }
}
// ctrl + k + c ti zakomentira vse kar oznacis
// ctr + k ctrl u odkomentira
// ctrl k s bliznice
// ctrl c prekine terminal
fn izracunaj_indeks(vrst: usize, stolp: usize) -> usize {
    (vrst as usize) * 9 + (stolp as usize)
}

fn preveri_indeks(vrst: usize, stolp: usize) -> bool {
    return 0 <= izracunaj_indeks(vrst, stolp) && 81 > izracunaj_indeks(vrst, stolp);
}

impl Application for App {
    type MSG = Msg;

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            // Msg::Stevka(1) => self.stevilo = 1,
            // Msg::Stevka(2) => self.stevilo = 2,
            // Msg::Stevka(3) => self.stevilo = 3,
            // Msg::Stevka(4) => self.stevilo = 4,
            // Msg::Stevka(5) => self.stevilo = 5,
            // Msg::Stevka(6) => self.stevilo = 6,
            // Msg::Stevka(7) => self.stevilo = 7,
            // Msg::Stevka(8) => self.stevilo = 8,
            // Msg::Stevka(9) => self.stevilo = 9,
            // Msg::Stevka(_) => self.stevilo = 0,
            // //
            // Msg::Vrstica(1) => self.vrstica = 1,
            // Msg::Vrstica(2) => self.vrstica = 2,
            // Msg::Vrstica(3) => self.vrstica = 3,
            // Msg::Vrstica(4) => self.vrstica = 4,
            // Msg::Vrstica(5) => self.vrstica = 5,
            // Msg::Vrstica(6) => self.vrstica = 6,
            // Msg::Vrstica(7) => self.vrstica = 7,
            // Msg::Vrstica(8) => self.vrstica = 8,
            // Msg::Vrstica(9) => self.vrstica = 9,
            // Msg::Vrstica(_) => self.vrstica = 0,
            // //
            // Msg::Stolpec(1) => self.stolpec = 1,
            // Msg::Stolpec(2) => self.stolpec = 2,
            // Msg::Stolpec(3) => self.stolpec = 3,
            // Msg::Stolpec(4) => self.stolpec = 4,
            // Msg::Stolpec(5) => self.stolpec = 5,
            // Msg::Stolpec(6) => self.stolpec = 6,
            // Msg::Stolpec(7) => self.stolpec = 7,
            // Msg::Stolpec(8) => self.stolpec = 8,
            // Msg::Stolpec(9) => self.stolpec = 9,
            // Msg::Stolpec(_) => self.stolpec = 0,
            Msg::Vrstica(a) if a > 0 && a < 10 => self.vrstica = a,
            Msg::Stolpec(a) if a > 0 && a < 10 => self.stolpec = a,
            Msg::Stevka(a) if a > 0 && a < 10 => self.stevilo = a,
            Msg::Polje(r, c, a) if preveri_indeks(r, c) && a > 0 && a < 10 => {
                self.mreza.mreza[izracunaj_indeks(r, c)].stevilo = a as u8;
                self.vrstica = r + 1;
                self.stolpec = c + 1;
                self.stevilo = a;
                self.mreza.napolni_polje(
                    self.vrstica as u8,
                    self.stolpec as u8,
                    self.stevilo as u8,
                );
            }
            Msg::Vrstica(_) => self.vrstica = 0,
            Msg::Stolpec(_) => self.stolpec = 0,
            Msg::Stevka(_) => self.stevilo = 0,
            Msg::Polje(_, _, _) => {
                self.mreza.mreza[izracunaj_indeks(self.vrstica, self.stolpec)].stevilo = 0
            }
        };
        self.mreza
            .napolni_polje(self.vrstica as u8, self.stolpec as u8, self.stevilo as u8);
        return Cmd::none();
    }

    fn view(&self) -> Node<Msg> {
        div(
            [],
            [
                p([], [text!("Števka:")]),
                input(
                    [
                        r#min(1),
                        r#max(9),
                        r#type("number"),
                        on_input(|event: InputEvent| Msg::Stevka(event.value().parse().unwrap())),
                    ],
                    [],
                ),
                p([], [text!("Vrstica:")]),
                input(
                    [
                        r#min(1),
                        r#max(9),
                        r#type("number"),
                        on_input(|event: InputEvent| Msg::Vrstica(event.value().parse().unwrap())),
                    ],
                    [],
                ),
                p([], [text!("Stolpec:")]),
                input(
                    [
                        r#min(1),
                        r#max(9),
                        r#type("number"),
                        on_input(|event: InputEvent| Msg::Stolpec(event.value().parse().unwrap())),
                    ],
                    [],
                ),
                p(
                    [],
                    [text!(
                        "V polje v R{}C{} si napisal število {}.",
                        self.vrstica,
                        self.stolpec,
                        self.stevilo,
                    )],
                ),
                p(
                    [],
                    [
                        text!("sudoku je",),
                        br([], []),
                        text!("{:?}", self.mreza.sudoku_kot_seznam_samo_vrednosti()[0]),
                        br([], []),
                        text!("{:?}", self.mreza.sudoku_kot_seznam_samo_vrednosti()[1]),
                        br([], []),
                        text!("{:?}", self.mreza.sudoku_kot_seznam_samo_vrednosti()[2]),
                        br([], []),
                        text!("{:?}", self.mreza.sudoku_kot_seznam_samo_vrednosti()[3]),
                        br([], []),
                        text!("{:?}", self.mreza.sudoku_kot_seznam_samo_vrednosti()[4]),
                        br([], []),
                        text!("{:?}", self.mreza.sudoku_kot_seznam_samo_vrednosti()[5]),
                        br([], []),
                        text!("{:?}", self.mreza.sudoku_kot_seznam_samo_vrednosti()[6]),
                        br([], []),
                        text!("{:?}", self.mreza.sudoku_kot_seznam_samo_vrednosti()[7]),
                        br([], []),
                        text!("{:?}", self.mreza.sudoku_kot_seznam_samo_vrednosti()[8]),
                        br([], []),
                        //izpisi_sudoku_po_poljih(&self.mreza),
                        idk(&self.mreza, 1),
                        izpisi_eno_vrstico_polj(&self.mreza, 1),
                        izpisi_vrstico(&self.mreza),
                        idk9krat(&self.mreza),
                    ],
                ),
            ],
        )
    }
}

fn izpisi_vrstico(sudoku: &Suduku) -> Node<Msg> {
    let mut sez: std::vec::Vec<sauron::Node<Msg>> = vec![text!("sudoku je",)];
    for i in 0..9 {
        sez.push(br([], []));
        sez.push(text!("{:?}", sudoku.sudoku_kot_seznam_samo_vrednosti()[i]));
    }
    div([], sez)
}

fn izpisi_eno_vrstico_polj(sudoku: &Suduku, vrstica: usize) -> Node<Msg> {
    let mut sez: std::vec::Vec<sauron::Node<Msg>> = vec![];
    for i in 0..9 {
        sez.push(text!(
            "{:?}",
            sudoku.sudoku_kot_seznam_samo_vrednosti()[vrstica][i]
        ));
    }
    div([], sez)
}

fn idk(sudoku: &Suduku, vrstica: usize) -> Node<Msg> {
    let mut sez: std::vec::Vec<sauron::Node<Msg>> = vec![];
    for i in 0..9 {
        sez.push(input(
            [
                r#min(1),
                r#max(9),
                r#type("number"),
                value(sudoku.sudoku_kot_seznam_samo_vrednosti()[vrstica][i]),
                //on_input(|event: InputEvent| Msg::Stevka(event.value().parse().unwrap())),
                on_input(move |event: InputEvent| {
                    Msg::Polje(vrstica, i, event.value().parse().unwrap())
                }),
            ],
            [],
        ));
    }
    div([], sez)
}

fn idk9krat(sudoku: &Suduku) -> Node<Msg> {
    let mut sez: std::vec::Vec<sauron::Node<Msg>> = vec![];
    for j in 0..9 {
        for i in 0..9 {
            sez.push(input(
                [
                    r#min(1),
                    r#max(9),
                    r#type("number"),
                    value(sudoku.sudoku_kot_seznam_samo_vrednosti()[j][i]),
                    //on_input(|event: InputEvent| Msg::Stevka(event.value().parse().unwrap())),
                    on_input(move |event: InputEvent| {
                        Msg::Polje(j, i, event.value().parse().unwrap())
                    }),
                ],
                [],
            ));
        }
        sez.push(br([], []))
    }
    div([], sez)
}

fn izpisi_sudoku_po_poljih(sudoku: &Suduku) -> Node<Msg> {
    let mut sez: std::vec::Vec<sauron::Node<Msg>> = vec![text!("sudoku je",)];
    for i in 0..9 {
        sez.push(br([], []));
        sez.push(text!("{:?}", izpisi_eno_vrstico_polj(sudoku, i)));
    }
    div([], sez)
}
//vpisi st naj ima kar div notri z devetimi inputi al neki

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
    // let mut sudoku_prvi = Suduku::prazen_suduku();
    // println!("{:?}", sudoku_prvi.mreza);
    // sudoku_prvi.napolni_polje(9, 9, 1);
    // println!("TO BO NOVA MREZAAAAAA");
    // println!("{:?}", sudoku_prvi.mreza);
    // sudoku_prvi.napolni_polje(9, 8, 1);
    // println!("TO BO NOVA MREZAAAAAA");
    // println!("{:?}", sudoku_prvi.mreza);
    // let seznam_vrednosti = sudoku_prvi.sudoku_kot_seznam_samo_vrednosti();
    // println!("{:?}", seznam_vrednosti)
}

//Če ne dela 0.0.0.0 spremeni v localhost

//zakaj dovoli dve isti števili v enem stolpcu???
//x-wing preveri
//škatle v debele obrobe css
