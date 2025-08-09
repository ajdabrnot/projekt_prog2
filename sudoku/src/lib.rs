//pub mod grafika;

mod testi;

use sauron::html::text;
use sauron::prelude::*;
use sauron::{node, Cmd, Component, Node, Program};

pub mod logika;
pub mod strukture;

use crate::logika::pojavitve_stevila;
use crate::strukture::{Polje, Suduku};

pub enum Msg {
    // Stevka(usize),
    // Vrstica(usize),
    // Stolpec(usize),
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

fn ali_je_vpisana_stvar_stevilo() -> () {}

impl Application for App {
    type MSG = Msg;

    // fn update(&mut self, msg: Msg) -> Cmd<Msg> {
    //     match msg {
    //         // Msg::Vrstica(a) if a > 0 && a < 10 => self.vrstica = a,
    //         // Msg::Stolpec(a) if a > 0 && a < 10 => self.stolpec = a,
    //         // Msg::Stevka(a) if a > 0 && a < 10 => self.stevilo = a,
    //         Msg::Polje(r, c, a)
    //         //prva opcija: vse je okej
    //             if preveri_indeks(r, c)
    //                 && a > 0
    //                 && a < 10
    //                 && self
    //                     .mreza
    //                     .ali_je_veljavno(r as u8 + 1, c as u8 + 1, a as u8) =>
    //         {
    //             self.mreza.mreza[izracunaj_indeks(r, c)].stevilo = a as u8;
    //             self.vrstica = r + 1;
    //             self.stolpec = c + 1;
    //             self.stevilo = a;

    //             self.mreza.napolni_polje(
    //                 self.vrstica as u8,
    //                 self.stolpec as u8,
    //                 self.stevilo as u8,
    //             );
    //         }
    //         Msg::Polje(r, c, a)
    //         // druga opcija: prevelika številka
    //             if preveri_indeks(r, c)
    //                 && self
    //                     .mreza
    //                     .ali_je_veljavno(r as u8 + 1, c as u8 + 1, a as u8) =>
    //         {
    //             self.mreza.mreza[izracunaj_indeks(r, c)].stevilo = a as u8;
    //             self.vrstica = r + 1;
    //             self.stolpec = c + 1;
    //             self.stevilo = a;

    //             self.mreza.napolni_polje(
    //                 self.vrstica as u8,
    //                 self.stolpec as u8,
    //                 self.stevilo as u8,
    //             );
    //         }
    //         Msg::Polje(r, c, a)=>
    //         //tretja opcija: številka se že nahaja v vrstici/stolpcu/škatli, glede velikosti mi je vseeno
    //         {
    //             self.mreza.mreza[izracunaj_indeks(r, c)].stevilo = a as u8;
    //             self.vrstica = r + 1;
    //             self.stolpec = c + 1;
    //             self.stevilo = a;

    //             self.mreza.napolni_polje(
    //                 self.vrstica as u8,
    //                 self.stolpec as u8,
    //                 self.stevilo as u8,
    //             );
    //         }
    //         // Msg::Vrstica(_) => self.vrstica = 0,
    //         // Msg::Stolpec(_) => self.stolpec = 0,
    //         // Msg::Stevka(_) => self.stevilo = 0,
    //         //Msg::Polje(_, _, _) => {
    //         //    self.mreza.mreza[izracunaj_indeks(self.vrstica, self.stolpec)].stevilo = 0;
    //         //    self.veljavnost = true
    //         //}
    //     };
    //     self.mreza
    //         .napolni_polje(self.vrstica as u8, self.stolpec as u8, self.stevilo as u8);
    //     return Cmd::none();
    // }

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            Msg::Polje(r, c, a) => {
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
        }; // do tu je številka vpisana v sudoku.

        // tuki je treba nrdit še, da pogleda ali je zdaj enolična rešitev in da sporoči, če je

        return Cmd::none();
    }

    fn view(&self) -> Node<Msg> {
        div(
            [],
            [
                h1([], [text!("**SUDOKU**")]),
                //p([], [izpisi_vrstice(&self.mreza)]),
                div([], [idk9krat(&self)]),
                div([], [izpisi_vse_vrstice_polj(&self.mreza)]),
                //div([], [idk9krat1(&self)]),
            ],
        )
    }
}

fn izpisi_vrstice(sudoku: &Suduku) -> Node<Msg> {
    //izpiše suduku kot [0,0,0,0,...], [0,0,0,0,...]
    let mut sez: std::vec::Vec<sauron::Node<Msg>> = vec![text!("sudoku je",)];
    for i in 0..9 {
        sez.push(br([], []));
        sez.push(text!("{:?}", sudoku.sudoku_kot_seznam_samo_vrednosti()[i]));
    }
    div([], sez)
}

