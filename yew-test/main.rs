
use cell::Cellule;
use gloo_timers::callback::Interval;
use rand::Rng;
use yew::{classes, html, html::Scope, Component, Context, Html};

mod cell;

pub enum Msg {
    Tick,
    ToggleCellule(usize),
    Start,
    Stop,
    Step,
    Reset,
    Random,
}

pub struct Model {
    active: bool,
    cellules: Vec<Cellule>,
    cellules_width: usize,
    cellules_height: usize,
    _interval: Interval,
}

impl Model {
    pub fn random_mutate(&mut self) {
        for cellule in self.cellules.iter_mut() {
            if rand::thread_rng().gen() {
                cellule.set_alive();
            } else {
                cellule.set_dead();
            }
        }
    }

    fn reset(&mut self) {
        for cellule in self.cellules.iter_mut() {
            cellule.set_dead();
        }
    }

    fn step(&mut self) {
        let mut to_dead = Vec::new();
        let mut to_live = Vec::new();
        for row in 0..self.cellules_height {
            for col in 0..self.cellules_width {
                let neighbors = self.neighbors(row as isize, col as isize);

                let current_idx = self.row_col_as_idx(row as isize, col as isize);
                if self.cellules[current_idx].is_alive() {
                    if Cellule::alone(&neighbors) || Cellule::overpopulated(&neighbors) {
                        to_dead.push(current_idx);
                    }
                } else if Cellule::can_be_revived(&neighbors) {
                    to_live.push(current_idx);
                }
            }
        }
        to_dead.iter()
            .for_each(|idx| self.cellules[*idx].set_dead());
        to_live.iter()
        .for_each(|idx| self.cellules[*idx].set_alive());
    }

    fn neighbors(&self, row: isize, col: isize) -> [Cellule; 8] {
        [
            self.cellules[self.row_col_as_idx(row + 1, col + 0)],
            self.cellules[self.row_col_as_idx(row + 1, col + 1)],
            self.cellules[self.row_col_as_idx(row + 1, col - 1)],
            self.cellules[self.row_col_as_idx(row - 1, col + 0)],
            self.cellules[self.row_col_as_idx(row - 1, col + 1)],
            self.cellules[self.row_col_as_idx(row - 1, col - 1)],
            self.cellules[self.row_col_as_idx(row + 0, col - 1)],
            self.cellules[self.row_col_as_idx(row + 0, col - 1)],
        ]
    }

    fn row_col_as_idx(&self, row: isize, col: isize) -> usize {
        let row = wrap(row, self.cellules_height as isize);
        let col = wrap(col, self.cellules_width as isize);

        row * self.cellules_width + col
    }

    fn view_cellule(&self, idx: usize, cellule: &Cellule, link: &Scope<Self>) -> Html {
        let cellule_status = {
            if cellule.is_alive() {
                "cellule-live"
            } else {
                "cellule-dead"
            }
        };
        html! {
            <div key={idx} class={classes!("game-cellule", cellule_status) }
                onclick={link.callback(move |_| Msg::ToggleCellule(idx))}>
            </div>
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(200, move || callback.emit(()));

        let (cellules_width, cellules_height) = (53, 40);

        Self {
            active: false,
            cellules: vec![Cellule::new_dead(); cellules_width * cellules_height],
            cellules_width, cellules_height, _interval: interval
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                if self.active {
                    self.step();
                    true
                } else {
                    false
                }
            },
            Msg::ToggleCellule(idx) => {
                let cellule = self.cellules.get_mut(idx).unwrap();
                cellule.toggle();
                true
            },
            Msg::Start => {
                self.active = true;
                log::info!("Start");
                false
            },
            Msg::Stop => {
                self.active = false;
                log::info!("Stop");
                false
            },
            Msg::Step => {
                self.step();
                true
            },
            Msg::Reset => {
                self.reset();
                log::info!("Reset");
                true
            },
            Msg::Random => {
                self.random_mutate();
                log::info!("Random");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cell_rows = self.cellules.chunks(self.cellules_width)
        .enumerate()
        .map(|(y, cellules)| {
            let idx_offset = y * self.cellules_width;
            let cells = cellules
                .iter()
                .enumerate()
                .map(|(x, cell)| self.view_cellule(idx_offset + x, cell, ctx.link()));
            
            html! {
                <div key={y} class="game-row">
                    {for cells}
                </div>
            }
        });

        html! {
            <div>
                <section class="game-container">
                    <header class="app-header">
                        <img alt="The app logo" src="favicon.ico" class="app-logo" />
                        <h1 class="app-title">{ "Game of Life"}</h1>
                    </header>

                    <section class="game-area">
                        <div class="game-of-life">
                            { for cell_rows }
                        </div>

                        <div class="game-buttons">
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Start)}>{ "Start" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Start)}>{ "Stop" }</button>
                        </div>
                    </section>
                </section>

                <footer class="app-footer">
                    <strong class="footer-text">
                        { "Game of life - a yaw experiment" }
                    </strong>
                </footer>

            </div>
        }
    }
}

fn wrap(coord: isize, range: isize) -> usize {
    let result = if coord < 0 {
        coord + range
    } else if coord >= range {
        coord - range
    } else {
        coord
    };
    result as usize
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::start_app::<Model>();
}
