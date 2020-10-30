use nannou::prelude::*;

#[macro_use]

#[path = "penrose.rs"]
mod penrose;

use penrose::*;

trait Drawable {
    fn draw(&self, draw: &nannou::draw::Draw, xoff: f32, yoff: f32, scale: f32);
}

impl Drawable for Dart {
    fn draw(&self, draw: &nannou::draw::Draw, xoff: f32, yoff: f32, scale: f32) {
        let pts = self.polygon(xoff, yoff, scale);
        let points = (0..4).map(|i| {
            pt2(pts[i].0, pts[i].1)
        });
        draw.polygon()
            .color(WHITE)
            .stroke(PINK)
            .stroke_weight(2.)
            .join_miter()
            .points(points);
    }
}

impl Drawable for Kite {
    fn draw(&self, draw: &nannou::draw::Draw, xoff: f32, yoff: f32, scale: f32) {
        let pts = self.polygon(xoff, yoff, scale);
        let points = (0..4).map(|i| {
            pt2(pts[i].0, pts[i].1)
        });
        draw.polygon()
            .color(WHITE)
            .stroke(PINK)
            .stroke_weight(2.)
            .join_miter()
            .points(points);
    }
}

fn build_vertex1(x: f64, y: f64, angle: i32) -> Result<Vec<Box<dyn Drawable>>, i32> {
    let phi = (1. + 5_f64.sqrt())/2.;
    let d1 = Dart::new(x - phi, y, angle);
    let d2 = place_dart_edge(3, d1.edge_center(2)?, d1.edge_angle(2)?);
    let d3 = place_dart_edge(3, d2.edge_center(2)?, d2.edge_angle(2)?);
    let d4 = place_dart_edge(3, d3.edge_center(2)?, d3.edge_angle(2)?);
    let d5 = place_dart_edge(3, d4.edge_center(2)?, d4.edge_angle(2)?);
    let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
    tiles.push(Box::new(d1));
    tiles.push(Box::new(d2));
    tiles.push(Box::new(d3));
    tiles.push(Box::new(d4));
    tiles.push(Box::new(d5));
    Ok(tiles)
}

fn build_vertex2(x: f64, y: f64, angle: i32) -> Result<Vec<Box<dyn Drawable>>, i32> {
    let d1 = Dart::new(x, y, angle);
    let k1 = place_kite_edge(2, d1.edge_center(4)?, d1.edge_angle(4)?);
    let k2 = place_kite_edge(4, k1.edge_center(1)?, k1.edge_angle(1)?);

    let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
    tiles.push(Box::new(d1));
    tiles.push(Box::new(k1));
    tiles.push(Box::new(k2));
    Ok(tiles)
}

fn build_vertex3(x: f64, y: f64, angle: i32) -> Result<Vec<Box<dyn Drawable>>, i32> {
    let phi = (1. + 5_f64.sqrt())/2.;

    let k1 = Kite::new(x + phi, y, angle);
    let k2 = place_kite_edge(1, k1.edge_center(4)?, k1.edge_angle(4)?);
    let k3 = place_kite_edge(1, k2.edge_center(4)?, k2.edge_angle(4)?);
    let k4 = place_kite_edge(1, k3.edge_center(4)?, k3.edge_angle(4)?);
    let k5 = place_kite_edge(1, k4.edge_center(4)?, k4.edge_angle(4)?);

    let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
    tiles.push(Box::new(k1));
    tiles.push(Box::new(k2));
    tiles.push(Box::new(k3));
    tiles.push(Box::new(k4));
    tiles.push(Box::new(k5));
    Ok(tiles)
}

fn build_vertex4(x: f64, y: f64, angle: i32) -> Result<Vec<Box<dyn Drawable>>, i32> {
    let phi = (1. + 5_f64.sqrt())/2.;

    let d1 = Dart::new(x - phi, y, angle);
    let d2 = place_dart_edge(3, d1.edge_center(2)?, d1.edge_angle(2)?);
    let k1 = place_kite_edge(1, d2.edge_center(2)?, d2.edge_angle(2)?);
    let k2 = place_kite_edge(1, k1.edge_center(4)?, k1.edge_angle(4)?);
    let d3 = place_dart_edge(3, k2.edge_center(4)?, k2.edge_angle(4)?);

    let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
    tiles.push(Box::new(d1));
    tiles.push(Box::new(d2));
    tiles.push(Box::new(k1));
    tiles.push(Box::new(k2));
    tiles.push(Box::new(d3));
    Ok(tiles)
}

