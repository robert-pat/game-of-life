use std::collections::VecDeque;
use crate::game::{CellState, GameAction};
use crate::{game, save_load, text, GAME_X, GAME_Y};
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

pub(crate) struct GUIGameState {
    pub(crate) board: game::GameBoard,
    #[allow(unused)] // Shouldn't need to read the previous board
    prev_board: game::GameBoard,
    current_action: GameAction,
    size_of_cell: (u32, u32),
    timing: (std::time::Duration, std::time::Instant),
}
impl GUIGameState {
    pub(crate) fn new(size: (usize, usize), pixels_per_cell: (u32, u32)) -> Self {
        GUIGameState {
            board: game::GameBoard::new(size.0, size.1),
            prev_board: game::GameBoard::new(size.0, size.1),
            current_action: GameAction::Paused,
            size_of_cell: pixels_per_cell,
            timing: (
                std::time::Duration::from_millis(200),
                std::time::Instant::now(),
            ),
        }
    }
    pub(crate) fn tick(&mut self) {
        std::mem::swap(&mut self.board, &mut self.prev_board);
        self.prev_board.update_to(&mut self.board);
    }
    pub(crate) fn load_new_board(&mut self, new: game::GameBoard) {
        let (b1, b2) = (new.clone(), new);
        self.board = b1;
        self.prev_board = b2;
        assert!(self.board == self.prev_board); //assert_eq!() requires std::fmt::Debug ??
    }
    pub(crate) fn consumer_current_event(&mut self) {
        match self.current_action {
            GameAction::Step => self.tick(),
            GameAction::Paused => {}
            GameAction::Play => {
                if self.timing.1.elapsed() >= self.timing.0 {
                    self.tick();
                }
            }
            GameAction::GrowCell => {
                text::prompt_user_to_change_cells(&mut self.board, CellState::Alive)
            }
            GameAction::KillCell => {
                text::prompt_user_to_change_cells(&mut self.board, CellState::Dead)
            }
            GameAction::Failed | GameAction::PrintBoard | GameAction::Save | GameAction::Quit => {
                eprintln!("Attempted to consume GameAction that's invalid for GUI!");
            }
        }
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
        ProgramManager { to_process: VecDeque::new() }
    }
    fn add_event(&mut self, event: ProgramEvent) -> Result<(), ()> {
        if let Some(e) = self.to_process.back() {
            if *e != event {
                self.to_process.push_back(event);
            }
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
const GAME_ACTIONS_TO_REMOVE: [GameAction; 2] = [GameAction::Quit, GameAction::Save];
/// Entry point for GUI control and handling of the application
/// The program will run
pub(crate) fn gui(start: Option<game::GameBoard>) -> ! {
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
) -> ! {
    l.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            debug_assert!(!GAME_ACTIONS_TO_REMOVE.contains(&game.current_action));
            game.consumer_current_event();

            // TODO: this currently breaks the whole loop, prevents the window from updating
            // Specifically, using "g" to grow cells
            if let Some(e) = state.pop(){
                use ProgramEvent as GEvent;
                match e{
                    GEvent::ShowHelp => println!(
                        "Menu: ','->Play, '.'->Pause, 'g'->Grow, 'K'->Kill, '='->Step, 'S'->Save, 'L'->Load"
                    ),
                    GEvent::SaveBoard => {
                        let path = save_load::get_file_path();
                        save_load::save_board_to_file(path.trim(), &game.board);
                    },
                    GEvent::LoadBoard => {
                        let path = save_load::get_file_path();
                        if let Some(b) = save_load::load_from_path(path.trim()){
                            game.load_new_board(b);
                        }
                        else{
                            eprintln!("Couldn't load board!");
                        }
                    },
                    GEvent::ExitApplication => *control_flow = ControlFlow::Exit,
                }
            }
        }
        Event::RedrawRequested(id) if window.id() == id => {
            draw_board(&game.board, &mut pixels, game.size_of_cell);
            pixels.render().expect("Pixels Render Failed!");
        }
        Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput { input, .. } if input.virtual_keycode.is_some() => {
                match input.virtual_keycode.unwrap() {
                    VirtualKeyCode::Comma => game.current_action = GameAction::Play,
                    VirtualKeyCode::Period => game.current_action = GameAction::Paused,
                    VirtualKeyCode::G => game.current_action = GameAction::GrowCell,
                    VirtualKeyCode::K => game.current_action = GameAction::KillCell,
                    VirtualKeyCode::Equals => game.current_action = GameAction::Step,

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
fn draw_board(board: &game::GameBoard, pixels: &mut Pixels, alignment: (u32, u32)) {
    const PADDING: u32 = 2;
    const BLACK: [u8; 4] = [0; 4];
    const WHITE: [u8; 4] = [200; 4];
    for (idx, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
        let (x, y) = (
            idx as u32 % WINDOW_SIZE.width,
            idx as u32 / WINDOW_SIZE.width,
        );
        let (c_row, c_col) = (x / alignment.0, y / alignment.1);
        let color = {
            #[allow(clippy::if_same_then_else)] // readability? could change later
            if board.get(c_row as usize, c_col as usize) == CellState::Dead {
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
