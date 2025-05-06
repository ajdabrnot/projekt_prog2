use sauron::html::text;
use sauron::prelude::*;
use sauron::{node, Cmd, Component, Node, Program};

pub enum Msg {
    R1C1(u32),
    R1C2(u32),
    R1C3(u32),
    R1C4(u32),
    R1C5(u32),
    R1C6(u32),
    R1C7(u32),
    R1C8(u32),
    R1C9(u32),
    //
    R2C1(u32),
    R2C2(u32),
    R2C3(u32),
    R2C4(u32),
    R2C5(u32),
    R2C6(u32),
    R2C7(u32),
    R2C8(u32),
    R2C9(u32),
    //
    R3C1(u32),
    R3C2(u32),
    R3C3(u32),
    R3C4(u32),
    R3C5(u32),
    R3C6(u32),
    R3C7(u32),
    R3C8(u32),
    R3C9(u32),
    //
    R4C1(u32),
    R4C2(u32),
    R4C3(u32),
    R4C4(u32),
    R4C5(u32),
    R4C6(u32),
    R4C7(u32),
    R4C8(u32),
    R4C9(u32),
    //
    R5C1(u32),
    R5C2(u32),
    R5C3(u32),
    R5C4(u32),
    R5C5(u32),
    R5C6(u32),
    R5C7(u32),
    R5C8(u32),
    R5C9(u32),
    //
    R6C1(u32),
    R6C2(u32),
    R6C3(u32),
    R6C4(u32),
    R6C5(u32),
    R6C6(u32),
    R6C7(u32),
    R6C8(u32),
    R6C9(u32),
    //
    R7C1(u32),
    R7C2(u32),
    R7C3(u32),
    R7C4(u32),
    R7C5(u32),
    R7C6(u32),
    R7C7(u32),
    R7C8(u32),
    R7C9(u32),
    //
    R8C1(u32),
    R8C2(u32),
    R8C3(u32),
    R8C4(u32),
    R8C5(u32),
    R8C6(u32),
    R8C7(u32),
    R8C8(u32),
    R8C9(u32),
    //
    R9C1(u32),
    R9C2(u32),
    R9C3(u32),
    R9C4(u32),
    R9C5(u32),
    R9C6(u32),
    R9C7(u32),
    R9C8(u32),
    R9C9(u32),
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
            Msg::R1C1(a) => self.stevilo = a,
            Msg::R1C2(a) => self.stevilo = a,
            Msg::R1C3(a) => self.stevilo = a,
            Msg::R1C4(a) => self.stevilo = a,
            Msg::R1C5(a) => self.stevilo = a,
            Msg::R1C6(a) => self.stevilo = a,
            Msg::R1C7(a) => self.stevilo = a,
            Msg::R1C8(a) => self.stevilo = a,
            Msg::R1C9(a) => self.stevilo = a,

            Msg::R2C1(a) => self.stevilo = a,
            Msg::R2C2(a) => self.stevilo = a,
            Msg::R2C3(a) => self.stevilo = a,
            Msg::R2C4(a) => self.stevilo = a,
            Msg::R2C5(a) => self.stevilo = a,
            Msg::R2C6(a) => self.stevilo = a,
            Msg::R2C7(a) => self.stevilo = a,
            Msg::R2C8(a) => self.stevilo = a,
            Msg::R2C9(a) => self.stevilo = a,
            //
            Msg::R3C1(a) => self.stevilo = a,
            Msg::R3C2(a) => self.stevilo = a,
            Msg::R3C3(a) => self.stevilo = a,
            Msg::R3C4(a) => self.stevilo = a,
            Msg::R3C5(a) => self.stevilo = a,
            Msg::R3C6(a) => self.stevilo = a,
            Msg::R3C7(a) => self.stevilo = a,
            Msg::R3C8(a) => self.stevilo = a,
            Msg::R3C9(a) => self.stevilo = a,
            //
            Msg::R4C1(a) => self.stevilo = a,
            Msg::R4C2(a) => self.stevilo = a,
            Msg::R4C3(a) => self.stevilo = a,
            Msg::R4C4(a) => self.stevilo = a,
            Msg::R4C5(a) => self.stevilo = a,
            Msg::R4C6(a) => self.stevilo = a,
            Msg::R4C7(a) => self.stevilo = a,
            Msg::R4C8(a) => self.stevilo = a,
            Msg::R4C9(a) => self.stevilo = a,
            //
            Msg::R5C1(a) => self.stevilo = a,
            Msg::R5C2(a) => self.stevilo = a,
            Msg::R5C3(a) => self.stevilo = a,
            Msg::R5C4(a) => self.stevilo = a,
            Msg::R5C5(a) => self.stevilo = a,
            Msg::R5C6(a) => self.stevilo = a,
            Msg::R5C7(a) => self.stevilo = a,
            Msg::R5C8(a) => self.stevilo = a,
            Msg::R5C9(a) => self.stevilo = a,
            //
            Msg::R6C1(a) => self.stevilo = a,
            Msg::R6C2(a) => self.stevilo = a,
            Msg::R6C3(a) => self.stevilo = a,
            Msg::R6C4(a) => self.stevilo = a,
            Msg::R6C5(a) => self.stevilo = a,
            Msg::R6C6(a) => self.stevilo = a,
            Msg::R6C7(a) => self.stevilo = a,
            Msg::R6C8(a) => self.stevilo = a,
            Msg::R6C9(a) => self.stevilo = a,
            //
            Msg::R7C1(a) => self.stevilo = a,
            Msg::R7C2(a) => self.stevilo = a,
            Msg::R7C3(a) => self.stevilo = a,
            Msg::R7C4(a) => self.stevilo = a,
            Msg::R7C5(a) => self.stevilo = a,
            Msg::R7C6(a) => self.stevilo = a,
            Msg::R7C7(a) => self.stevilo = a,
            Msg::R7C8(a) => self.stevilo = a,
            Msg::R7C9(a) => self.stevilo = a,
            //
            Msg::R8C1(a) => self.stevilo = a,
            Msg::R8C2(a) => self.stevilo = a,
            Msg::R8C3(a) => self.stevilo = a,
            Msg::R8C4(a) => self.stevilo = a,
            Msg::R8C5(a) => self.stevilo = a,
            Msg::R8C6(a) => self.stevilo = a,
            Msg::R8C7(a) => self.stevilo = a,
            Msg::R8C8(a) => self.stevilo = a,
            Msg::R8C9(a) => self.stevilo = a,
            //
            Msg::R9C1(a) => self.stevilo = a,
            Msg::R9C2(a) => self.stevilo = a,
            Msg::R9C3(a) => self.stevilo = a,
            Msg::R9C4(a) => self.stevilo = a,
            Msg::R9C5(a) => self.stevilo = a,
            Msg::R9C6(a) => self.stevilo = a,
            Msg::R9C7(a) => self.stevilo = a,
            Msg::R9C8(a) => self.stevilo = a,
            Msg::R9C9(a) => self.stevilo = a,
        };
        return Cmd::none();
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
                            on_input(|event: InputEvent| {
                                Msg::Stevka(event.value().parse().unwrap())
                            }),
                        ],
                        [],
                    )],
                ),
                p([], []),
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
