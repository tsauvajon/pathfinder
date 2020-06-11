#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::{IntervalService, Task};

use std::fmt;
use std::time::Duration;

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
        let handle = interval.spawn(Duration::from_millis(100), callback);

        let g = Self {
            stage: Stage::Init,
            matrix: new_matrix(20, 40),
            link,
            steps: 0,
            start: None,
            target: None,
            down: false,
            moving_target: false,
            moving_start: false,
            job: Box::new(handle), // enable interval
        };

        g
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Next => match self.stage {
                Stage::Started => {
                    if step(&mut self.matrix) {
                        self.steps += 1;
                    } else {
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
                    Stage::StartSet => match self.target {
                        Some(_) => {
                            self.stage = Stage::TargetSet;
                            true
                        }
                        None => false,
                    },
                    Stage::Done => {
                        self.moving_start = false;
                        self.moving_target = false;
                        false
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
            {
                match self.stage {
                    Stage::TargetSet | Stage::Paused => html! {
                        <button onclick=self.link.callback(|_| Msg::Start)>
                        {"Start"}
                        </button>
                    },
                    Stage::Started => html! {
                        <button onclick=self.link.callback(|_| Msg::Start)>
                        {"Pause"}
                        </button>
                    },
                    _ => html!{},
                }
            }
            <br />
            <div class="help">{ self.help() }</div>
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
            {
                match self.stage {
                    Stage::Init => html! {<>{"Place"}<div class="cell start" />{"start"}</>},
                    Stage::StartSet => html! {<>{"Place"}<div class="cell target" />{"target"}</>},
                    Stage::TargetSet =>html! {<>{"Place"}<div class="cell wall" />{"walls and Start when ready"}</>},
                    Stage::Started => html! {"Solving..."},
                    Stage::Paused => html! {"Paused"},
                    Stage::Done => html! {
                        <>
                        {"Move"}
                        <div class="cell start" />
                        {"start or"}
                        <div class="cell target" />
                        {"target around!"}
                        </>
                    },
                }
            }
            </>
        }
    }

    pub fn set_start(&mut self, i: usize, j: usize) {
        if let Some((i, j)) = self.start {
            self.matrix[i][j].state = State::Empty;
        };

        self.start = Some((i, j));
        self.matrix[i][j].state = State::Start;
        self.matrix[i][j].active = true;
    }

    fn set_target(&mut self, i: usize, j: usize) {
        if let Some((i, j)) = self.target {
            self.matrix[i][j].state = State::Empty;
        };

        self.target = Some((i, j));
        self.matrix[i][j].state = State::Target;
    }

    // mouse down, either initially or on reaching a new cell
    fn activate(&mut self, i: usize, j: usize) -> bool {
        match self.stage {
            Stage::Init => {
                self.set_start(i, j);
                true
            }
            Stage::StartSet => match self.matrix[i][j].state {
                State::Start => false,
                _ => {
                    self.set_target(i, j);
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
                match self.matrix[i][j].state {
                    State::Target => {
                        if !self.moving_target && !self.moving_start {
                            self.moving_target = true;
                        }
                    }
                    State::Start => {
                        if !self.moving_target && !self.moving_start {
                            self.moving_start = true;
                        }
                    }
                    State::Wall => {}
                    _ => {
                        if self.moving_target {
                            self.set_target(i, j);
                            self.update_solution();
                            return true;
                        }

                        if self.moving_start {
                            self.set_start(i, j);
                            self.update_solution();
                            return true;
                        }
                    }
                }

                false
            }
        }
    }

    // clears "visited", "path"
    // keeps "start", "target", "wall"
    fn clear(&mut self) {
        for i in 0..self.matrix.height() {
            for j in 0..self.matrix.width() {
                self.matrix[i][j].active = false;
                match self.matrix[i][j].state {
                    State::Start => self.matrix[i][j].active = true,
                    State::Target | State::Wall | State::Empty => {}
                    _ => self.matrix[i][j].state = State::Empty,
                }
            }
        }
    }

    fn solve(&mut self) {
        loop {
            if !step(&mut self.matrix) {
                return;
            }
        }
    }

    fn update_solution(&mut self) {
        self.clear();
        self.solve();
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

    fn width(&self) -> usize {
        self[0].len()
    }

    fn height(&self) -> usize {
        self.len()
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

    // flags
    down: bool,
    moving_target: bool,
    moving_start: bool,

    // Worker
    job: Box<Task>,
}

fn step(m: &mut Matrix) -> bool {
    let mut active_nb = 0;

    for (i, r) in m.clone().iter().enumerate() {
        for (j, n) in r.iter().enumerate() {
            if !n.active {
                continue;
            };

            active_nb += 1;

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

    // stop if there's no solution
    return active_nb > 0;
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
