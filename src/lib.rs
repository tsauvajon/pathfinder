#![recursion_limit = "512"]
use std::time::Duration;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::{ConsoleService, IntervalService, Task};

use std::fmt;

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

fn state_class(state: State) -> String {
    match state {
        State::Path => String::from("path"),
        State::Empty => String::from("empty"),
        State::Start => String::from("start"),
        State::Visited => String::from("visited"),
        State::Target => String::from("target"),
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

pub enum Msg {
    Next,
    Random,
    // Select(usize, usize),
}

impl Component for Grid {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Msg::Next);
        let mut interval = IntervalService::new();
        let handle = interval.spawn(Duration::from_millis(200), callback);

        let mut m = new_matrix(20, 20);

        m[3][3].set_start();
        m[14][14].set_target();

        let g = Self {
            done: false,
            matrix: m,
            link,
            steps: 0,
            job: Box::new(handle), // enable interval
        };

        g
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let mut console = ConsoleService::new();
        console.log(format!("{}", self.matrix[10][10].state).as_ref());
        match msg {
            Msg::Next => {
                if self.done {
                    return false;
                }
                if step(&mut self.matrix) {
                    self.steps += 1;
                    true
                } else {
                    console.log("done");
                    self.done = true;
                    true
                }
            }
            Msg::Random => {
                self.done = false;
                self.matrix = new_matrix(30, 30);
                self.matrix[23][28].set_start();
                self.matrix[1][9].set_target();

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
            <button onclick=self.link.callback(|_| Msg::Random)>{"Reset"}</button>
            // { self.value }
            <div class="board">
                {
                    for self.matrix.iter().map(|mut row| {
                        html! {
                            <div class="row">
                                {
                                    for row.iter().map(|n| {
                                        html! { <span class=("cell", state_class(n.state))>{'\u{00a0}'}</span> }
                                    })
                                }
                            </div>
                        }
                    })
                }
            </div>
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Grid>::new().mount_to_body();
}

///////////////

pub type Matrix = Vec<Vec<Node>>;

fn new_matrix(width: usize, height: usize) -> Matrix {
    let mut g = vec![];
    for i in 0..width {
        let mut row: Vec<Node> = vec![];
        for j in 0..height {
            row.push(Node::new(i, j));
        }
        g.push(row);
    }
    g
}
pub trait MatrixMethods {
    fn neighbours(&self, i: usize, j: usize) -> Vec<Node>;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

impl MatrixMethods for Matrix {
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
}

pub struct Grid {
    matrix: Matrix,
    link: ComponentLink<Self>,
    steps: i64,
    done: bool,
    job: Box<Task>,
}

fn step(m: &mut Matrix) -> bool {
    for (i, r) in m.clone().iter().enumerate() {
        for (j, n) in r.iter().enumerate() {
            if !n.active {
                continue;
            };

            m[n.x][n.y].active = false;

            let nbs = m.neighbours(n.x, n.y);
            for nb in nbs {
                if nb.is_target() {
                    let end = mark_path(m, n);
                    // end.print();
                    return false;
                } else if nb.is_empty() {
                    m[nb.x][nb.y].visit(i, j);
                }
            }
        }
    }

    true
}

fn mark_path(m: &mut Matrix, n: &Node) {
    let mut x = n.x;
    let mut y = n.y;

    loop {
        match m[x][y].mark_path() {
            Some((nx, ny)) => {
                x = nx;
                y = ny;
            }
            None => break,
        }
    }
}
