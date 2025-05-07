use sauron::html::text;
use sauron::prelude::*;
use sauron::{node, Cmd, Component, Node, Program};

pub mod strukture;
pub mod logika;


use crate::strukture::{Polje, Suduku};

pub enum Msg {
    Stevka(u8),
    Vrstica(u8),
    Stolpec(u8),
}

pub struct App {
    stevilo: u8,
    vrstica: u8,
    stolpec: u8,
    mreza: Suduku
}


impl App {
    pub fn new() -> App {
        App { stevilo: 0, vrstica: 0, stolpec: 0, mreza: Suduku::prazen_suduku() }
    }
}
// ctrl + k + c ti zakomentira vse kar oznacis
// ctr + k ctrl u odkomentira
// ctrl k s bliznice
// ctrl c prekine terminal

impl Application for App {
    type MSG = Msg;

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            Msg::Stevka(1) => self.stevilo = 1,
            Msg::Stevka(2) => self.stevilo = 2,
            Msg::Stevka(3) => self.stevilo = 3,
            Msg::Stevka(4) => self.stevilo = 4,
            Msg::Stevka(5) => self.stevilo = 5,
            Msg::Stevka(6) => self.stevilo = 6,
            Msg::Stevka(7) => self.stevilo = 7,
            Msg::Stevka(8) => self.stevilo = 8,
            Msg::Stevka(9) => self.stevilo = 9,
            Msg::Stevka(_) => self.stevilo = 0,
            //
            Msg::Vrstica(1) => self.vrstica = 1,
            Msg::Vrstica(2) => self.vrstica = 2,
            Msg::Vrstica(3) => self.vrstica = 3,
            Msg::Vrstica(4) => self.vrstica = 4,
            Msg::Vrstica(5) => self.vrstica = 5,
            Msg::Vrstica(6) => self.vrstica = 6,
            Msg::Vrstica(7) => self.vrstica = 7,
            Msg::Vrstica(8) => self.vrstica = 8,
            Msg::Vrstica(9) => self.vrstica = 9,
            Msg::Vrstica(_) => self.vrstica = 0,
            //
            Msg::Stolpec(1) => self.stolpec = 1,
            Msg::Stolpec(2) => self.stolpec = 2,
            Msg::Stolpec(3) => self.stolpec = 3,
            Msg::Stolpec(4) => self.stolpec = 4,
            Msg::Stolpec(5) => self.stolpec = 5,
            Msg::Stolpec(6) => self.stolpec = 6,
            Msg::Stolpec(7) => self.stolpec = 7,
            Msg::Stolpec(8) => self.stolpec = 8,
            Msg::Stolpec(9) => self.stolpec = 9,
            Msg::Stolpec(_) => self.stolpec = 0,
        };
        //self.mreza.napolni_polje(self.vrstica, self.stolpec, self.stevilo);
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
                        "V polje v R{}C{} si napisal število {}, sudoku je",
                        self.vrstica,
                        self.stolpec,
                        self.stevilo
                        //self.mreza
                    )],
                ),
            ],
        )
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    // console_log::init_with_level(log::Level::Trace).unwrap();
    // console_error_panic_hook::set_once();
    // Program::mount_to_body(App::new());
    let mut sudoku_prvi = Suduku::prazen_suduku();
    println!("{:?}", sudoku_prvi.mreza);
    sudoku_prvi.napolni_polje(9, 9, 1);
    println!("TO BO NOVA MREZAAAAAA");
    println!("{:?}", sudoku_prvi.mreza);
    sudoku_prvi.napolni_polje(9, 8, 1);
    println!("TO BO NOVA MREZAAAAAA");
    println!("{:?}", sudoku_prvi.mreza);
    let seznam_vrednosti = sudoku_prvi.sudoku_kot_seznam_samo_vrednosti();
    println!("{:?}", seznam_vrednosti)


}