fn izpisi_eno_vrstico_polj(sudoku: &Suduku, vrstica: usize) -> Node<Msg> {
    //izpiše eno vrstiso sudokuja, ampak ne kot seznam, vsaka številka pozna svojo vrstico, stolpec, škatlo
    let mut sez: std::vec::Vec<sauron::Node<Msg>> = vec![];
    for i in 0..9 {
        sez.push(td(
            [r#class("celica"), r#id(ustvari_id(vrstica, i))],
            [div(
                [r#class("celica")],
                [text!(
                    "{:?}",
                    sudoku.sudoku_kot_seznam_samo_vrednosti()[vrstica][i]
                )],
            )],
        ));
    }
    div([], [tr([], sez)])
}

fn izpisi_vse_vrstice_polj(sudoku: &Suduku) -> Node<Msg> {
    //kot vrstica polj amapk napiše vseh 9
    let mut sez: std::vec::Vec<sauron::Node<Msg>> = vec![];
    for i in 0..9 {
        sez.push(izpisi_eno_vrstico_polj(sudoku, i))
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
                on_input(move |event: InputEvent| {
                    Msg::Polje(vrstica, i, event.value().parse().unwrap())
                }),
            ],
            [],
        ));
    }
    div([], sez)
}

fn ustvari_id(vrstica: usize, stolpec: usize) -> String {
    //naredi string ki je uporabljen kot id za celice sudukuja
    let id = format!("r{}c{}", vrstica.to_string(), stolpec.to_string());
    return id;
}

fn idk9krat(sudoku: &App) -> Node<Msg> {
    //izpiše suduku kot number inpute, če je številka prevelika jo obarva rdeče
    let mut sez: std::vec::Vec<sauron::Node<Msg>> = vec![];
    let mut ponovitve = vec![];
    for j in 0..9 {
        for i in 0..9 {
            ponovitve.extend(pojavitve_stevila(
                &sudoku.mreza,
                &sudoku.mreza.mreza[izracunaj_indeks(j, i)],
            ));
        }
    }

    for j in 0..9 {
        for i in 0..9 {
            //se mi zdi da 1. if stavek niveč potreben ker je niz itak dolžine 1
            if sudoku.mreza.mreza[izracunaj_indeks(j, i)].stevilo < 10 {
                if ponovitve.contains(&(j + 1, i + 1)) {
                    sez = izmisli_si_ime(sez, "ponovitev".to_string(), sudoku, j, i);
                } else {
                    sez = izmisli_si_ime(sez, "dobro".to_string(), sudoku, j, i);
                }
            } else {
                sez = izmisli_si_ime(sez, "slabo".to_string(), sudoku, j, i);
            }

            //sez.push(input(
            //    [
            //        r#min(1),
            //        r#max(9),
            //        r#type("number"),
            //        r#placeholder(sudoku.mreza.mreza[izracunaj_indeks(j, i)].stevilo),
            //        //r#id(ustvari_id(j, i)),
            //        //r#class("celica1"),
            //        on_input(move |event: InputEvent| {
            //            Msg::Polje(j, i, event.value().parse().unwrap())
            //        }),
            //    ],
            //    [],
            //));
        }
        sez.push(br([], []))
    }

    div([], sez)
}

fn izmisli_si_ime(
    mut sez: Vec<Node<Msg>>,
    razred: String,
    sudoku: &App,
    j: usize,
    i: usize,
) -> Vec<Node<Msg>> {
    sez.push(input(
        [
            r#min(1),
            r#max(9),
            r#type("text"),
            r#maxlength("1"),
            r#placeholder(sudoku.mreza.mreza[izracunaj_indeks(j, i)].stevilo),
            r#id(ustvari_id(j, i)),
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
    ));
    return sez;
}

//|event| vec!["0","1","2","3","4","5","6", "7", "8","9"].contains(event.key )

// fn idk9krat(sudoku: &Suduku) -> Node<Msg> {
//     let mut sez: Vec<Node<Msg>> = vec![];

//     for j in 0..9 {
//         for i in 0..9 {
//             let vrednost = sudoku.sudoku_kot_seznam_samo_vrednosti()[j][i];
//             let moznosti = sudoku.mreza[9 * j + i].moznosti.clone(); // Potreben za move v closure

//             sez.push(input(
//                 [
//                     r#min(1),
//                     r#max(9),
//                     r#type("number"),
//                     value(vrednost.to_string()),
//                     on_input(move |event: InputEvent| {
//                         // Poskusimo razparsati število
//                         if let Ok(vnos) = event.value().parse::<u8>() {
//                             // Če je med dovoljenimi možnostmi
//                             if moznosti.contains(&vnos) {
//                                 Msg::Polje(j, i, vnos as usize)
//                             } else {
//                                 Msg::Ignoriraj // Ali pa definiraš nekaj, kar ne spremeni stanja
//                             }
//                         } else {
//                             Msg::Ignoriraj // Če ni število (prazno ali invalid)
//                         }
//                     }),
//                 ],
//                 [],
//             ));
//         }
//         sez.push(br([], [])); // Nova vrstica
//     }

//     div([], sez)
// }

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

// wasm-pack build --target web --release
// basic-http-server -a 0.0.0.0:4000
// puscica gor dol za zgodovino v teminalu
