use crate::strukture::*;
use sauron::html::text;
use sauron::prelude::*;
use sauron::{node, Cmd, Component, Node, Program};
use crate::logika::*;
use crate::strukture::{App, Msg, Polje, Suduku};
use crate::sauron_funkcije::*;




impl App {
    pub fn new() -> App {
        App {
            stevilo: 0,
            vrstica: 0,
            stolpec: 0,
            mreza: Suduku::prazen_suduku(),
            prikaz_navodil: "nevidna".to_string(),
        }
    }
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
                // do tu je številka vpisana v sudoku.

                // tuki je treba nrdit še, da pogleda ali je zdaj enolična rešitev in da sporoči, če je.
            }
            Msg::Resi => self.mreza.resi_sudoku(),
            Msg::NavodilaOn => izpise_navodila(self, true),
            Msg::NavodilaOff => izpise_navodila(self, false),
            
            
        };

        return Cmd::none();
    }

    fn view(&self) -> Node<Msg> {
        div(
            [],
            [table(
                [],
                [
                    thead(
                        [],
                        [tr(
                            [],
                            [th(
                                [r#colspan("2")],
                                [div([r#class("naslov")], [h1([], [text("**SUDOKU**")])])],
                            )],
                        )],
                    ),
                    tbody(
                        [],
                        [
                            tr(
                                [],
                                [td(
                                    [r#colspan("2")],
                                    [div(
                                        [],
                                        [
                                            div(
                                                [],
                                                [input(
                                                    [
                                                        r#id("gumb_navodila"),
                                                        r#type("button"),
                                                        r#value("NAVODILA"),
                                                        on_click(|_| Msg::NavodilaOn),
                                                    ],
                                                    [],
                                                )],
                                            ),
                                            div(
                                                [r#id(&self.prikaz_navodil)],
                                                [div(
                                                    [r#id("text")],
                                                    [input(
                                                        [
                                                            r#id("izpisana_navodila"),
                                                            r#type("button"),
                                                            r#value("V levi sudoku vpisuj števke, za katere želiš, da so v sudokuju podane.\n Ko jih bo vpisanih dovolj, da je sudoku enolično rešljiv, te bo program na to opozoril.\n S klikom na gumb 'NATISNI' se sestavljeni sudoku shrani v pdf obliki.\n Sudoku bo izgledal kot je prikazan sudoku na desni.\n Sklikom na gumb 'REŠI' se sudoku na desni reši."),
                                                            on_click(|_| Msg::NavodilaOff),
                                                        ],
                                                        [],
                                                    )],
                                                )],
                                            ), 
                                        ],
                                    )],
                                )],
                            ),
                            tr(
                                [],
                                [
                                    td([r#class("osnovna_tabela")], [sudoku_inputi(&self)]),
                                    td(
                                        [r#class("osnovna_tabela")],
                                        [izpisi_vse_vrstice_polj(&self.mreza)],
                                    ),
                                ],
                            ),
                        ],
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
                                    [
                                        r#type("button"),
                                        r#id("resi"),
                                        r#value("REŠI SUDOKU"),
                                        on_click(|_| Msg::Resi),
                                    ],
                                    [],
                                )],
                            ),
                        ],
                    ),
                    tr([],[td([r#colspan("2")],[div([r#id("sporocilo_resljivosti")],[text(ali_je_enolicno_resljiv(&self))]),
                       ])])
                ],
            )],
        )
    }
}
