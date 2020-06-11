#![recursion_limit = "512"]
use std::time::Duration;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::{ConsoleService, IntervalService, Task};

use crate::State::Start;
use std::fmt;

#[derive(fmt::Debug, Copy, Clone, PartialEq)]
pub enum State {
    Empty,
    Start,
    Visited,
    Path,
    Target,
    Wall,
}

#[derive(Copy, Clone)]
pub enum Stage {
    Init,
    StartSet,
    TargetSet,
    Started,
    Paused,
    Done,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Empty => write!(f, "."),
            State::Visited => write!(f, "_"),
            State::Start => write!(f, "O"),
            State::Path => write!(f, "+"),
            State::Target => write!(f, "x"),
            State::Wall => write!(f, "|"),
        }
    }
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stage::Init => write!(f, "Init"),
            Stage::StartSet => write!(f, "StartSet"),
            Stage::TargetSet => write!(f, "TargetSet"),
            Stage::Started => write!(f, "Started"),
            Stage::Paused => write!(f, "Paused"),
            Stage::Done => write!(f, "Done"),
        }
    }
}

fn state_class(state: State) -> String {
    match state {
        State::Path => String::from("path"),
        State::Empty => String::from("empty"),
        State::Start => String::from("start"),
        State::Wall => String::from("wall"),
        State::Visited => String::from("visited"),
        State::Target => String::from("target"),
    }
}

#[derive(fmt::Debug, Clone)]
pub struct Node {
    x: usize,
    y: usize,
    active: bool,
    state: State,
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
    Reset,
    Start,

    Hover(usize, usize),
    Down(usize, usize),
    Up(usize, usize),
}

impl Component for Grid {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Msg::Next);
        let mut interval = IntervalService::new();
        let handle = interval.spawn(Duration::from_millis(50), callback);

        let mut m = new_matrix(20, 40);

        let g = Self {
            stage: Stage::Init,
            matrix: m,
            link,
            steps: 0,
            start: None,
            target: None,
            down: false,
            job: Box::new(handle), // enable interval
        };

        g
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let mut console = ConsoleService::new();
        console.log(format!("{}", self.matrix[10][10].state).as_ref());
        match msg {
            Msg::Next => match self.stage {
                Stage::Started => {
                    if step(&mut self.matrix) {
                        self.steps += 1;
                    } else {
                        console.log("done");
                        self.stage = Stage::Done;
                    }
                    true
                }
                _ => false,
            },
            Msg::Reset => {
                self.stage = Stage::Init;
                self.matrix = new_matrix(20, 40);

                true
            }
            Msg::Hover(i, j) => {
                if !self.down {
                    return false;
                }
                self.activate(i, j)
            }
            Msg::Down(i, j) => {
                self.down = true;
                self.activate(i, j)
            }
            Msg::Up(_, _) => {
                self.down = false;
                match self.stage {
                    Stage::Init => {
                        if let Some(_) = self.start {
                            self.stage = Stage::StartSet;
                            true
                        } else {
                            false
                        }
                    }
                    Stage::StartSet => {
                        if let Some(_) = self.target {
                            self.stage = Stage::TargetSet;
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            }
            Msg::Start => match self.stage {
                Stage::Paused | Stage::TargetSet => {
                    self.stage = Stage::Started;
                    true
                }
                Stage::Started => {
                    self.stage = Stage::Paused;
                    true
                }
                _ => false,
            },
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
            <button onclick=self.link.callback(|_| Msg::Reset)>{"Reset"}</button>
            <button onclick=self.link.callback(|_| Msg::Start)>{"Start/Pause"}</button>
            <br />
            <div class="help">{ self.help() }</div>
            <div>{ self.stage }{ self.steps }</div>
            <div class="board disable-select">
            {
                for self.matrix.iter().enumerate().map(|(i, mut row)| {
                    html! {
                        <div class="row">
                        {
                            for row.iter().enumerate().map(|(j, n)| {
                                html! {
                                    <span
                                        class=("cell", state_class(n.state))
                                        onmouseover=self.link.callback(move |_| Msg::Hover(i, j))
                                        onmousedown=self.link.callback(move |_| Msg::Down(i, j))
                                        onmouseup=self.link.callback(move |_| Msg::Up(i, j))
                                    >
                                        {'\u{00a0}'}
                                    </span>
                                }
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

impl Grid {
    fn help(&self) -> Html {
        html! {
            <>
            {"Next step:"}
            {'\u{00a0}'}
            {
                match self.stage {
                    Stage::Init => html! {<>{"mark"}<div class="cell start" />{"start"}</>},
                    Stage::StartSet => html! {<>{"mark"}<div class="cell target" />{"target"}</>},
                    Stage::TargetSet =>html! {<>{"mark"}<div class="cell wall" />{"wall"}</>},
                    Stage::Started => html! {"solving..."},
                    Stage::Paused => html! {"paused"},
                    Stage::Done => html! {"shortest path found!"},
                }
            }
            </>
        }
    }

    // mouse down, either initially or on reaching a new cell
    fn activate(&mut self, i: usize, j: usize) -> bool {
        match self.stage {
            Stage::Init => {
                if let Some((x, y)) = self.start {
                    self.matrix[x][y].state = State::Empty;
                };

                self.matrix[i][j].set_start();
                self.start = Some((i, j));
                // self.stage = Stage::StartSet;
                true
            }
            Stage::StartSet => match self.matrix[i][j].state {
                State::Start => false,
                _ => {
                    if let Some((x, y)) = self.target {
                        self.matrix[x][y].state = State::Empty;
                    };

                    self.matrix[i][j].set_target();
                    self.target = Some((i, j));

                    true
                }
            },
            Stage::TargetSet => match self.matrix[i][j].state {
                State::Start | State::Target => false,
                _ => {
                    self.matrix[i][j].state = State::Wall;
                    true
                }
            },
            Stage::Started => false,
            Stage::Paused => false,
            Stage::Done => {
                if self.matrix[i][j].state != State::Target {
                    false
                } else {
                    true
                }
                // TODO: move target and solve
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Grid>::new().mount_to_body();
}

pub type Matrix = Vec<Vec<Node>>;

fn new_matrix(height: usize, width: usize) -> Matrix {
    let mut g = vec![];
    for i in 0..height {
        let mut row: Vec<Node> = vec![];
        for j in 0..width {
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
            if nb_i < self.height() && nb_j < self.width() {
                nb.push(self[nb_i][nb_j].clone());
            }
        }

        nb
    }

    fn height(&self) -> usize {
        self.len()
    }

    fn width(&self) -> usize {
        self[0].len()
    }
}

pub struct Grid {
    link: ComponentLink<Self>,
    matrix: Matrix,

    // state
    steps: i64,
    start: Option<(usize, usize)>,
    target: Option<(usize, usize)>,
    stage: Stage,
    down: bool,

    // Worker
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
                    let _end = mark_path(m, n);
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
