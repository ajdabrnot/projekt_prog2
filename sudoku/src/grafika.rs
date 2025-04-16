use sauron::html::text;
use sauron::prelude::*;
use sauron::{node, Cmd, Component, Node, Program};
#[derive(Debug)]
pub enum Msg {
    Increment,
    Decrement,
}
pub struct App {
    count: i32,
}
impl App {
    pub fn new() -> Self {
        App { count: 0 }
    }
}
impl Component<Msg> for App {
    fn view(&self) -> Node<Msg> {
        node! {
            <main>
                <input type="button"
                    value="+"
                    key=1
                    on_click={|_| {
                        Msg::Increment
                    }}
                />
                <div>{text(self.count)}</div>
                <input type="button"
                    value="-"
                    key=1
                    on_click={|_| {
                        Msg::Decrement
                    }}
                />
            </main>
        }
    }
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
        }
        Cmd::none()
    }
}
#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
}
