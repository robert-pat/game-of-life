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
}
const RENDERED_CELL_SIZE: (u32, u32) = (8u32, 8u32);
const WINDOW_SIZE: PhysicalSize<u32> = PhysicalSize::new(
    GAME_X as u32 * RENDERED_CELL_SIZE.0,
    GAME_Y as u32 * RENDERED_CELL_SIZE.1,
);
/// Entry point for GUI control and handling of the application
/// The program will run
pub(crate) fn gui() -> ! {
    let game = GUIGameState::new((GAME_X, GAME_Y), RENDERED_CELL_SIZE);
    let (mut pixels, window, event_loop) = gui_init(WINDOW_SIZE);
    initial_gui_draw(&mut pixels);
    match pixels.render() {
        Ok(_) => {}
        Err(e) => eprintln!("Error rendering pixels! {e}"),
    };
    run_gui(event_loop, window, pixels, game);
}
pub(crate) fn gui_init(size: PhysicalSize<u32>) -> (Pixels, Window, EventLoop<()>) {
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
pub(crate) fn initial_gui_draw(pixels: &mut Pixels) {
    for (idx, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
        let (x, y) = (
            idx % WINDOW_SIZE.width as usize,
            idx / WINDOW_SIZE.width as usize,
        );
        let color = [(x % 255) as u8, (y % 255) as u8, 128u8, 128u8];
        pixel.copy_from_slice(&color);
    }
}
pub(crate) fn run_gui(
    l: EventLoop<()>,
    window: Window,
    mut pixels: Pixels,
    mut game: GUIGameState,
) -> ! {
    l.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => match &game.current_action {
            GameAction::Quit => *control_flow = ControlFlow::Exit,
            GameAction::Step => game.tick(),
            GameAction::Paused => {}
            GameAction::Play => {
                if game.timing.1.elapsed() >= game.timing.0 {
                    game.tick();
                }
            }
            GameAction::GrowCell => {
                text::prompt_user_to_change_cells(&mut game.board, CellState::Alive)
            }
            GameAction::KillCell => {
                text::prompt_user_to_change_cells(&mut game.board, CellState::Dead)
            }
            GameAction::Save => {
                save_load::save_board_to_file(&save_load::get_user_path(), &game.board)
            }
            GameAction::Failed | GameAction::PrintBoard => {
                eprintln!("Invalid Action for GUI")
            }
        },
        Event::RedrawRequested(id) if window.id() == id => {
            draw_board(&game.board, &mut pixels, game.size_of_cell);
            pixels.render().expect("Pixels Render Failed!");
        }
        Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::Focused(is_focused) if !is_focused => {
                game.current_action = GameAction::Paused
            }
            WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode.unwrap() {
                VirtualKeyCode::Comma => game.current_action = GameAction::Play,
                VirtualKeyCode::Period => game.current_action = GameAction::Paused,
                _ => {}
            },
            _ => {}
        },
        _ => {}
    });
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
