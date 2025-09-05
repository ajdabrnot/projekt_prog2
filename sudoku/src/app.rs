use sauron::html::text;
use sauron::prelude::*;
use sauron::{Cmd, Node};
use crate::pomozne_funkcije::*;
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
            barvne_sheme: vec!["crno_bela".to_string(), "oranzna_zelena_vijola".to_string(), "jacaster".to_string(), "bolece_oci".to_string(),  "taka_natural".to_string(), "pastelna_mesanca".to_string(), "pinky".to_string(),"roza".to_string(), "dark".to_string(),"dark_2".to_string(), "modra".to_string(), "zelena".to_string()],
            trenutna_barvna_shema: 0,
            trenutna_vrstica_cursor: 0,
            trenutni_stolpec_cursor: 0
        }
    }
    fn focus_na_polje(&self) {
        use wasm_bindgen::JsCast;
        use web_sys::{window, HtmlInputElement};
        
        let window = window().unwrap();
        let document = window.document().unwrap();
        let element_id = ustvari_id(self.trenutna_vrstica_cursor, self.trenutni_stolpec_cursor);
        
        if let Some(element) = document.get_element_by_id(&element_id) {
            if let Some(input_element) = element.query_selector("input").unwrap() {
                if let Ok(input) = input_element.dyn_into::<HtmlInputElement>() {
                    let _ = input.focus();
                }
            }
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

                self.trenutna_vrstica_cursor = r;
                self.trenutni_stolpec_cursor = c;

                self.stare_mreze.push(self.mreza.kopiraj_sudoku());
                self.trenutna_mreza = self.trenutna_mreza +1;
                
                self.mreza.napolni_polje(
                    self.vrstica as u8,
                    self.stolpec as u8,
                    self.stevilo as u8,
                );
                
            }
            Msg::Resi => {
                self.mreza.hitro_resi_sudoku();
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
            },
            Msg::Barve => {
                if self.trenutna_barvna_shema == self.barvne_sheme.len() -1 {
                    self.trenutna_barvna_shema = 0
                } else{
                    self.trenutna_barvna_shema += 1
                }
            },
            Msg::PremakniCursor(smer) => {
                match smer.as_str() {
                    "ArrowUp" => {
                        if self.trenutna_vrstica_cursor > 0 {
                            self.trenutna_vrstica_cursor -= 1;
                        }
                    }
                    "ArrowDown" => {
                        if self.trenutna_vrstica_cursor < 8 {
                            self.trenutna_vrstica_cursor += 1;
                        }
                    }
                    "ArrowLeft" => {
                        if self.trenutni_stolpec_cursor > 0 {
                            self.trenutni_stolpec_cursor -= 1;
                        }
                    }
                    "ArrowRight" => {
                        if self.trenutni_stolpec_cursor < 8 {
                            self.trenutni_stolpec_cursor += 1;
                        }
                    }
                    _ => {}
                }
                self.focus_na_polje();
            }

        };

        return Cmd::none();
    }


    
    fn view(&self) -> Node<Msg> {
    body(
        [
            r#class(&self.barvne_sheme[self.trenutna_barvna_shema]),
            on_keydown(|event: KeyboardEvent| {
                match event.key().as_str() {
                    "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight" => {
                        event.prevent_default();
                        Msg::PremakniCursor(event.key())
                    }
                    _ => Msg::PremakniCursor("".to_string())
                }
            }),
            r#tabindex(0), 
        ],
        [div(
            [r#class(&self.barvne_sheme[self.trenutna_barvna_shema])],
            [
                div([r#id("ogromen_pravokotnik")], [text("")]),
                table(
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
                                                            r#class("gumb_navodila"),
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
                                                                r#value("Cilj je sestaviti svojo nalogo sudoku. \n V levi sudoku vpisuj števke, za katere želiš, da so v sudokuju podane.\n Če bo ista številka v isti vrstici/stolpcu/škatli vpisana večkrat, se bodo polja obarvala rdeče.\n V spodnji vrstici program sproti opozarja, ali je sudoku enolično oziroma sploh\n rešljiv. Sklikom na gumb 'REŠI SUDOKU' se sudoku na desni reši. Ta gumb je omogočen, ko\n je sudoku enoličo rešljiv. S klikom na gumb 'SHRANI PDF' se sestavljeni\n sudoku in rešitve shranijo v pdf obliki. \nZabavaj se! \np.s. Ne bodi dolgočasna. Obarvaj si življanje!"),
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
                                    [
                                        input([
                                            r#type("button"),
                                            r#class("gumb_naprej_nazaj"),
                                            r#value("<--"),
                                            on_click(|_| Msg::KorakNazaj)
                                        ], []),
                                        input([
                                            r#type("button"),
                                            r#class("gumb_naprej_nazaj"),
                                            r#value("ZAČNI ZNOVA"),
                                            on_click(|_| Msg::ZacniZnova)
                                        ], []),
                                        input([
                                            r#type("button"),
                                            r#class("gumb_naprej_nazaj"),
                                            r#value("-->"),
                                            on_click(|_| Msg::KorakNaprej)
                                        ], [])
                                    ],
                                ),
                                td(
                                    [r#class("osnovna_tabela")],
                                    [
                                        input(
                                            [
                                                r#type("button"),
                                                r#class("resi_shrani_barve"),
                                                r#disabled(!(
                                                    self.mreza.ali_je_resljiv_hitro() && self.mreza.je_enolicno_resljivo_hitra()
                                                )),
                                                r#value("REŠI SUDOKU"),
                                                on_click(|_| Msg::Resi),
                                            ],
                                            [],
                                        ),
                                        input([
                                            r#class("resi_shrani_barve"),
                                            r#type("button"),
                                            r#value("SHRANI PDF"),
                                            on_click(|_| Msg::ShraniPdf)
                                        ], []),
                                        input(
                                            [
                                                r#class("resi_shrani_barve"),
                                                r#type("button"),
                                                r#value("BARVE"),
                                                on_click(|_| Msg::Barve)
                                            ],
                                            []
                                        )
                                    ],
                                ),
                            ],
                        ),
                        tr(
                            [],
                            [td(
                                [r#colspan("2")],
                                [div(
                                    [r#id("sporocilo_resljivosti")],
                                    [text(ali_je_sploh_oz_enolicno_resljiv(&self))]
                                )],
                            )],
                        ),
                    ],
                ),
            ]
        )],
    )
}
}
