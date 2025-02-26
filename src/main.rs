extern crate glfw_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::process::exit;

use glfw;
use glfw::WindowMode;
use glfw_window::GlfwWindow as Window;
use opengl_graphics::GlGraphics;
use opengl_graphics::OpenGL;
use piston::window::WindowSettings;
use piston::Button;
use piston::ButtonEvent;
use piston::ButtonState;
use piston::EventSettings;
use piston::Events;
use piston::MouseCursorEvent;
use piston::RenderArgs;
use piston::RenderEvent;
use piston::UpdateArgs;
use piston::UpdateEvent;

const WINDOW_NAME: &str = "Pomiar szybkości celowania myszką";
/// Delay before displaying new point after user clicked previous point and stopped moving mouse
const NEW_POINT_DELAY_TIME: f64 = 0.2;
/// Size of clicking targets
const CLICK_POINT_SIZE: f64 = 20.;
/// Number of clicks
const N: u32 = 5;
/// Number of clicks, that should be ignored before program starts actually measuring time. This
/// allows user to get used to using this program.
const IGNORE_FIRST_N: u32 = 3;

/// State of application
#[derive(Debug)]
enum AppState {
    /// user is moving mouse after clicking previous point, program will not continue until user
    /// stops moving the mouse
    WaitingForMouseStop,
    /// user clicked point and stopped moving mouse, there is short delay before displaying next
    /// point, f64 - time since mouse stopped moving
    NewPointDelay(f64),
    /// new point is displayed on the screen, but user has not started moving mouse yet, this
    /// program is for measuring user pointing time, not user reaction time, so it will not measure
    /// time now
    WaitingForMouseMove,
    /// user started moving mouse to click new point, now program measures time, f64 - measured
    /// time
    Measuring(f64),
}

pub struct PomiarMyszyApp {
    gl: GlGraphics,
    state: AppState,
    point_position: (f64, f64),
    point_distance: f64,
    mouse_pos: (f64, f64),
    mouse_moving: bool,
    window_size: (i32, i32),
    point_number: u32,
    ignored: bool,
}

impl PomiarMyszyApp {
    fn new(gl: GlGraphics, window_size: (i32, i32)) -> Self {
        PomiarMyszyApp {
            gl,
            state: AppState::WaitingForMouseStop,
            point_position: (0., 0.),
            point_distance: 0.,
            mouse_pos: (0., 0.),
            mouse_moving: false,
            window_size,
            point_number: 0,
            ignored: true,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0]; // background
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0]; // point

        let click_point = rectangle::square(
            self.point_position.0,
            self.point_position.1,
            CLICK_POINT_SIZE,
        );

        let render_point = self.is_point_rendered();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);

            if render_point {
                rectangle(BLACK, click_point, c.transform, gl);
            }
        });
    }

    /// Returns if click point should be now rendered
    fn is_point_rendered(&self) -> bool {
        match self.state {
            AppState::WaitingForMouseMove => true,
            AppState::Measuring(_) => true,
            _ => false,
        }
    }

    /// Update state, dt - delta time
    fn update_state(&mut self, dt: f64) {
        match self.state {
            AppState::WaitingForMouseStop => {
                if !self.mouse_moving {
                    self.state = AppState::NewPointDelay(0.);
                }
            }
            AppState::NewPointDelay(time) => {
                if self.mouse_moving {
                    self.state = AppState::WaitingForMouseStop;
                    return;
                }

                if time >= NEW_POINT_DELAY_TIME {
                    self.state = AppState::WaitingForMouseMove;
                    return;
                }
                self.state = AppState::NewPointDelay(time + dt);
            }
            AppState::WaitingForMouseMove => {
                if self.mouse_moving {
                    self.state = AppState::Measuring(0.);
                }
            }
            AppState::Measuring(time) => {
                self.state = AppState::Measuring(time + dt);
            }
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.update_state(args.dt);

        self.mouse_moving = false;
    }

    /// User moved mouse, update mouse position
    fn mouse_moved(&mut self, x: f64, y: f64) {
        self.mouse_pos = (x, y);
        self.mouse_moving = true;
    }

    fn mouse_clicked(&mut self) {
        // In other states clicking mouse is not important
        if let AppState::Measuring(time) = self.state {
            // Create temporary variables with shroter names
            let (mx, my) = self.mouse_pos;
            let (px1, py1) = self.point_position;
            let px2 = px1 + CLICK_POINT_SIZE;
            let py2 = py1 + CLICK_POINT_SIZE;
            // If cursor is at clicking point
            if px1 <= mx && mx <= px2 && py1 <= my && my < py2 {
                if self.ignored && self.point_number >= IGNORE_FIRST_N {
                    // if clicks should not be ignored anymore
                    self.ignored = false;
                    self.point_number = 0;
                }
                if !self.ignored {
                    println!("{}; {}; {}", self.point_number, self.point_distance, time);
                    if self.point_number >= N - 1 {
                        exit(0);
                    }
                }
                self.point_number += 1;
                self.state = AppState::WaitingForMouseStop;
                self.new_point();
            }
        }
    }

    /// Generate new random point
    fn new_point(&mut self) {
        let new_x = rand::random_range(0..(self.window_size.0 - CLICK_POINT_SIZE as i32));
        let new_y = rand::random_range(0..(self.window_size.1 - CLICK_POINT_SIZE as i32));
        self.point_position = (new_x as f64, new_y as f64);

        // Compute distance (sqrt((x1-x1)^2 + (y1-y2)^2))
        let dist_squared = ((new_x - self.mouse_pos.0 as i32).pow(2)
            + (new_y - self.mouse_pos.1 as i32).pow(2)) as f64;
        self.point_distance = dist_squared.sqrt();
    }
}

fn enable_fullscreen(win: &mut Window) {
    win.glfw
        .with_primary_monitor(|_: &mut _, m: Option<&mut glfw::Monitor>| {
            let monitor = m.unwrap();
            let mode = monitor.get_video_mode().unwrap();

            win.window.set_size(mode.width as i32, mode.height as i32);

            win.window.set_monitor(
                WindowMode::FullScreen(&monitor),
                0,
                0,
                mode.width,
                mode.height,
                Some(mode.refresh_rate),
            );
        });
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(WINDOW_NAME, [1920, 1080])
        .graphics_api(opengl)
        .resizable(false)
        .build()
        .unwrap();

    enable_fullscreen(&mut window);

    let size = window.window.get_size();

    let mut app = PomiarMyszyApp::new(GlGraphics::new(opengl), size);

    let mut events = Events::new(EventSettings::new());

    println!("szerokośc monitora; {}", size.0);
    println!("wysokość monitora: {}", size.1);

    println!("i; odległość [piksele]; czas [s]");

    app.new_point();

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.mouse_cursor_args() {
            app.mouse_moved(args[0], args[1]);
        }

        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press
                && args.button == Button::Mouse(piston::MouseButton::Left)
            {
                app.mouse_clicked();
            }
        }
    }
}
