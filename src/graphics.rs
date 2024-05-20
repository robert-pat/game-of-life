use crate::game::CellState;
use crate::{game, save_load, text, GAME_X, GAME_Y};
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use std::collections::VecDeque;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

// TODO: better name
struct GameTiming {
    last_step: std::time::Instant,
    step_length: std::time::Duration,
}
impl GameTiming {
    fn new(len: std::time::Duration, last: std::time::Instant) -> Self {
        GameTiming {
            last_step: last,
            step_length: len,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GUIGameAction {
    Step,
    Paused,
    Play,
    GrowCell,
    KillCell,
}
impl TryFrom<game::GameAction> for GUIGameAction {
    type Error = ();
    fn try_from(value: game::GameAction) -> Result<Self, Self::Error> {
        match value {
            game::GameAction::GrowCell => Ok(GUIGameAction::GrowCell),
            game::GameAction::KillCell => Ok(GUIGameAction::KillCell),
            game::GameAction::Paused => Ok(GUIGameAction::Paused),
            game::GameAction::Play => Ok(GUIGameAction::Play),
            game::GameAction::Step => Ok(GUIGameAction::Step),
            _ => Err(()),
        }
    }
}
pub(crate) struct GUIGameState {
    pub(crate) board: game::Game,
    current_action: Option<GUIGameAction>,
    size_of_cell: (u32, u32),
    timing: GameTiming,
}
impl GUIGameState {
    pub(crate) fn new(size: (usize, usize), pixels_per_cell: (u32, u32)) -> Self {
        GUIGameState {
            board: game::Game::new(size.0, size.1),
            current_action: None,
            size_of_cell: pixels_per_cell,
            timing: GameTiming::new(
                std::time::Duration::from_millis(200),
                std::time::Instant::now(),
            ),
        }
    }
    pub(crate) fn tick(&mut self) {
        self.board.step(1);
    }
    pub(crate) fn load_new_board(&mut self, new: game::Game) {
        self.board = new;
    }
    pub(crate) fn consume_current_event(&mut self) {
        if self.current_action.is_none() {
            return;
        }
        match self.current_action.unwrap() {
            GUIGameAction::Step => self.tick(),
            GUIGameAction::Paused => return,
            GUIGameAction::Play => {
                if self.timing.last_step.elapsed() >= self.timing.step_length {
                    self.tick();
                    self.timing.last_step = std::time::Instant::now();
                }
            }
            GUIGameAction::GrowCell => {
                // text::prompt_user_to_change_cells(&mut self.board, CellState::Alive)
                todo!("need to update text functions and/or remove functionality")
            }
            GUIGameAction::KillCell => {
                // text::prompt_user_to_change_cells(&mut self.board, CellState::Dead)
                todo!("need to update text functions and/or remove functionality")
            }
        }
        self.current_action = None;
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ProgramEvent {
    ShowHelp,
    SaveBoard,
    LoadBoard,
    ExitApplication,
}
struct ProgramManager {
    to_process: VecDeque<ProgramEvent>,
}
impl ProgramManager {
    fn new() -> Self {
        ProgramManager {
            to_process: VecDeque::new(),
        }
    }
    fn add_event(&mut self, event: ProgramEvent) -> Result<(), ()> {
        if let Some(e) = self.to_process.back() {
            if *e != event {
                self.to_process.push_back(event);
            }
            return Ok(());
        } else if self.to_process.is_empty() {
            self.to_process.push_back(event);
            return Ok(());
        }
        Err(())
    }
    fn add_event_ignore(&mut self, event: ProgramEvent) {
        let _ = self.add_event(event);
    }
    fn pop(&mut self) -> Option<ProgramEvent> {
        self.to_process.pop_front()
    }
}

const RENDERED_CELL_SIZE: (u32, u32) = (8u32, 8u32);
const WINDOW_SIZE: PhysicalSize<u32> = PhysicalSize::new(
    GAME_X as u32 * RENDERED_CELL_SIZE.0,
    GAME_Y as u32 * RENDERED_CELL_SIZE.1,
);

/// Entry point for GUI control and handling of the application
/// The program will run
pub(crate) fn gui(start: Option<game::Game>) {
    let mut game = GUIGameState::new((GAME_X, GAME_Y), RENDERED_CELL_SIZE);
    let (mut pixels, window, event_loop) = gui_init(WINDOW_SIZE);

    initial_gui_draw(&mut pixels);
    match pixels.render() {
        Ok(_) => {}
        Err(e) => eprintln!("Error rendering pixels! {e}"),
    };
    if let Some(starting_board) = start {
        game.load_new_board(starting_board);
    }
    println!("Controls: , -> Play, . -> Pause, g -> Grow, k -> Kill, = -> Step");
    println!("s -> Save, h -> Help, l -> Load, q -> Quit");

    run_gui(event_loop, window, pixels, game, ProgramManager::new());
}

fn gui_init(size: PhysicalSize<u32>) -> (Pixels, Window, EventLoop<()>) {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(size.width as f64, size.height as f64);
        WindowBuilder::new()
            .with_title("Game of Life")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_window_icon(None) //TODO: Add an icon here
            .build(&event_loop)
            .unwrap()
    };
    let pixels = {
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
        PixelsBuilder::new(size.width, size.height, surface_texture)
            .build()
            .unwrap()
    };
    (pixels, window, event_loop)
}

fn run_gui(
    l: EventLoop<()>,
    window: Window,
    mut pixels: Pixels,
    mut game: GUIGameState,
    mut state: ProgramManager,
) {
    l.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            game.consume_current_event(); // handle the game events

            if let Some(e) = state.pop(){
                match e{
                    ProgramEvent::ShowHelp => println!(
                        "Menu: ','->Play, '.'->Pause, 'g'->Grow, 'K'->Kill, '='->Step, 'S'->Save, 'L'->Load"
                    ),
                    ProgramEvent::SaveBoard => {
                        let _path = text::get_file_path();
                        // save_load::save_board(path.trim(), &game.board);
                        todo!("add saving / loading support for new game struct")
                    },
                    ProgramEvent::LoadBoard => {
                        let path = text::get_file_path();
                        if let Some(_b) = save_load::load_board_from_path(path.trim()){
                            // game.load_new_board(b);
                            todo!("add loading new board struct from path")
                        }
                        else{
                            eprintln!("Couldn't load board!");
                        }
                    },
                    ProgramEvent::ExitApplication => *control_flow = ControlFlow::Exit,
                }
            }

            window.request_redraw(); // This shouldn't be needed, but doesn't update w/o it
        }
        Event::RedrawRequested(id) if window.id() == id => {
            draw_board(&game.board, &mut pixels, game.size_of_cell);
            match pixels.render(){
                Ok(_) => {},
                Err(e) => eprintln!("Error Rendering with Pixels: {e}"),
            };
        }
        Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput { input, .. } if input.virtual_keycode.is_some() => {
                match input.virtual_keycode.unwrap() {
                    VirtualKeyCode::Comma => game.current_action = Some(GUIGameAction::Play),
                    VirtualKeyCode::Period => game.current_action = Some(GUIGameAction::Paused),
                    VirtualKeyCode::G => game.current_action = Some(GUIGameAction::GrowCell),
                    VirtualKeyCode::K => game.current_action = Some(GUIGameAction::KillCell),
                    VirtualKeyCode::Equals => game.current_action = Some(GUIGameAction::Step),

                    VirtualKeyCode::S => state.add_event_ignore(ProgramEvent::SaveBoard),
                    VirtualKeyCode::H => state.add_event_ignore(ProgramEvent::ShowHelp),
                    VirtualKeyCode::L => state.add_event_ignore(ProgramEvent::LoadBoard),
                    VirtualKeyCode::Q => state.add_event_ignore(ProgramEvent::ExitApplication),

                    VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                    _ => {}
                }
            }
            _ => {}
        },
        _ => {}
    });
}
fn initial_gui_draw(pixels: &mut Pixels) {
    for (idx, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
        let (x, y) = (
            idx % WINDOW_SIZE.width as usize,
            idx / WINDOW_SIZE.width as usize,
        );
        let color = [(x % 255) as u8, (y % 255) as u8, 128u8, 128u8];
        pixel.copy_from_slice(&color);
    }
}
fn draw_board(board: &game::Game, pixels: &mut Pixels, alignment: (u32, u32)) {
    const PADDING: u32 = 2;
    const BLACK: [u8; 4] = [0; 4];
    const WHITE: [u8; 4] = [200; 4];

    for (idx, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
        let (x, y) = (
            idx as u32 % WINDOW_SIZE.width,
            idx as u32 / WINDOW_SIZE.width,
        );

        // convert physical pixel coordinate into cell coordinate
        // also idk how this works exactly (comment written after code)
        let (cell_row, cell_col) = (x / alignment.0, y / alignment.1);
        let color = {
            #[allow(clippy::if_same_then_else)] // readability? could change later
            if board[(cell_row as usize, cell_col as usize)] == CellState::Dead {
                BLACK
            } else if x % alignment.0 <= PADDING || y % alignment.1 <= PADDING {
                BLACK
            } else {
                WHITE
            }
        };
        pixel.copy_from_slice(&color);
    }
}
