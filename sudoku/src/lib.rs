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
    Polje(usize, usize, usize),
    Resi
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

impl Application for App {
    type MSG = Msg;

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
            Msg::Resi=> self.mreza.resi_sudoku(),
        }; // do tu je številka vpisana v sudoku.

        // tuki je treba nrdit še, da pogleda ali je zdaj enolična rešitev in da sporoči, če je.

        return Cmd::none();
    }

    fn view(&self) -> Node<Msg> {
        div(
            [],
            [
                //div(
                //    [r#id("overlay")],
                //    [div([r#id("text")], [text("Overlay text")])],
                //),
                table(
                    [],
                    [
                        thead(
                            [],
                            [tr(
                                [],
                                [th([r#colspan("2")], [h1([], [text("**SUDOKU**")])])],
                            )],
                        ),
                        tbody(
                            [],
                            [tr(
                                [],
                                [td([r#colspan("2")], [div(
                                [],
                                [
                                    div([class("myDIV")], [text("NAVODILA")]),
                                    div(
                                        [class("hide")],
                                        [text(
                                            "Vpiši številke! Zabavaj se! kako lep je svet sudokuja! noro! obožujem sudoku!!!! wawwww za kosilo sem jedla palačinke! :)",
                                        )],
                                    ),
                                ],
                            )])],
                            ),tr(
                                [],
                                [
                                    td([r#class("osnovna_tabela")], [sudoku_inputi(&self)]),
                                    td(
                                        [r#class("osnovna_tabela")],
                                        [izpisi_vse_vrstice_polj(&self.mreza)],
                                    ),
                                ],
                            )],
                        ),
                        tr(
                            [],
                            [
                                td(
                                    [r#class("osnovna_tabela")],
                                    [input(
                                        [r#type("button"), r#id("natisni"), r#value("NATISNI")],
                                        [],
                                    )],
                                ),
                                td(
                                    [r#class("osnovna_tabela")],
                                    [input(
                                        [r#type("button"), r#id("resi"), r#value("REŠI SUDOKU"), on_click(|_|{ Msg::Resi})],
                                        [],
                                    )],
                                ),
                            ],
                        ),
                        
                    ],
                ),
            ],
        )
    }
}

fn izpisi_vrstice(sudoku: &Suduku) -> Node<Msg> {
    //neuporabljena, čaka na izbris
    //izpiše suduku kot [0,0,0,0,...], [0,0,0,0,...]
    let mut sez: std::vec::Vec<sauron::Node<Msg>> = vec![text!("sudoku je",)];
    for i in 0..9 {
        sez.push(br([], []));
        sez.push(text!("{:?}", sudoku.sudoku_kot_seznam_samo_vrednosti()[i]));
    }
    div([], sez)
}

// te funkcije tuki bi se loh dale v posebi datoteko?? da ni tuki tok natlačen??
fn izpisi_eno_vrstico_polj(sudoku: &Suduku, vrstica: usize) -> Node<Msg> {
    //izpiše eno vrstico sudokuja po poljih
    let mut sez: std::vec::Vec<sauron::Node<Msg>> = vec![];
    for i in 0..9 {
        sez.push(td(
            [r#class("celica"), r#id(ustvari_id(vrstica, i))],
            [div(
                [],
                [text(sudoku.sudoku_kot_seznam_samo_vrednosti()[vrstica][i])],
            )],
        ));
    }
    div([], [tr([], sez)])
}

fn izpisi_vse_vrstice_polj(sudoku: &Suduku) -> Node<Msg> {
    //izpiše cel sudoku po poljih
    let mut sez: std::vec::Vec<sauron::Node<Msg>> = vec![];
    for i in 0..9 {
        sez.push(tr([], [izpisi_eno_vrstico_polj(sudoku, i)]))
    }
    div([], sez)
}
fn idk(sudoku: &Suduku, vrstica: usize) -> Node<Msg> {
    //sploh ne vem kaj to nrdi ampak ni nikoli uporabljena --> izbris
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
    //naredi string, ki je uporabljen kot id za celice sudukuja
    let id = format!("r{}c{}", vrstica.to_string(), stolpec.to_string());
    return id;
}

fn sudoku_inputi(sudoku: &App) -> Node<Msg> {
    //izpiše suduku kot tabelo input polj, če se dve številki ponovita, ju obarva.
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

fn ustvari_input_polje(
    mut sez: Vec<Node<Msg>>,
    razred: String,
    sudoku: &App,
    j: usize,
    i: usize,
) -> Vec<Node<Msg>> {
    //ustvari j-i-to input polje. trenutno dovoli vpis črk, a se te ne vpišejo v dejanski sudoku
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
                        //a se da to lepše preverit ne pa vsakiš to_string()???
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

//ahah je mnogo zakomentirane kode, ki si jo želim izbrisat.
fn ahah() -> () {
    // fn sudoku_inputi(sudoku: &Suduku) -> Node<Msg> {
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
    // }}
}

fn izpisi_sudoku_po_poljih(sudoku: &Suduku) -> Node<Msg> {
    //idk kaj je fora, nikoli ni rabljeno
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
}

//Če ne dela 0.0.0.0 spremeni v localhost

//zakaj dovoli dve isti števili v enem stolpcu???
//x-wing preveri
//škatle v debele obrobe css

// wasm-pack build --target web --release
// basic-http-server -a 0.0.0.0:4000
// puscica gor dol za zgodovino v teminalu
