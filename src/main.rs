use nannou::prelude::*;

#[macro_use]

#[path = "penrose.rs"]
mod penrose;

trait Drawable {
    fn get_points(&self, xoff: f32, yoff: f32, scale: f32) -> Vec<(f32,f32)> ;
}

impl Drawable for penrose::Dart {
    fn get_points(&self, xoff: f32, yoff: f32, scale: f32) -> Vec<(f32,f32)> {
        let pts = self.polygon(xoff, yoff, scale);
        pts
    }
}

impl Drawable for penrose::Kite {
    fn get_points(&self, xoff: f32, yoff: f32, scale: f32) -> Vec<(f32,f32)> {
        let pts = self.polygon(xoff, yoff, scale);
        pts
    }
}

fn build_vertex1() -> Result<Vec<Box<dyn Drawable>>, i32> {
    let s5 = (5 as f64).sqrt();
    let phi = (1.+s5)/2.;
    let d1 = penrose::Dart::new(-phi, 0., 0);
    let d2 = penrose::place_dart_edge(3, d1.edge_center(2)?, d1.edge_angle(2)?);
    let d3 = penrose::place_dart_edge(3, d2.edge_center(2)?, d2.edge_angle(2)?);
    let d4 = penrose::place_dart_edge(3, d3.edge_center(2)?, d3.edge_angle(2)?);
    let d5 = penrose::place_dart_edge(3, d4.edge_center(2)?, d4.edge_angle(2)?);
    let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
    tiles.push(Box::new(d1));
    tiles.push(Box::new(d2));
    tiles.push(Box::new(d3));
    tiles.push(Box::new(d4));
    tiles.push(Box::new(d5));
    Ok(tiles)
}

fn build_vertex2() -> Result<Vec<Box<dyn Drawable>>, i32> {
    let d1 = penrose::Dart::new(0., 0., 0);
    let k1 = penrose::place_kite_edge(2, d1.edge_center(4)?, d1.edge_angle(4)?);
    let k2 = penrose::place_kite_edge(4, k1.edge_center(1)?, k1.edge_angle(1)?);

    let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
    tiles.push(Box::new(d1));
    tiles.push(Box::new(k1));
    tiles.push(Box::new(k2));
    Ok(tiles)
}

fn build_vertex3() -> Result<Vec<Box<dyn Drawable>>, i32> {
    let s5 = (5 as f64).sqrt();
    let phi = (1.+s5)/2.;

    let k1 = penrose::Kite::new(phi, 0., 0);
    let k2 = penrose::place_kite_edge(1, k1.edge_center(4)?, k1.edge_angle(4)?);
    let k3 = penrose::place_kite_edge(1, k2.edge_center(4)?, k2.edge_angle(4)?);
    let k4 = penrose::place_kite_edge(1, k3.edge_center(4)?, k3.edge_angle(4)?);
    let k5 = penrose::place_kite_edge(1, k4.edge_center(4)?, k4.edge_angle(4)?);

    let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
    tiles.push(Box::new(k1));
    tiles.push(Box::new(k2));
    tiles.push(Box::new(k3));
    tiles.push(Box::new(k4));
    tiles.push(Box::new(k5));
    Ok(tiles)
}

fn build_vertex4() -> Result<Vec<Box<dyn Drawable>>, i32> {
    let s5 = (5 as f64).sqrt();
    let phi = (1.+s5)/2.;

    let d1 = penrose::Dart::new(-phi, 0., 0);
    let d2 = penrose::place_dart_edge(3, d1.edge_center(2)?, d1.edge_angle(2)?);
    let k1 = penrose::place_kite_edge(1, d2.edge_center(2)?, d2.edge_angle(2)?);
    let k2 = penrose::place_kite_edge(1, k1.edge_center(4)?, k1.edge_angle(4)?);
    let d3 = penrose::place_dart_edge(3, k2.edge_center(4)?, k2.edge_angle(4)?);

    let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
    tiles.push(Box::new(d1));
    tiles.push(Box::new(d2));
    tiles.push(Box::new(k1));
    tiles.push(Box::new(k2));
    tiles.push(Box::new(d3));
    Ok(tiles)
}

