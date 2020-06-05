// mod node;

use std::time::Duration;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::{ConsoleService, IntervalService, Task};

// use node::{Cell, Node};

// extern crate rand;

// use rand::Rng;
use std::fmt;

use yew::prelude::*;

// fn main() {
//     let grid = Grid::create(15, 15);
//
//     let mut rng = rand::thread_rng();
//     for _ in 0..20 {
//         let mut g = grid.clone();
//
//         let sx: usize = rng.gen_range(0, g.width());
//         let sy: usize = rng.gen_range(0, g.height());
//         let tx: usize = rng.gen_range(0, g.width());
//         let ty: usize = rng.gen_range(0, g.height());
//
//         g[sx][sy].set_start();
//         g[tx][ty].set_target();
//         solve(g);
//     }
// }

#[derive(fmt::Debug, Copy, Clone, PartialEq)]
pub enum State {
    Empty,
    Start,
    Visited,
    Path,
    Target,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Empty => write!(f, "."),
            State::Visited => write!(f, "_"),
            State::Start => write!(f, "O"),
            State::Path => write!(f, "+"),
            State::Target => write!(f, "x"),
        }
    }
}

#[derive(fmt::Debug, Clone)]
pub struct Node {
    x: usize,
    y: usize,
    state: State,
    active: bool,
    parent_coords: Option<(usize, usize)>,
}

impl Node {
    fn new(x: usize, y: usize) -> Node {
        Node {
            x,
            y,
            state: State::Empty,
            active: false,
            parent_coords: None,
            // link: ComponentLink<Self>::new,
        }
    }

    fn is_empty(&self) -> bool {
        match self.state {
            State::Empty => true,
            _ => false,
        }
    }

    fn is_target(&self) -> bool {
        match self.state {
            State::Target => true,
            _ => false,
        }
    }

    fn is_visited(&self) -> bool {
        match self.state {
            State::Visited => true,
            _ => false,
        }
    }

    pub fn set_start(&mut self) {
        self.state = State::Start;
        self.active = true;
    }

    pub fn set_target(&mut self) {
        self.state = State::Target;
    }

    fn visit(&mut self, p_x: usize, p_y: usize) {
        match self.state {
            State::Empty => (),
            _ => return,
        }
        self.state = State::Visited;
        self.active = true;
        self.parent_coords = Some((p_x, p_y))
    }

    fn mark_path(&mut self) -> Option<(usize, usize)> {
        if self.is_visited() {
            self.state = State::Path;
            self.parent_coords
        } else {
            None
        }
    }
}

pub struct Cell {
    state: State,
    active: bool,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub state: State,
    pub active: bool,
}

pub enum CellMsg {
    Select,
}

impl Component for Cell {
    type Message = CellMsg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: props.state,
            active: props.active,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            CellMsg::Select => self.state = State::Visited,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        self.active == props.active && self.state == props.state
    }

    fn view(&self) -> Html {
        html! {
            <button onclick=self.link.callback(|_| CellMsg::Select)>
                { format!("{}", self.state) }
            </button>
        }
    }
}
// impl fmt::Display for Node {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    grid: Grid,
    job: Box<Task>,
}

enum Msg {
    Next,
    // Select(usize, usize),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut g = Grid::create(20, 20);

        let callback = link.callback(|_| Msg::Next);
        let mut interval = IntervalService::new();
        let handle = interval.spawn(Duration::from_millis(200), callback);

        g[3][3].set_start();
        g[14][14].set_target();
        Self {
            link,
            value: 0,
            grid: g,
            job: Box::new(handle), // enable interval
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let mut console = ConsoleService::new();
        console.log("hello");
        match msg {
            Msg::Next => {
                match step(self.grid.clone()) {
                    Some(next) => {
                        self.value += 1;
                        self.grid = next;
                        true
                    }
                    None => false, // TODO: stop callbacks
                }
            }
        }
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
                { self.value }
                { self.grid.render() }
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}

///////////////

pub type Grid = Vec<Vec<Node>>;

pub trait GridMethods {
    fn create(width: usize, height: usize) -> Self;
    fn print(&self);

    fn neighbours(&self, i: usize, j: usize) -> Vec<Node>;

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn render(&self) -> Html;
}

impl GridMethods for Grid {
    fn create(width: usize, height: usize) -> Grid {
        let mut matrix: Grid = vec![];
        for i in 0..width {
            let mut row: Vec<Node> = vec![];
            for j in 0..height {
                row.push(Node::new(i, j));
            }
            matrix.push(row);
        }
        matrix
    }

    fn print(&self) {
        print!("\x1B[2J");
        for r in self {
            for n in r {
                print!("({}) ", n.state);
            }
            println!();
        }
    }

    fn neighbours(&self, i: usize, j: usize) -> Vec<Node> {
        let mut nb: Vec<Node> = vec![];

        let possible_nb: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        for p in possible_nb {
            let nb_i = (i as isize + p.0) as usize;
            let nb_j = (j as isize + p.1) as usize;
            if (0..self.width() as usize).contains(&nb_i)
                && (0..self.height() as usize).contains(&nb_j)
            {
                nb.push(self[nb_i][nb_j].clone());
            }
        }

        nb
    }

    fn width(&self) -> usize {
        self.len()
    }

    fn height(&self) -> usize {
        self[0].len()
    }

    fn render(&self) -> Html {
        html! {
            // <div>{ "hello" }</div>
            for self.iter().map(|mut row| {
                html! {
                    <div>
                        {
                            for row.iter().map(|n| {
                            // "a"
                                html! { <Cell state=n.state active=n.active /> }
                                // n.render()
                            })
                        }
                    // <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                    </div>
                }
            })
        }
    }
}

/// update grid by one step
/// actually for now it's one loop not 1 step
pub fn step_mut(mut g: Grid) -> bool {
    match step(g.clone()) {
        Some(next) => {
            g = next;
            true
        }
        None => false,
    }
}

fn step(g: Grid) -> Option<Grid> {
    // todo: keep track of active nodes to skip lots of browsing
    // and also if active.len() == 0 that means there's no path
    let mut next = g.clone();
    for (i, r) in g.iter().enumerate() {
        for (j, n) in r.iter().enumerate() {
            if !n.active {
                continue;
            };

            next[n.x][n.y].active = false;

            let nbs = g.neighbours(n.x, n.y);
            for nb in nbs {
                if nb.is_target() {
                    let end = mark_path(next, n);
                    // end.print();
                    return None;
                } else if nb.is_empty() {
                    next[nb.x][nb.y].visit(i, j);
                }
            }
        }
    }

    Some(next)
}

fn mark_path(g: Grid, n: &Node) -> Grid {
    let mut next = g;
    let mut x = n.x;
    let mut y = n.y;

    loop {
        match next[x][y].mark_path() {
            Some((nx, ny)) => {
                x = nx;
                y = ny;
            }
            None => break,
        }
    }

    next
}

pub fn solve(g: Grid) {
    let mut g = g;
    loop {
        g = match step(g) {
            Some(next) => next,
            None => return,
        };
    }
}
