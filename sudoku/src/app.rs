use sauron::html::text;
use sauron::prelude::*;
use sauron::{Cmd, Node};
use crate::logika::*;
use crate::strukture::{App, Msg, Suduku};
use crate::sauron_funkcije::*;




impl App {
    pub fn new() -> App {
        App {
            stevilo: 0,
            vrstica: 0,
            stolpec: 0,
            mreza: Suduku::prazen_suduku(),
            prikaz_navodil: "nevidna".to_string(),
            stare_mreze: vec![Suduku::prazen_suduku()],
            trenutna_mreza: 0,
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

                self.stare_mreze.push(self.mreza.kopiraj_sudoku());
                self.trenutna_mreza = self.trenutna_mreza +1;
                
                //if self.stare_mreze.len() > 6 { //si zapomne do 5 korakov nazaj
                //    self.stare_mreze.remove(0);
                //}
                self.mreza.napolni_polje(
                    self.vrstica as u8,
                    self.stolpec as u8,
                    self.stevilo as u8,
                );
                
            }
            //Msg::Resi => self.mreza.resi_sudoku(),
            Msg::Resi => {
                self.mreza.hitro_resi_sudoku_1();
                self.stare_mreze.push(self.mreza.kopiraj_sudoku());
                self.trenutna_mreza += 1
            },
            Msg::NavodilaOn => izpise_navodila(self, true),
            Msg::NavodilaOff => izpise_navodila(self, false),
            Msg::KorakNazaj => {
                if self.trenutna_mreza > 0 {
                    self.mreza = self.stare_mreze[self.trenutna_mreza -1].clone();
                    self.trenutna_mreza -= 1;
                }else{}
                
                
            },
            Msg::KorakNaprej =>{
                if self.trenutna_mreza < self.stare_mreze.len() - 1{
                    self.mreza = self.stare_mreze[self.trenutna_mreza +1].clone();
                    self.trenutna_mreza += 1;
                }else {}
                
            },
            Msg::ZacniZnova => {
                self.mreza = Suduku::prazen_suduku();
                self.stare_mreze = vec![ Suduku::prazen_suduku()];
                self.trenutna_mreza = 0
            },
            

            Msg::ShraniPdf => {
                poklici_shrani_pdf();
            }

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
                                                            r#value("V levi sudoku vpisuj števke, za katere želiš, da so v sudokuju podane.\n V spodnji vrstici program sproti opozarja,\n ali je sudoku enolično oziroma sploh rešljiv.\n S klikom na gumb 'NATISNI' se sestavljeni sudoku shrani v pdf obliki.\n Sklikom na gumb 'REŠI' se sudoku na desni reši.\n Ta gumb je omogočen, ko je sudoku enoličo rešljiv.\n Zabavaj se!"),
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
                                [input([r#type("button"),r#class("gumb_naprej_nazaj"),r#value("<--"), on_click(|_| Msg::KorakNazaj)], []),input([r#type("button"),r#class("gumb_naprej_nazaj"),r#value("ZAČNI ZNOVA"), on_click(|_| Msg::ZacniZnova)], []),input([r#type("button"),r#class("gumb_naprej_nazaj"),r#value("-->"), on_click(|_| Msg::KorakNaprej)], [])//input(
                                    //[r#type("button"), r#id("natisni"), r#value("NATISNI")],
                                    //[],
                                //)
                                ],
                            ),
                            td(
                                [r#class("osnovna_tabela")],
                                [//input([r#type("button"),r#value("<--"), on_click(|_| Msg::KorakNazaj)], []),
                                input(
                                    [
                                        r#type("button"),
                                        r#id("resi"),
                                        r#disabled(!(self.mreza.ali_je_resljiv_hitro() && self.mreza.je_enolicno_resljivo_hitra())),
                                        r#value("REŠI SUDOKU"),
                                        on_click(|_| Msg::Resi),
                                    ],
                                    [],
                                ),
                                //input([r#type("button"),r#value("-->"), on_click(|_| Msg::KorakNaprej)], [])
                                ],
                            ),
                        ],
                    ),
                    tr([],[td([r#colspan("2")],[div([r#id("sporocilo_resljivosti")],[text(ali_je_sploh_oz_enolicno_resljiv(&self))]),
                       ])]
                    ),

                    //tole zdej je novo!!
                    button(
                        [on_click(|_| Msg::ShraniPdf)],
                        [text("Shrani kot PDF")],
                    ),
                ],
            )],
        )
    }
}
