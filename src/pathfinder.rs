use crate::{
    matrix::{Matrix, Solve},
    node::{state_class, State},
};
use gloo_timers::callback::Interval;
use yew::{classes, html, html::Scope, Component, Context, Html};

pub enum Msg {
    NextTick,
    Reset,
    Start,

    MouseHover(usize, usize),
    MouseDown(usize, usize),
    MouseUp,
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

pub struct Pathfinder {
    matrix: Matrix,

    // state
    steps: i64,
    start: Option<(usize, usize)>,
    target: Option<(usize, usize)>,
    stage: Stage,

    // flags
    down: bool,
    currently_moving: Option<State>,
}

impl Component for Pathfinder {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();

        let interval = Interval::new(100, move || {
            link.send_message(Msg::NextTick);
        });
        interval.forget();

        let (h, w) = get_grid_dimensions();

        Self {
            start: None,
            target: None,
            stage: Stage::Init,
            matrix: Matrix::new(h, w),
            steps: 0,
            down: false,
            currently_moving: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NextTick => match self.stage {
                Stage::Started => {
                    match self.matrix.step() {
                        Solve::Continue => self.steps += 1,
                        Solve::Stop => self.stage = Stage::Done,
                    }
                    true
                }
                _ => false,
            },
            Msg::Reset => {
                self.stage = Stage::Init;
                let (h, w) = get_grid_dimensions();
                self.matrix = Matrix::new(h, w);

                true
            }
            Msg::MouseHover(i, j) => {
                if !self.down {
                    false
                } else {
                    self.activate(i, j)
                }
            }
            Msg::MouseDown(i, j) => {
                self.down = true;
                self.activate(i, j)
            }
            Msg::MouseUp => {
                self.down = false;
                self.currently_moving = None;
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
                    Stage::Done => false,
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <>
            { self.menu(link) }
            { self.grid(link) }
            { self.help(link) }
            </>
        }
    }
}

fn solving_class(stage: Stage) -> Option<String> {
    match stage {
        Stage::Started => Some(String::from("solving")),
        _ => None,
    }
}

impl Pathfinder {
    fn grid(&self, link: &Scope<Pathfinder>) -> Html {
        html! {
            <div
                class={classes!("board", "disable-select", solving_class(self.stage))}
                onmouseup={link.callback(move |_| Msg::MouseUp)}
            > {
                for self.matrix.clone().into_iter().enumerate().map(|(i, row)| {
                    html! {
                        <div class="row">
                        {
                            for row.iter().enumerate().map(|(j, n)| {
                                html! {
                                    <span
                                        class={classes!("cell", state_class(n.state), if n.active { "active" } else {""})}
                                        onmouseover={link.callback(move |_| Msg::MouseHover(i, j))}
                                        onmousedown={link.callback(move |_| Msg::MouseDown(i, j))}
                                    >
                                        {'\u{00a0}'}
                                    </span>
                                }
                            })
                        }
                        </div>
                    }
                })
            } </div>
        }
    }

    fn menu(&self, link: &Scope<Pathfinder>) -> Html {
        html! {
            <div class="main menu">
                <button onclick={link.callback(|_| Msg::Reset)}>{"Reset"}</button>
                {
                    match self.stage {
                        Stage::TargetSet | Stage::Paused => html! {
                            <button onclick={link.callback(|_| Msg::Start)}>
                            {"Start"}
                            </button>
                        },
                        Stage::Started => html! {
                            <button onclick={link.callback(|_| Msg::Start)}>
                            {"Pause"}
                            </button>
                        },
                        _ => html!{},
                    }
                }
            </div>
        }
    }

