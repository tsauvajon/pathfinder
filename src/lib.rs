// extern crate console_error_panic_hook;

mod grid;

// use std::panic;

use rand::Rng;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use grid::{Grid, GridMethods};

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    grid: Grid,
}

enum Msg {
    AddOne,
    Select(usize, usize),
}



impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // console_error_panic_hook::set_once();

        let mut g= Grid::create(20,20);
        // let mut rng = rand::thread_rng();
        //
        // let sx: usize = rng.gen_range(0, g.width());
        // let sy: usize = rng.gen_range(0, g.height());
        // let tx: usize = rng.gen_range(0, g.width());
        // let ty: usize = rng.gen_range(0, g.height());

        g[3][3].set_start();
        g[14][14].set_target();
        Self {
            link,
            value: 0,
            grid: g,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::Select(x, y) => (),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
                {
                    for self.grid.iter().map(|mut row| {
                        html! {
                            <div>
                                {
                                    for row.iter().map(|node| {
                                        // html! {<button>{ "X" }</button>}
                                        "X"
                                    })
                                }
                            </div>
                        }
                        // item.props.value = format!("item-{}", item.props.value);
                        // item
                    })
                }

                // <button onclick=self.link.callback(|_| Msg:Select(4, 4))></button>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    // panic::set_hook(Box::new(console_error_panic_hook::hook));
    App::<Model>::new().mount_to_body();
}
