use sauron::{html::text, prelude::*, Application, Cmd, Node, Program};

struct Container {
    nodes: Vec<u32>,
    last: String,
}
enum Msg {
    InsertNode,
    AddNode,
    ClickNode(u32),
}

impl Application<Msg> for Container {
    fn view(&self) -> Node<Msg> {
        let buttons = self
            .nodes
            .iter()
            .map(|x| *x)
            .map(move |x| button([key(x), on_click(move |_| Msg::ClickNode(x))], [text(x)]))
            .collect::<Vec<_>>();
        div(
            [],
            [
                text(&self.last),
                button([on_click(|_| Msg::InsertNode)], [text("add item (broken)")]),
                button([on_click(|_| Msg::AddNode)], [text("add item (working)")]),
                div([id("buttons")], buttons),
            ],
        )
    }
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::InsertNode => self.nodes.insert(0, self.nodes.len() as u32),
            Msg::AddNode => self.nodes.push(self.nodes.len() as u32),
            Msg::ClickNode(pos) => self.last = pos.to_string(),
        }
        Cmd::none()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();

    Program::mount_to_body(Container {
        nodes: vec![],
        last: String::default(),
    });
}