    fn help(&self, link: &Scope<Pathfinder>) -> Html {
        html! {
            <div class="help menu">
            {
                match self.stage {
                    Stage::Init => html! {<>{"Place"}<div class="cell start" />{" start node"}</>},
                    Stage::StartSet => html! {<>{"Place"}<div class="cell target" />{" target node"}</>},
                    Stage::TargetSet =>html! {
                        <>
                        {"Place"}
                        <div class="cell wall" />
                        {"walls, then"}
                        <button onclick={link.callback(|_| Msg::Start)}>
                            {"Start"}
                        </button>
                        </>
                    },
                    Stage::Started => html! {"Solving..."},
                    Stage::Paused => html! {"Paused"},
                    Stage::Done => html! {
                        <>
                        {"Move"}
                        <div class="cell start" />
                        {"/"}
                        <div class="cell target" />
                        {"/"}
                        <div class="cell wall" />
                        {"around!"}
                        </>
                    },
                }
            }
            </div>
        }
    }
}

impl Pathfinder {
    pub fn set_start(&mut self, i: usize, j: usize) {
        if let Some((i, j)) = self.start {
            self.matrix.set_inactive(i, j);
            self.matrix.set_state(i, j, State::Empty);
        };

        self.start = Some((i, j));
        self.matrix.set_state(i, j, State::Start);
        self.matrix.set_active(i, j);
    }

    fn set_target(&mut self, i: usize, j: usize) {
        if let Some((i, j)) = self.target {
            self.matrix.set_state(i, j, State::Empty);
        };

        self.target = Some((i, j));
        self.matrix.set_state(i, j, State::Target);
    }

    // mouse down, either initially or on reaching a new cell
    fn activate(&mut self, i: usize, j: usize) -> bool {
        match self.stage {
            Stage::Init => {
                self.set_start(i, j);
                true
            }
            Stage::StartSet => match self.matrix.state(i, j) {
                State::Start => false,
                _ => {
                    self.set_target(i, j);
                    true
                }
            },
            Stage::TargetSet => self.change_cell(i, j),
            Stage::Started => false,
            Stage::Paused => false,
            Stage::Done => {
                let should_move = self.change_cell(i, j);
                if should_move {
                    self.update_solution()
                }
                should_move
            }
        }
    }

    // should we change a cell from one state to another?
    fn change_cell(&mut self, i: usize, j: usize) -> bool {
        match self.matrix.state(i, j) {
            State::Target | State::Start => {
                if self.currently_moving.is_none() {
                    self.currently_moving = Some(self.matrix.state(i, j));
                }
                false
            }
            State::Wall => match self.currently_moving {
                None => {
                    self.currently_moving = Some(State::Empty);
                    self.matrix.set_state(i, j, State::Empty);
                    true
                }
                Some(State::Empty) => {
                    self.matrix.set_state(i, j, State::Empty);
                    true
                }
                Some(_) => false,
            },
            _ => match self.currently_moving {
                None => {
                    self.currently_moving = Some(State::Wall);
                    self.matrix.set_state(i, j, State::Wall);
                    true
                }
                Some(state) => match state {
                    State::Start => {
                        self.set_start(i, j);
                        true
                    }
                    State::Target => {
                        self.set_target(i, j);
                        true
                    }
                    State::Wall => {
                        self.matrix.set_state(i, j, State::Wall);
                        true
                    }
                    _ => {
                        self.matrix.set_state(i, j, State::Empty);
                        true
                    }
                },
            },
        }
    }

    // clears "visited", "path"
    // keeps "start", "target", "wall"
    fn clear(&mut self) {
        for i in 0..self.matrix.height() {
            for j in 0..self.matrix.width() {
                self.matrix.set_inactive(i, j);
                self.matrix.clear_parent(i, j);
                match self.matrix.state(i, j) {
                    State::Start => self.matrix.set_active(i, j),
                    State::Target | State::Wall | State::Empty => {}
                    _ => self.matrix.set_state(i, j, State::Empty),
                }
            }
        }
    }

    fn solve(&mut self) {
        loop {
            match self.matrix.step() {
                Solve::Continue => continue,
                Solve::Stop => break,
            }
        }
    }

    fn update_solution(&mut self) {
        self.clear();
        self.solve();
    }
}

const CELL_WIDTH: usize = 30;
const CELL_HEIGHT: usize = 30;

use gloo_utils::window;

fn get_grid_dimensions() -> (usize, usize) {
    // let mut console = ConsoleService::new();

    let height = window().inner_height().unwrap().as_f64().unwrap() as usize;
    let width = window().inner_width().unwrap().as_f64().unwrap() as usize;

    let h = (height / CELL_HEIGHT) - 4;
    let w = (width / CELL_WIDTH) - 2;

    // console.log(format!("wdh:{} wdw:{} h:{} w:{}", wd.height, wd.width, h, w).as_ref());
    (h as usize, w as usize)
}