fn build_vertex5(x: f64, y: f64, angle: i32) -> Result<Vec<Box<dyn Drawable>>, i32> {

    let k1 = Kite::new(x - 1., y, angle);
    let d1 = place_dart_edge(4, k1.edge_center(2)?, k1.edge_angle(2)?);
    let k2 = place_kite_edge(1, d1.edge_center(3)?, d1.edge_angle(3)?);
    let k3 = place_kite_edge(1, k2.edge_center(4)?, k2.edge_angle(4)?);
    let d2 = place_dart_edge(2, k3.edge_center(4)?, k3.edge_angle(4)?);

    let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
    tiles.push(Box::new(k1));
    tiles.push(Box::new(d1));
    tiles.push(Box::new(k2));
    tiles.push(Box::new(k3));
    tiles.push(Box::new(d2));
    Ok(tiles)
}

fn build_vertex6(x: f64, y: f64, angle: i32) -> Result<Vec<Box<dyn Drawable>>, i32> {
    let phi = (1. + 5_f64.sqrt())/2.;

    let d1 = Dart::new(x - phi, y, angle);
    let k1 = place_kite_edge(4, d1.edge_center(2)?, d1.edge_angle(2)?);
    let k2 = place_kite_edge(2, k1.edge_center(3)?, k1.edge_angle(3)?);
    let k3 = place_kite_edge(4, k2.edge_center(1)?, k2.edge_angle(1)?);
    let k4 = place_kite_edge(2, k3.edge_center(3)?, k3.edge_angle(3)?);

    let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
    tiles.push(Box::new(d1));
    tiles.push(Box::new(k1));
    tiles.push(Box::new(k2));
    tiles.push(Box::new(k3));
    tiles.push(Box::new(k4));
    Ok(tiles)
}

fn build_vertex7(x: f64, y: f64, angle: i32) -> Result<Vec<Box<dyn Drawable>>, i32> {
    let s5 = 5_f64.sqrt();
    let phi = (1.+s5)/2.;
    let k = (2.+s5) / (1.+s5);
    let p = (10. + 20_f64.sqrt()).sqrt()/4.;

    let k1 = Kite::new(x + k - 1., y - p, angle);
    let k2 = place_kite_edge(3, k1.edge_center(2)?, k1.edge_angle(2)?);
    let d1 = place_dart_edge(4, k2.edge_center(2)?, k2.edge_angle(2)?);
    let d2 = place_dart_edge(2, d1.edge_center(3)?, d1.edge_angle(3)?);

    let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
    tiles.push(Box::new(k1));
    tiles.push(Box::new(k2));
    tiles.push(Box::new(d1));
    tiles.push(Box::new(d2));
    Ok(tiles)
}

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .view(view)
        .run();
}

struct Model {
    tiles: Vec<Box<dyn Drawable>>,
    current_point: Point2,
    scale: f64,
    vertex_type: i32,
    angle: i32,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(720, 720)
        .event(window_event)
        .raw_event(raw_window_event)
        .key_pressed(key_pressed)
        .key_released(key_released)
        .mouse_moved(mouse_moved)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .mouse_wheel(mouse_wheel)
        .mouse_entered(mouse_entered)
        .mouse_exited(mouse_exited)
        .touch(touch)
        .touchpad_pressure(touchpad_pressure)
        .moved(window_moved)
        .resized(window_resized)
        .hovered_file(hovered_file)
        .hovered_file_cancelled(hovered_file_cancelled)
        .dropped_file(dropped_file)
        .focused(window_focused)
        .unfocused(window_unfocused)
        .closed(window_closed)
        .build()
        .unwrap();
    Model { tiles: Vec::new(),
            current_point: pt2(0.,0.),
            scale: 25.,
            vertex_type: 1,
            angle: 0,
    }
}

