use eframe::{egui, glow::STENCIL_FAIL};
use egui::{emath::RectTransform, epaint::RectShape, Color32, Pos2, Rect, Rounding, Shape, Stroke};

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rusty Chess",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

#[derive(Default)]
struct MyEguiApp {
    board: Vec<egui::Rect>,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        cc.egui_ctx.set_visuals(egui::Visuals::light());
        Self::default()
    }

    fn default() -> MyEguiApp {
        return MyEguiApp {
            board: create_board(),
        };
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("");

            // TODO: Don't create the board every loop
            let board = create_board();
            let mut count = 0;
            let mut cycle = 0;

            // Fix this janky logic for checkerboard colors later
            for sqr in board {
                if cycle == 0 {
                    if count % 2 == 0 {
                        ui.painter()
                            .rect_filled(sqr, Rounding::ZERO, Color32::BLACK);
                    } else {
                        ui.painter()
                            .rect_filled(sqr, Rounding::ZERO, Color32::WHITE);
                    }
                } else {
                    if count % 2 == 0 {
                        ui.painter()
                            .rect_filled(sqr, Rounding::ZERO, Color32::WHITE);
                    } else {
                        ui.painter()
                            .rect_filled(sqr, Rounding::ZERO, Color32::BLACK);
                    }
                }
                count += 1;
                // println!("Count: {} :: Cycle {}", count, cycle);
                if count % 8 == 0 {
                    if cycle == 1 {
                        cycle = 0;
                    } else {
                        cycle = 1;
                    }
                    count = 0;
                    // println!("Finished column. Setting cycle={} & count={}", cycle, count);
                    continue;
                }
            }
        });
    }
}

fn create_board() -> Vec<egui::Rect> {
    // Take example region 800x800
    // - 8x8 board would be squares 100x100
    // - dimensions starting at
    // sq1(0,0),(100,100)
    // sq2(0,100),(200, 100)
    let board_dim = 8;
    let x_dim = 800;
    let y_dim = 800;
    let step_size = x_dim / board_dim;

    let mut board: Vec<egui::Rect> = Vec::new();
    for i in (0..x_dim).step_by(step_size) {
        for j in (0..y_dim).step_by(step_size.into()) {
            // Make squares and append to a vector
            board.push(egui::Rect {
                min: Pos2 {
                    x: i as f32,
                    y: j as f32,
                },
                max: Pos2 {
                    x: (i + step_size) as f32,
                    y: (j + step_size) as f32,
                },
            });
        }
    }
    // println!("{:?}", board[0]);
    // println!("{:?}", board[1]);
    // println!("{:?}", board[2]);
    // println!("{:?}", board[3]);
    return board;
}
