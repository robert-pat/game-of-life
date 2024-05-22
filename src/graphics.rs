use crate::game::CellState;
use crate::{game, save_load, text, GAME_X, GAME_Y};
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use std::collections::VecDeque;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

struct StepDelay {
    prev_step: std::time::Instant,
    delay: std::time::Duration,
}
impl StepDelay {
    fn can_step(&mut self) -> bool {
        self.prev_step.elapsed() > self.delay
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
    timing: StepDelay,
}
impl GUIGameState {
    pub(crate) fn new(size: (usize, usize)) -> Self {
        GUIGameState {
            board: game::Game::new(size.0, size.1),
            current_action: None,
            timing: StepDelay {
                delay: std::time::Duration::from_millis(200),
                prev_step: std::time::Instant::now(),
            },
        }
    }
    pub(crate) fn from_game(game: game::Game) -> Self {
        GUIGameState {
            board: game,
            current_action: None,
            timing: StepDelay {
                delay: std::time::Duration::from_millis(200),
                prev_step: std::time::Instant::now(),
            },
        }
    }
    pub(crate) fn tick(&mut self) {
        self.board.step(1);
        self.timing.prev_step = std::time::Instant::now();
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
                if self.timing.can_step() {
                    self.tick();
                }
            }
            GUIGameAction::GrowCell => {
                let to_change = text::get_coordinates(&std::io::stdin());
                self.board.set_many(&to_change, &[CellState::Alive; 1]);
            }
            GUIGameAction::KillCell => {
                let to_change = text::get_coordinates(&std::io::stdin());
                self.board.set_many(&to_change, &[CellState::Dead; 1]);
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

#[derive(Debug)]
struct DrawInformation {
    screen_size: PhysicalSize<u32>,
    cell_size: (u32, u32),
    padding: u32,
}
impl DrawInformation {
    #[allow(unused)]
    fn index_to_pixel(&self, idx: usize) -> (u32, u32) {
        (
            idx as u32 % self.screen_size.width,
            idx as u32 / self.screen_size.width,
        )
    }
    #[warn(incomplete_features)]
    fn index_to_cell(&self, idx: usize) -> Option<(usize, usize)> {
        // pixel coordinates
        let (x, y) = (
            idx as u32 % self.screen_size.width,
            idx as u32 / self.screen_size.width,
        );
        if x % (self.cell_size.0 + self.padding) <= self.padding
            || y % (self.cell_size.1 + self.padding) <= self.padding
        {
            return None;
        }

        let (cell_x, cell_y) = (
            x / (self.cell_size.0 + self.padding),
            y / (self.cell_size.1 + self.padding),
        );
        Some((cell_x as usize, cell_y as usize))
    }
}

/// Entry point for GUI control and handling of the application
/// The program will run
pub(crate) fn gui(start: Option<game::Game>) {
    let game = match start {
        Some(g) => GUIGameState::from_game(g),
        None => GUIGameState::new((GAME_X, GAME_Y)),
    };

    const PIXELS_PER_CELL: (u32, u32) = (8u32, 8u32);
    const PADDING: u32 = 2u32;
    let draw_info = DrawInformation {
        screen_size: PhysicalSize::new(
            (game.board.x_max as u32) * (PIXELS_PER_CELL.0 + PADDING) + PADDING,
            (game.board.y_max as u32) * (PIXELS_PER_CELL.1 + PADDING) + PADDING,
        ),
        cell_size: PIXELS_PER_CELL,
        padding: PADDING,
    };
    let (mut p, w, e) = gui_init(draw_info.screen_size);
    initial_gui_draw(&mut p, &draw_info);

    match p.render() {
        Ok(_) => {}
        Err(e) => eprintln!("Error rendering pixels! {e}"),
    };

    println!("Controls:");
    println!(" , -> Play, . -> Pause, g -> Grow, k -> Kill, = -> Step, s -> Save, h -> Help, l -> Load, q -> Quit");

    run_gui(e, w, p, game, ProgramManager::new(), draw_info);
}

fn gui_init(size: PhysicalSize<u32>) -> (Pixels, Window, EventLoop<()>) {
    let event_loop = EventLoop::new();
    let window = {
        // let size = LogicalSize::new(size.width as f64, size.height as f64);
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
    mut pixels: Pixels, // TODO: just move the code for gui_init here + idk
    mut game: GUIGameState,
    mut state: ProgramManager,
    draw_info: DrawInformation,
) {
    l.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            game.consume_current_event(); // handle the game events

            if let Some(e) = state.pop(){
                match e {
                    ProgramEvent::ShowHelp => println!(
                        "Menu: ','->Play, '.'->Pause, 'g'->Grow, 'K'->Kill, '='->Step, 'S'->Save, 'L'->Load"
                    ),
                    ProgramEvent::SaveBoard => {
                        let path = text::get_file_path();
                        match save_load::save_game(&game.board, &path) {
                            Ok(_) => {},
                            Err(e) => eprintln!("Issue saving board: {:?}", e),
                        };
                    },
                    ProgramEvent::LoadBoard => {
                        let path = text::get_file_path();
                        match save_load::load_game(path.trim()) {
                            Ok(b) => game.load_new_board(b),
                            Err(e) => eprintln!("Couldn't load board: {:?}", e),
                        };
                    },
                    ProgramEvent::ExitApplication => *control_flow = ControlFlow::Exit,
                }
            }
        }
        Event::RedrawRequested(id) if window.id() == id => {
            draw_board(&game.board, &mut pixels, &draw_info);
            match pixels.render(){
                Ok(_) => {},
                Err(e) => eprintln!("Error Rendering with Pixels: {e}"),
            };
        }
        Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput { input, .. } if input.virtual_keycode.is_some() => {
                // prevent double pressing, NOTE: this prob needs to be changed later !!!
                if input.state == ElementState::Released {
                    return;
                }
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
                window.request_redraw();
            }
            _ => {}
        },
        _ => {}
    });
}
fn initial_gui_draw(pixels: &mut Pixels, info: &DrawInformation) {
    for (idx, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
        let color = match info.index_to_cell(idx) {
            // Some((x, y)) => [(x % 255) as u8, (y % 255) as u8, 128u8, 128u8],
            Some((x, y)) => [(x % 2) as u8 * 128u8, (y % 2) as u8 * 128u8, 128u8, 128u8],
            None => [0u8; 4],
        };
        pixel.copy_from_slice(&color);
    }
}

#[warn(incomplete_features)]
fn draw_board(board: &game::Game, pixels: &mut Pixels, draw_info: &DrawInformation) {
    const BLACK: [u8; 4] = [0; 4];
    const WHITE: [u8; 4] = [200; 4];

    for (idx, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
        if let Some((x, y)) = draw_info.index_to_cell(idx) {
            match board[(x, y)] {
                CellState::Alive => pixel.copy_from_slice(&WHITE),
                CellState::Dead => pixel.copy_from_slice(&BLACK),
            };
            continue;
        }

        pixel.copy_from_slice(&BLACK);
    }
}
