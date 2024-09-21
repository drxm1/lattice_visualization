use eframe::egui::{self, Color32, Painter, Pos2, Stroke, Vec2};
use eframe::{App, Frame};

struct MyApp {
    n: usize,
    lattice_extent: i32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            n: 3,
            lattice_extent: 4,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        egui::SidePanel::left("controls").show(ctx, |ui| {
            ui.heading("Controls");
            ui.add(egui::Slider::new(&mut self.n, 1..=12).text("n"));
            ui.add(egui::Slider::new(&mut self.lattice_extent, 1..=10).text("Lattice Extent"));

            if [2, 3, 4, 6].contains(&self.n) {
                ui.label("Valid lattice structure");
            } else {
                ui.label("Invalid lattice structure");
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            draw_lattice(painter, self.n, self.lattice_extent, ui.available_size());
        });
    }
}

fn draw_lattice(painter: &Painter, n: usize, lattice_extent: i32, available_size: egui::Vec2) {
    let a = 100.0; // Lattice constant
    let origin = Pos2::new(400.0, 300.0); // Center of the screen

    let phi = 2.0 * std::f32::consts::PI / n as f32;

    // Points A and B
    let a_point = origin;
    let b_point = Pos2::new(origin.x + a, origin.y);

    // Rotate A around B to get C
    let c_point = rotate_point(a_point, b_point, phi);

    // Rotate B around A to get D
    let d_point = rotate_point(b_point, a_point, -phi);

    // Draw points
    let points = vec![a_point, b_point, c_point, d_point];
    for &point in &points {
        painter.circle_filled(point, 5.0, Color32::RED);
    }

    // Draw lines between points
    painter.line_segment([a_point, b_point], (2.0, Color32::WHITE));
    painter.line_segment([b_point, c_point], (2.0, Color32::WHITE));
    painter.line_segment([a_point, d_point], (2.0, Color32::WHITE));
    painter.line_segment([c_point, d_point], (2.0, Color32::YELLOW));

    // Define primitive lattice vectors
    let v1 = Vec2::new(b_point.x - a_point.x, b_point.y - a_point.y);
    let v2 = Vec2::new(d_point.x - a_point.x, d_point.y - a_point.y);

    // Draw primitive lattice vectors
    let primitive_vector_color = Color32::from_rgba_unmultiplied(128, 0, 128, 200); // Dark purple, semi-opaque
    painter.arrow(a_point, v1, Stroke::new(3.0, primitive_vector_color));
    painter.arrow(a_point, v2, Stroke::new(3.0, primitive_vector_color));

    // Generate and draw lattice points
    let is_valid_lattice = [1, 2, 3, 4, 6].contains(&n);
    let lattice_point_color = if is_valid_lattice {
        Color32::LIGHT_BLUE
    } else {
        Color32::LIGHT_RED
    };

    for i in -lattice_extent..=lattice_extent {
        for j in -lattice_extent..=lattice_extent {
            let lattice_point = Pos2::new(
                a_point.x + i as f32 * v1.x + j as f32 * v2.x,
                a_point.y + i as f32 * v1.y + j as f32 * v2.y,
            );
            painter.circle_filled(lattice_point, 3.0, lattice_point_color);
        }
    }

    // Draw a note about lattice validity at the bottom right
    let text = if is_valid_lattice {
        "Valid lattice structure"
    } else {
        "Invalid lattice structure"
    };
    let font_size = 20.0;
    let text_color = Color32::WHITE;
    let font_id = egui::FontId::proportional(font_size);
    let text_galley = painter.layout_no_wrap(text.to_string(), font_id.clone(), text_color);
    let text_pos = Pos2::new(
        available_size.x - text_galley.rect.width() - 10.0, // 10 pixels from right edge
        available_size.y - text_galley.rect.height() - 10.0, // 10 pixels from bottom edge
    );
    painter.text(
        text_pos,
        egui::Align2::RIGHT_BOTTOM,
        text,
        font_id,
        text_color,
    );
}

fn rotate_point(p: Pos2, o: Pos2, theta: f32) -> Pos2 {
    let sin_theta = theta.sin();
    let cos_theta = theta.cos();

    let dx = p.x - o.x;
    let dy = p.y - o.y;

    let x_new = cos_theta * dx - sin_theta * dy + o.x;
    let y_new = sin_theta * dx + cos_theta * dy + o.y;

    Pos2::new(x_new, y_new)
}

fn main() {
    let native_options = eframe::NativeOptions::default();

    // Check if the backend is set
    if let Ok(backend) = std::env::var("WINIT_UNIX_BACKEND") {
        println!("Backend: {}", backend);
    }

    if let Err(e) = eframe::run_native(
        "Lattice Symmetry Visualization",
        native_options,
        Box::new(|_cc| Box::new(MyApp::default())),
    ) {
        eprintln!("Error: {}", e);
    }
}
