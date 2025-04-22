use sauron::html::text;
use sauron::prelude::*;
use sauron::{node, Cmd, Component, Node, Program};

pub enum Msg {
    Stevka(u32),
}

pub struct App {
    stevilo: u32,
}

impl App {
    pub fn new() -> App {
        App { stevilo: 0 }
    }
}

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
        }
    }

    fn view(&self) -> Node<Msg> {
        div(
            [class("some-class"), id("some-id"), attr("data-id", 1)],
            [
                div(
                    [],
                    [input(
                        [
                            r#type("text"),
                            on_input(|event: InputEvent| Msg::Stevka(event.value())),
                        ],
                        [],
                    )],
                ),
                p([]),
                div(
                    [],
                    [
                        p([], [text!("Tell us something about yourself:")]),
                        div(
                            [],
                            [textarea(
                                [
                                    rows(10),
                                    cols(80),
                                    on_input(|event: InputEvent| {
                                        Msg::ChangeBiography(event.value())
                                    }),
                                    placeholder("I'm a..."),
                                ],
                                [],
                            )],
                        ),
                        p([], [text!("{}", self.biography)]),
                    ],
                ),
                div(
                    [],
                    [
                        text("What are you thinking right now?"),
                        input(
                            [
                                r#type("text"),
                                on_change(|event: InputEvent| Msg::ChangeThought(event.value())),
                                placeholder("Elephants..."),
                            ],
                            [],
                        ),
                        if let Some(thought) = &self.thought {
                            text(format!("Hmmn {}... Interesting.", thought))
                        } else {
                            span([], [])
                        },
                    ],
                ),
            ],
        )
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
}