fn build_vertex5() -> Result<Vec<Box<dyn Drawable>>, i32> {
    let s5 = (5 as f64).sqrt();
    let phi = (1.+s5)/2.;

    let k1 = penrose::Kite::new(-1., 0., 0);
    let d1 = penrose::place_dart_edge(4, k1.edge_center(2)?, k1.edge_angle(2)?);
    let k2 = penrose::place_kite_edge(1, d1.edge_center(3)?, d1.edge_angle(3)?);
    let k3 = penrose::place_kite_edge(1, k2.edge_center(4)?, k2.edge_angle(4)?);
    let d2 = penrose::place_dart_edge(2, k3.edge_center(4)?, k3.edge_angle(4)?);

    let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
    tiles.push(Box::new(k1));
    tiles.push(Box::new(d1));
    tiles.push(Box::new(k2));
    tiles.push(Box::new(k3));
    tiles.push(Box::new(d2));
    Ok(tiles)
}

fn build_vertex6() -> Result<Vec<Box<dyn Drawable>>, i32> {
    let s5 = (5 as f64).sqrt();
    let phi = (1.+s5)/2.;
    let d1 = penrose::Dart::new(-phi, 0., 0);
    let k1 = penrose::place_kite_edge(4, d1.edge_center(2)?, d1.edge_angle(2)?);
    let k2 = penrose::place_kite_edge(2, k1.edge_center(3)?, k1.edge_angle(3)?);
    let k3 = penrose::place_kite_edge(4, k2.edge_center(1)?, k2.edge_angle(1)?);
    let k4 = penrose::place_kite_edge(2, k3.edge_center(3)?, k3.edge_angle(3)?);

    let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
    tiles.push(Box::new(d1));
    tiles.push(Box::new(k1));
    tiles.push(Box::new(k2));
    tiles.push(Box::new(k3));
    tiles.push(Box::new(k4));
    Ok(tiles)
}

fn build_vertex7() -> Result<Vec<Box<dyn Drawable>>, i32> {
    let s5 = (5 as f64).sqrt();
    let phi = (1.+s5)/2.;
    let k = (2.+s5) / (1.+s5);
    let p = (10. + (20. as f64).sqrt()).sqrt()/4.;

    let k1 = penrose::Kite::new(k-1.,-p, 108);
    let k2 = penrose::place_kite_edge(3, k1.edge_center(2)?, k1.edge_angle(2)?);
    let d1 = penrose::place_dart_edge(4, k2.edge_center(2)?, k2.edge_angle(2)?);
    let d2 = penrose::place_dart_edge(2, d1.edge_center(3)?, d1.edge_angle(3)?);

    let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
    tiles.push(Box::new(k1));
    tiles.push(Box::new(k2));
    tiles.push(Box::new(d1));
    tiles.push(Box::new(d2));
    Ok(tiles)
}

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    let mut tiles: Vec<Box<dyn Drawable>> = Vec::new();
    // match build_vertex1() {
    //     Ok(t) => tiles = t,
    //     Err(_) => println!("Error building vertex 2"),
    // }
    // match build_vertex2() {
    //     Ok(t) => for o in t { tiles.push(o) },
    //     Err(_) => println!("Error building vertex 2"),
    // }
    // match build_vertex3() {
    //     Ok(t) => for o in t { tiles.push(o) },
    //     Err(_) => println!("Error building vertex 2"),
    // }
    // match build_vertex4() {
    //     Ok(t) => for o in t { tiles.push(o) },
    //     Err(_) => println!("Error building vertex 2"),
    // }
    // match build_vertex5() {
    //     Ok(t) => for o in t { tiles.push(o) },
    //     Err(_) => println!("Error building vertex 2"),
    // }
    // match build_vertex6() {
    //     Ok(t) => for o in t { tiles.push(o) },
    //     Err(_) => println!("Error building vertex 2"),
    // }
    // match build_vertex7() {
    //     Ok(t) => for o in t { tiles.push(o) },
    //     Err(_) => println!("Error building vertex 2"),
    // }

    // Begin drawing
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(CORNFLOWERBLUE);

    // Draw a purple triangle in the top left half of the window.
    let win = app.window_rect();

    for t in tiles {

        let pts = t.get_points(0., 0., 25.);
        let points = (0..4).map(|i| {
            pt2(pts[i].0, pts[i].1)
        });
        draw.polygon()
            .x(-win.w() * 0.25)
            .color(WHITE)
            .stroke(PINK)
            .stroke_weight(2.)
            .join_miter()
            .points(points);
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