fn event(_app: &App, _model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _,
            //raw: _,
            simple: _,
        } => {}
        Event::DeviceEvent(_device_id, _event) => {}
        Event::Update(_dt) => {}
        Event::Suspended => {}
        Event::Resumed => {}
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {

    // Begin drawing
    let draw: nannou::draw::Draw = app.draw();

    // Clear the background to blue.
    draw.background().color(CORNFLOWERBLUE);

    for t in &model.tiles {
        t.draw(&draw, 0., 0., model.scale as f32);
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

use nannou::event::*;

fn window_event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => {
            match key {
                Key::Key1 => model.vertex_type = 1,
                Key::Key2 => model.vertex_type = 2,
                Key::Key3 => model.vertex_type = 3,
                Key::Key4 => model.vertex_type = 4,
                Key::Key5 => model.vertex_type = 5,
                Key::Key6 => model.vertex_type = 6,
                Key::Key7 => model.vertex_type = 7,
                Key::Up => { model.scale = 2.*model.scale.min(100.) },
                Key::Down => { model.scale = 0.5*model.scale.max(1.) },
                Key::Left => { model.angle = (model.angle + 72) % 360 },
                Key::Right => { model.angle = (model.angle + 360 - 72) % 360 },
                _ => println!("KeyPressed = {:?}", key),
            }
        }
        KeyReleased(_key) => {}
        MouseMoved(pos) => { model.current_point = pos }
        MousePressed(_button) => {
            let x = model.current_point.x as f64 / model.scale;
            let y = model.current_point.y as f64 / model.scale;
            let res = match model.vertex_type {
                1 => build_vertex1(x, y, model.angle),
                2 => build_vertex2(x, y, model.angle),
                3 => build_vertex3(x, y, model.angle),
                4 => build_vertex4(x, y, model.angle),
                5 => build_vertex5(x, y, model.angle),
                6 => build_vertex6(x, y, model.angle),
                7 => build_vertex7(x, y, model.angle),
                _ => Err(666),
            };
            match res {
                Ok(t) => for o in t { model.tiles.push(o) },
                Err(_) => println!("Error building vertex 2"),
            }
        }
        MouseReleased(_button) => {}
        MouseEntered => {}
        MouseExited => {}
        MouseWheel(_amount, _phase) => {}
        Moved(_pos) => {}
        Resized(_size) => {}
        Touch(_touch) => {}
        TouchPressure(_pressure) => {}
        HoveredFile(_path) => {}
        DroppedFile(_path) => {}
        HoveredFileCancelled => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
    }
}

fn raw_window_event(_app: &App, _model: &mut Model, _event: &nannou::winit::event::WindowEvent) {}

fn key_pressed(_app: &App, _model: &mut Model, _key: Key) {}

fn key_released(_app: &App, _model: &mut Model, _key: Key) {}

fn mouse_moved(_app: &App, _model: &mut Model, _pos: Point2) {}

fn mouse_pressed(_app: &App, _model: &mut Model, _button: MouseButton) {}

fn mouse_released(_app: &App, _model: &mut Model, _button: MouseButton) {}

fn mouse_wheel(_app: &App, _model: &mut Model, _dt: MouseScrollDelta, _phase: TouchPhase) {}

fn mouse_entered(_app: &App, _model: &mut Model) {}

fn mouse_exited(_app: &App, _model: &mut Model) {}

fn touch(_app: &App, _model: &mut Model, _touch: TouchEvent) {}

fn touchpad_pressure(_app: &App, _model: &mut Model, _pressure: TouchpadPressure) {}

fn window_moved(_app: &App, _model: &mut Model, _pos: Point2) {}

fn window_resized(_app: &App, _model: &mut Model, _dim: Vector2) {}

fn window_focused(_app: &App, _model: &mut Model) {}

fn window_unfocused(_app: &App, _model: &mut Model) {}

fn window_closed(_app: &App, _model: &mut Model) {}

fn hovered_file(_app: &App, _model: &mut Model, _path: std::path::PathBuf) {}

fn hovered_file_cancelled(_app: &App, _model: &mut Model) {}

fn dropped_file(_app: &App, _model: &mut Model, _path: std::path::PathBuf) {}
