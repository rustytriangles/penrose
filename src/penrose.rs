use crate::geom::{Point2, pt2, Vector2, vec2, Polygon};

#[derive(PartialEq, Debug)]
pub enum EdgeLength {
    SHORT,
    LONG
}

pub struct Dart {
    pub cx: f64,
    pub cy: f64,
    pub angle: i32,
}

fn edge_index_to_vertex_tuple(e: i32) -> Result<(usize, usize), i32> {
    match e {
        1 => Ok((0 as usize, 1 as usize)),
        2 => Ok((1 as usize, 2 as usize)),
        3 => Ok((2 as usize, 3 as usize)),
        4 => Ok((3 as usize, 0 as usize)),
        _ => Err(e),
    }
}

impl Dart {
    pub fn new(x: f64, y: f64, a: i32) -> Self {
        Self {
            cx: x,
            cy: y,
            angle: (a+360)%360
        }
    }

    pub fn rotate(&self, angle: i32) -> Dart {
        Dart{ cx: self.cx, cy: self.cy, angle: (self.angle + angle)%360 }
    }

    pub fn translate(&self, ox: f64, oy: f64) -> Dart {
        Dart{ cx: (self.cx + ox), cy: (self.cy + oy), angle: self.angle }
    }

    pub fn polygon(&self, xoff: f32, yoff: f32, scale: f32) -> Vec<(f32,f32)> {
        let pts = self.geometry();
        let xoff64 = xoff as f64;
        let yoff64 = yoff as f64;
        let scale64 = scale as f64;

        vec![( (pts[2*0+0]*scale64 + xoff64) as f32, (pts[2*0+1]*scale64 + yoff64) as f32 ),
             ( (pts[2*1+0]*scale64 + xoff64) as f32, (pts[2*1+1]*scale64 + yoff64) as f32 ),
             ( (pts[2*2+0]*scale64 + xoff64) as f32, (pts[2*2+1]*scale64 + yoff64) as f32 ),
             ( (pts[2*3+0]*scale64 + xoff64) as f32, (pts[2*3+1]*scale64 + yoff64) as f32 )]
    }

    pub fn edge_angle(&self, e: i32) -> Result<i32, i32> {
        match e {
            1 => Ok((252 + self.angle)%360),
            2 => Ok(( 36 + self.angle)%360),
            3 => Ok((144 + self.angle)%360),
            4 => Ok((288 + self.angle)%360),
            _ => Err(e),
        }
    }

    pub fn edge_length(&self, e: i32) -> Result<EdgeLength, i32> {
        match e {
            1 => Ok(EdgeLength::SHORT),
            2 => Ok(EdgeLength::LONG),
            3 => Ok(EdgeLength::LONG),
            4 => Ok(EdgeLength::SHORT),
            _ => Err(e),
        }
    }

    pub fn edge_center(&self, e: i32) -> Result<(f64, f64), i32> {
        let (i1, i2) = edge_index_to_vertex_tuple(e)?;
        let pts = self.geometry();
        let x1 = pts[2*i1+0];
        let y1 = pts[2*i1+1];
        let x2 = pts[2*i2+0];
        let y2 = pts[2*i2+1];
        Ok(( (x1+x2)/2., (y1+y2)/2. ))
    }

    pub fn edge_points(&self, e: i32) -> Result<((f64, f64), (f64, f64)), i32> {
        let (i1, i2) = edge_index_to_vertex_tuple(e)?;
        let pts = self.geometry();
        let x1 = pts[2*i1+0];
        let y1 = pts[2*i1+1];
        let x2 = pts[2*i2+0];
        let y2 = pts[2*i2+1];
        Ok(( (x1,y1), (x2,y2) ))
    }

    fn geometry(&self) ->  Box<[f64]> {
        let angle_in_radians = self.angle as f64 * std::f64::consts::PI / 180.;
        let c = angle_in_radians.cos();
        let s = angle_in_radians.sin();
        let s5 = 5_f64.sqrt();
        let phi = (1.+s5)/2.;
        let h = (5.+2.*s5).sqrt()/2.;

        let mut boxed_arr = Box::new([0.; 8]);
        boxed_arr[2*0+0] = self.cx + c*( 0.) - s*( 0.);
        boxed_arr[2*0+1] = self.cy + s*( 0.) + c*( 0.);

        boxed_arr[2*1+0] = self.cx + c*(-0.5) - s*(-h);
        boxed_arr[2*1+1] = self.cy + s*(-0.5) + c*(-h);

        boxed_arr[2*2+0] = self.cx + c*(phi) - s*( 0.);
        boxed_arr[2*2+1] = self.cy + s*(phi) + c*( 0.);

        boxed_arr[2*3+0] = self.cx + c*(-0.5) - s*( h);
        boxed_arr[2*3+1] = self.cy + s*(-0.5) + c*( h);

        boxed_arr
    }
}

pub fn place_dart_edge(e: i32, pt: (f64,f64), edge_angle: i32) -> Dart {
    let s5 = 5_f64.sqrt();
    let phi = (1.+s5)/2.;
    let h = (5.+2.*s5).sqrt()/2.;

    match e {
        1 => {
            let new_angle = (edge_angle + 360 - 252) % 360;
            let c = ((new_angle as f64) * std::f64::consts::PI / 180.).cos();
            let s = ((new_angle as f64) * std::f64::consts::PI / 180.).sin();

            let dx = 0.25;
            let dy = h/2.;
            Dart::new(pt.0 + dx*c - dy*s,
                      pt.1 + dx*s + dy*c,
                      new_angle)
        }
        2 => {
            let new_angle = (edge_angle + 144) % 360;
            let c = ((new_angle as f64) * std::f64::consts::PI / 180.).cos();
            let s = ((new_angle as f64) * std::f64::consts::PI / 180.).sin();

            let dx = 0.25 - phi/2.;
            let dy = h/2.;
            Dart::new(pt.0 + dx*c - dy*s,
                      pt.1 + dx*s + dy*c,
                      new_angle)
        }
        3 => {
            let new_angle = (edge_angle + 36) % 360;
            let c = ((new_angle as f64) * std::f64::consts::PI / 180.).cos();
            let s = ((new_angle as f64) * std::f64::consts::PI / 180.).sin();

            let dx = 0.25 - phi/2.;
            let dy = -h/2.;
            Dart::new(pt.0 + dx*c - dy*s,
                      pt.1 + dx*s + dy*c,
                      new_angle)
        }
        4 => {
            let new_angle = (edge_angle + 252) % 360;
            let c = ((new_angle as f64) * std::f64::consts::PI / 180.).cos();
            let s = ((new_angle as f64) * std::f64::consts::PI / 180.).sin();

            let dx = 0.25;
            let dy = -h/2.;
            Dart::new(pt.0 + dx*c - dy*s,
                      pt.1 + dx*s + dy*c,
                      new_angle)
        }
        _ => {
            Dart::new(0.,0.,0)
        }
    }
}

pub struct Kite {
    pub cx: f64,
    pub cy: f64,
    pub angle: i32,
}
impl Kite {
    pub fn new(x: f64, y: f64, a: i32) -> Self {
        Self {
            cx: x,
            cy: y,
            angle: (a+360)%360
        }
    }

    pub fn rotate(&self, angle: i32) -> Kite {
        return Kite{ cx: self.cx, cy: self.cy, angle: (self.angle + angle)%360 };
    }

    pub fn translate(&self, ox: f64, oy: f64) -> Kite {
        return Kite{ cx: (self.cx + ox), cy: (self.cy + oy), angle: self.angle };
    }

    pub fn polygon(&self, xoff: f32, yoff: f32, scale: f32) -> Vec<(f32,f32)> {
        let pts = self.geometry();
        let xoff64 = xoff as f64;
        let yoff64 = yoff as f64;
        let scale64 = scale as f64;

        vec![( (pts[2*0+0]*scale64 + xoff64) as f32, (pts[2*0+1]*scale64 + yoff64) as f32 ),
             ( (pts[2*1+0]*scale64 + xoff64) as f32, (pts[2*1+1]*scale64 + yoff64) as f32 ),
             ( (pts[2*2+0]*scale64 + xoff64) as f32, (pts[2*2+1]*scale64 + yoff64) as f32 ),
             ( (pts[2*3+0]*scale64 + xoff64) as f32, (pts[2*3+1]*scale64 + yoff64) as f32 )]
    }

    pub fn edge_angle(&self, e: i32) -> Result<i32, i32> {
        match e {
            1 => Ok((324 + self.angle)%360),
            2 => Ok(( 72 + self.angle)%360),
            3 => Ok((108 + self.angle)%360),
            4 => Ok((216 + self.angle)%360),
            _ => Err(e),
        }
    }

    pub fn edge_length(&self, e: i32) -> Result<EdgeLength, i32> {
        match e {
            1 => Ok(EdgeLength::LONG),
            2 => Ok(EdgeLength::SHORT),
            3 => Ok(EdgeLength::SHORT),
            4 => Ok(EdgeLength::LONG),
            _ => Err(e),
        }
    }

    pub fn edge_center(&self, e: i32) -> Result<(f64, f64), i32> {
        let (i1, i2) = edge_index_to_vertex_tuple(e)?;
        let pts = self.geometry();
        let x1 = pts[2*i1+0];
        let y1 = pts[2*i1+1];
        let x2 = pts[2*i2+0];
        let y2 = pts[2*i2+1];
        Ok(( (x1+x2)/2., (y1+y2)/2. ))
    }

    pub fn edge_points(&self, e: i32) -> Result<((f64, f64), (f64, f64)), i32> {
        let (i1, i2) = edge_index_to_vertex_tuple(e)?;
        let pts = self.geometry();
        let x1 = pts[2*i1+0];
        let y1 = pts[2*i1+1];
        let x2 = pts[2*i2+0];
        let y2 = pts[2*i2+1];
        Ok(( (x1,y1), (x2,y2) ))
    }

    fn geometry(&self) ->  Box<[f64]> {
        let angle_in_radians = self.angle as f64 * std::f64::consts::PI / 180.;
        let c = angle_in_radians.cos();
        let s = angle_in_radians.sin();
        let s5 = 5_f64.sqrt();
        let phi = (1.+s5)/2.;
        let h = (5.+2.*s5).sqrt()/2.;

        let mut boxed_arr = Box::new([0.; 8]);
        boxed_arr[2*0+0] = self.cx + c*(-phi) - s*( 0.);
        boxed_arr[2*0+1] = self.cy + s*(-phi) + c*( 0.);

        boxed_arr[2*1+0] = self.cx + c*( 0.5) - s*(-h);
        boxed_arr[2*1+1] = self.cy + s*( 0.5) + c*(-h);

        boxed_arr[2*2+0] = self.cx + c*( 1.  ) - s*( 0.);
        boxed_arr[2*2+1] = self.cy + s*( 1. ) + c*( 0.);

        boxed_arr[2*3+0] = self.cx + c*( 0.5) - s*( h);
        boxed_arr[2*3+1] = self.cy + s*( 0.5) + c*( h);

        boxed_arr
    }
}

pub fn place_kite_edge(e: i32, pt: (f64,f64), edge_angle: i32) -> Kite {
    let s5 = 5_f64.sqrt();
    let phi = (1.+s5)/2.;
    let h = (5.+2.*s5).sqrt()/2.;

    match e {
        1 => {
            let new_angle = (edge_angle + 216) % 360;
            let c = ((new_angle as f64) * std::f64::consts::PI / 180.).cos();
            let s = ((new_angle as f64) * std::f64::consts::PI / 180.).sin();

            let dx = phi/2. - 0.25;
            let dy = h/2.;
            Kite::new(pt.0 + dx*c - dy*s,
                      pt.1 + dx*s + dy*c,
                      new_angle)
        }
        2 => {
            let new_angle = (edge_angle + 360 - 252) % 360;
            let c = ((new_angle as f64) * std::f64::consts::PI / 180.).cos();
            let s = ((new_angle as f64) * std::f64::consts::PI / 180.).sin();

            let dx = -0.75;
            let dy = h/2.;
            Kite::new(pt.0 + dx*c - dy*s,
                      pt.1 + dx*s + dy*c,
                      new_angle)
        }
        3 => {
            let new_angle = (edge_angle + 72) % 360;
            let c = ((new_angle as f64) * std::f64::consts::PI / 180.).cos();
            let s = ((new_angle as f64) * std::f64::consts::PI / 180.).sin();

            let dx = -0.75;
            let dy = -h/2.;
            Kite::new(pt.0 + dx*c - dy*s,
                      pt.1 + dx*s + dy*c,
                      new_angle)
        }
        4 => {
            let new_angle = (edge_angle + 360 - 36) % 360;
            let c = ((new_angle as f64) * std::f64::consts::PI / 180.).cos();
            let s = ((new_angle as f64) * std::f64::consts::PI / 180.).sin();

            let dx = phi/2. - 0.25;
            let dy = -h/2.;
            Kite::new(pt.0 + dx*c - dy*s,
                      pt.1 + dx*s + dy*c,
                      new_angle)
        }
        _ => {
            Kite::new(0.,0.,0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn angle_func(p: ((f64, f64), (f64, f64))) -> i32 {
        let dx = p.1.0 - p.0.0;
        let dy = p.1.1 - p.0.1;
        (360 + (dy.atan2(dx) * 180. / std::f64::consts::PI) as i32) % 360
    }

    fn midpt_func(p: ((f64, f64), (f64, f64))) -> (f64,f64) {
        ((p.0.0 + p.1.0) / 2., (p.0.1 + p.1.1) / 2.)
    }

    fn dist_func(p1: (f64,f64), p2: (f64,f64)) -> f64 {
        let dx = p2.0 - p1.0;
        let dy = p2.1 - p1.1;
        (dx*dx + dy*dy).sqrt()
    }

    // Dart tests
    #[test]
    fn test_dart_new() {
        let d1 = Dart::new(0., 0., 0);
        assert_eq!(d1.cx, 0.);
        assert_eq!(d1.cy, 0.);
        assert_eq!(d1.angle, 0);

        let d2 = Dart::new(0., 0.,-40);
        assert_eq!(d2.cx, 0.);
        assert_eq!(d2.cy, 0.);
        assert_eq!(d2.angle,320);

        let d3 = Dart::new(0., 0.,400);
        assert_eq!(d3.cx, 0.);
        assert_eq!(d3.cy, 0.);
        assert_eq!(d3.angle, 40);
    }

    #[test]
    fn test_dart_angle_internal() {
        let internal_angles = [36, 72, 36, 216];
        let d = Dart::new(0.,0.,0);
        let check_angles = || -> Result<(), i32> {
            assert_eq!((d.edge_angle(1)? + (540 - internal_angles[0]))%360, d.edge_angle(2)?);
            assert_eq!((d.edge_angle(2)? + (540 - internal_angles[1]))%360, d.edge_angle(3)?);
            assert_eq!((d.edge_angle(3)? + (540 - internal_angles[2]))%360, d.edge_angle(4)?);
            assert_eq!((d.edge_angle(4)? + (540 - internal_angles[3]))%360, d.edge_angle(1)?);
            Ok(())
        };
        if let Err(_) = check_angles() {
            assert!(false);
        }
    }

    #[test]
    fn test_dart_angle_0() {
        let d = Dart::new(0.,0.,0);
        let check_angles = || -> Result<(), i32> {
            assert_eq!(d.edge_angle(1)?, 252);
            assert_eq!(d.edge_angle(2)?,  36);
            assert_eq!(d.edge_angle(3)?, 144);
            assert_eq!(d.edge_angle(4)?, 288);
            Ok(())
        };
        if let Err(_) = check_angles() {
            assert!(false);
        }
    }

    #[test]
    fn test_dart_angle_90() {
        let d = Dart::new(0.,0.,90);
        let check_angles = || -> Result<(), i32> {
            assert_eq!(d.edge_angle(1)?, 342);
            assert_eq!(d.edge_angle(2)?, 126);
            assert_eq!(d.edge_angle(3)?, 234);
            assert_eq!(d.edge_angle(4)?,  18);
            Ok(())
        };
        if let Err(_) = check_angles() {
            assert!(false);
        }
    }

    // -270 should be the same as +90
    #[test]
    fn test_dart_angle_m270() {
        let d = Dart::new(0.,0.,-270);
        let check_angles = || -> Result<(), i32> {
            assert_eq!(d.edge_angle(1)?, 342);
            assert_eq!(d.edge_angle(2)?, 126);
            assert_eq!(d.edge_angle(3)?, 234);
            assert_eq!(d.edge_angle(4)?,  18);
            Ok(())
        };
        if let Err(_) = check_angles() {
            assert!(false);
        }
    }

    #[test]
    fn test_dart_angle_180() {
        let d = Dart::new(0.,0.,180);
        let check_angles = || -> Result<(), i32> {
            assert_eq!(d.edge_angle(1)?,  72);
            assert_eq!(d.edge_angle(2)?, 216);
            assert_eq!(d.edge_angle(3)?, 324);
            assert_eq!(d.edge_angle(4)?, 108);
            Ok(())
        };
        if let Err(_) = check_angles() {
            assert!(false);
        }
    }

    // -180 should be the same as +180
    #[test]
    fn test_dart_angle_m180() {
        let d = Dart::new(0.,0.,-180);
        let check_angles = || -> Result<(), i32> {
            assert_eq!(d.edge_angle(1)?,  72);
            assert_eq!(d.edge_angle(2)?, 216);
            assert_eq!(d.edge_angle(3)?, 324);
            assert_eq!(d.edge_angle(4)?, 108);
            Ok(())
        };
        if let Err(_) = check_angles() {
            assert!(false);
        }
    }

    #[test]
    fn test_dart_edge_slopes() {
        let d = Dart::new(0.,0.,0);
        let check_angles = || -> Result<(), i32> {
            assert_eq!(angle_func(d.edge_points(1)?), d.edge_angle(1)?);
            assert_eq!(angle_func(d.edge_points(2)?), d.edge_angle(2)?);
            assert_eq!(angle_func(d.edge_points(3)?), d.edge_angle(3)?);
            assert_eq!(angle_func(d.edge_points(4)?), d.edge_angle(4)?);
            Ok(())
        };
        if let Err(_) = check_angles() {
            assert!(false);
        }
    }

    #[test]
    fn test_dart_edge_lengths() {
        let d = Dart::new(0.,0.,0);
        let check_angles = || -> Result<(), i32> {
            assert_eq!(d.edge_length(1)?, EdgeLength::SHORT);
            assert_eq!(d.edge_length(2)?, EdgeLength::LONG);
            assert_eq!(d.edge_length(3)?, EdgeLength::LONG);
            assert_eq!(d.edge_length(4)?, EdgeLength::SHORT);
            Ok(())
        };
        if let Err(_) = check_angles() {
            assert!(false);
        }
    }

    #[test]
    fn test_dart_edge_centers() {
        let d = Dart::new(0.,0.,0);
        let check_angles = || -> Result<(), i32> {
            assert_eq!(midpt_func(d.edge_points(1)?), d.edge_center(1)?);
            assert_eq!(midpt_func(d.edge_points(2)?), d.edge_center(2)?);
            assert_eq!(midpt_func(d.edge_points(3)?), d.edge_center(3)?);
            assert_eq!(midpt_func(d.edge_points(4)?), d.edge_center(4)?);
            Ok(())
        };
        if let Err(_) = check_angles() {
            assert!(false);
        }
    }

    // Kite tests
    #[test]
    fn test_kite_new() {
        let k1 = Kite::new(0., 0., 0);
        assert_eq!(k1.cx, 0.);
        assert_eq!(k1.cy, 0.);
        assert_eq!(k1.angle, 0);

        let k2 = Kite::new(0., 0.,-40);
        assert_eq!(k2.cx, 0.);
        assert_eq!(k2.cy, 0.);
        assert_eq!(k2.angle,320);

        let k3 = Kite::new(0., 0.,400);
        assert_eq!(k3.cx, 0.);
        assert_eq!(k3.cy, 0.);
        assert_eq!(k3.angle, 40);
    }

    #[test]
    fn test_kite_angle_internal() {
        let internal_angles = [72, 144, 72, 72];
        let k = Kite::new(0.,0.,0);
        let check_angles = || -> Result<(), i32> {
            assert_eq!((k.edge_angle(1)? + (540 - internal_angles[0]))%360, k.edge_angle(2)?);
            assert_eq!((k.edge_angle(2)? + (540 - internal_angles[1]))%360, k.edge_angle(3)?);
            assert_eq!((k.edge_angle(3)? + (540 - internal_angles[2]))%360, k.edge_angle(4)?);
            assert_eq!((k.edge_angle(4)? + (540 - internal_angles[3]))%360, k.edge_angle(1)?);
            Ok(())
        };
        if let Err(_) = check_angles() {
            assert!(false);
        }
    }

    #[test]
    fn test_kite_angle_0() {
        let k = Kite::new(0.,0.,0);
        let check_angles = || -> Result<(), i32> {
            assert_eq!(k.edge_angle(1)?, 324);
            assert_eq!(k.edge_angle(2)?,  72);
            assert_eq!(k.edge_angle(3)?, 108);
            assert_eq!(k.edge_angle(4)?, 216);
            Ok(())
        };
        if let Err(_) = check_angles() {
            assert!(false);
        }
    }

    #[test]
    fn test_kite_angle_90() {
        let k = Kite::new(0.,0.,90);
        let check_angles = || -> Result<(), i32> {
            assert_eq!(k.edge_angle(1)?,  54);
            assert_eq!(k.edge_angle(2)?, 162);
            assert_eq!(k.edge_angle(3)?, 198);
            assert_eq!(k.edge_angle(4)?, 306);
            Ok(())
        };
        if let Err(_) = check_angles() {
            assert!(false);
        }
    }

    // -270 should be the same as +90
    #[test]
    fn test_kite_angle_m270() {
        let k = Kite::new(0.,0.,-270);
        let check_angles = || -> Result<(), i32> {
            assert_eq!(k.edge_angle(1)?,  54);
            assert_eq!(k.edge_angle(2)?, 162);
            assert_eq!(k.edge_angle(3)?, 198);
            assert_eq!(k.edge_angle(4)?, 306);
            Ok(())
        };
        if let Err(_) = check_angles() {
            assert!(false);
        }
    }

    #[test]
    fn test_kite_angle_180() {
        let k = Kite::new(0.,0.,180);
        let check_angles = || -> Result<(), i32> {
            assert_eq!(k.edge_angle(1)?, 144);
            assert_eq!(k.edge_angle(2)?, 252);
            assert_eq!(k.edge_angle(3)?, 288);
            assert_eq!(k.edge_angle(4)?,  36);
            Ok(())
        };
        if let Err(_) = check_angles() {
            assert!(false);
        }
    }

    // -180 should be the same as +180
    #[test]
    fn test_kite_angle_m180() {
        let k = Kite::new(0.,0.,-180);
        let check_angles = || -> Result<(), i32> {
            assert_eq!(k.edge_angle(1)?, 144);
            assert_eq!(k.edge_angle(2)?, 252);
            assert_eq!(k.edge_angle(3)?, 288);
            assert_eq!(k.edge_angle(4)?,  36);
            Ok(())
        };
        if let Err(_) = check_angles() {
            assert!(false);
        }
    }

    #[test]
    fn test_kite_edge_slopes() {
        let k = Kite::new(0.,0.,0);
        let check_angles = || -> Result<(), i32> {
            assert_eq!(angle_func(k.edge_points(1)?), k.edge_angle(1)?);
            assert_eq!(angle_func(k.edge_points(2)?), k.edge_angle(2)?);
            assert_eq!(angle_func(k.edge_points(3)?), k.edge_angle(3)?);
            assert_eq!(angle_func(k.edge_points(4)?), k.edge_angle(4)?);
            Ok(())
        };
        if let Err(_) = check_angles() {
            assert!(false);
        }
    }

    #[test]
    fn test_kite_edge_lengths() {
        let k = Kite::new(0.,0.,0);
        let check_angles = || -> Result<(), i32> {
            assert_eq!(k.edge_length(1)?, EdgeLength::LONG);
            assert_eq!(k.edge_length(2)?, EdgeLength::SHORT);
            assert_eq!(k.edge_length(3)?, EdgeLength::SHORT);
            assert_eq!(k.edge_length(4)?, EdgeLength::LONG);
            Ok(())
        };
        if let Err(_) = check_angles() {
            assert!(false);
        }
    }

    #[test]
    fn test_kite_edge_centers() {
        let k = Kite::new(0.,0.,0);
        let check_angles = || -> Result<(), i32> {
            assert_eq!(midpt_func(k.edge_points(1)?), k.edge_center(1)?);
            assert_eq!(midpt_func(k.edge_points(2)?), k.edge_center(2)?);
            assert_eq!(midpt_func(k.edge_points(3)?), k.edge_center(3)?);
            assert_eq!(midpt_func(k.edge_points(4)?), k.edge_center(4)?);
            Ok(())
        };
        if let Err(_) = check_angles() {
            assert!(false);
        }
    }

    //////////////////////////////////////
    #[test]
    fn test_vertex1() {
        let s5 = 5_f64.sqrt();
        let phi = (1.+s5)/2.;

        let d1 = Dart::new(-phi, 0., 0);
        let check_placements = || -> Result<(), i32> {
            let d2 = place_dart_edge(3, d1.edge_center(2)?, d1.edge_angle(2)?);
            let d3 = place_dart_edge(3, d2.edge_center(2)?, d2.edge_angle(2)?);
            let d4 = place_dart_edge(3, d3.edge_center(2)?, d3.edge_angle(2)?);
            let d5 = place_dart_edge(3, d4.edge_center(2)?, d4.edge_angle(2)?);

            let h = (5.+2.*s5).sqrt()/2.;
            let k = (2.+s5) / (1.+s5);
            let p = (10. + (20. as f64).sqrt()).sqrt()/4.;

            assert!(dist_func((d1.cx, d1.cy), (-phi, 0.)) < 5e-8);
            assert_eq!(d1.angle, 0);

            assert!(dist_func((d2.cx, d2.cy), (-0.5,-h)) < 5e-8);
            assert_eq!(d2.angle, 72);

            assert!(dist_func((d3.cx, d3.cy), (k,-p)) < 5e-8);
            assert_eq!(d3.angle, 144);

            assert!(dist_func((d4.cx, d4.cy), (k,p)) < 5e-8);
            assert_eq!(d4.angle, 216);

            assert!(dist_func((d5.cx, d5.cy), (-0.5,h)) < 5e-8);
            assert_eq!(d5.angle, 288);

            Ok(())
        };
        if let Err(_) = check_placements() {
            assert!(false);
        }
    }

    #[test]
    fn test_vertex2() {
        let d1 = Dart::new(0., 0., 0);
        let check_placements = || -> Result<(), i32> {
            let k1 = place_kite_edge(2, d1.edge_center(4)?, d1.edge_angle(4)?);
            let k2 = place_kite_edge(4, k1.edge_center(1)?, k1.edge_angle(1)?);

            let s5 = 5_f64.sqrt();
            let k = (2.+s5) / (1.+s5);
            let p = (10. + (20. as f64).sqrt()).sqrt()/4.;

            assert!(dist_func((d1.cx, d1.cy), (0., 0.)) < 5e-8);
            assert_eq!(d1.angle, 0);

            assert!(dist_func((k1.cx, k1.cy), (-k, p)) < 5e-8);
            assert_eq!(k1.angle, 36);

            assert!(dist_func((k2.cx, k2.cy), (-k,-p)) < 5e-8);
            assert_eq!(k2.angle, 324);

            Ok(())
        };
        if let Err(_) = check_placements() {
            assert!(false);
        }
    }

    #[test]
    fn test_vertex3() {
        let s5 = 5_f64.sqrt();
        let phi = (1.+s5)/2.;

        let k1 = Kite::new(phi, 0., 0);
        let check_placements = || -> Result<(), i32> {
            let k2 = place_kite_edge(1, k1.edge_center(4)?, k1.edge_angle(4)?);
            let k3 = place_kite_edge(1, k2.edge_center(4)?, k2.edge_angle(4)?);
            let k4 = place_kite_edge(1, k3.edge_center(4)?, k3.edge_angle(4)?);
            let k5 = place_kite_edge(1, k4.edge_center(4)?, k4.edge_angle(4)?);

            let h = (5.+2.*s5).sqrt()/2.;
            let k = (2.+s5) / (1.+s5);
            let p = (10. + (20. as f64).sqrt()).sqrt()/4.;

            assert!(dist_func((k1.cx, k1.cy), (phi, 0.)) < 5e-8);
            assert_eq!(k1.angle, 0);

            assert!(dist_func((k2.cx, k2.cy), (0.5,h)) < 5e-8);
            assert_eq!(k2.angle, 72);

            assert!(dist_func((k3.cx, k3.cy), (-k,p)) < 5e-8);
            assert_eq!(k3.angle, 144);

            assert!(dist_func((k4.cx, k4.cy), (-k,-p)) < 5e-8);
            assert_eq!(k4.angle, 216);

            assert!(dist_func((k5.cx, k5.cy), (0.5,-h)) < 5e-8);
            assert_eq!(k5.angle, 288);

            Ok(())
        };
        if let Err(_) = check_placements() {
            assert!(false);
        }
    }

    #[test]
    fn test_vertex4() {
        let s5 = 5_f64.sqrt();
        let phi = (1.+s5)/2.;

        let d1 = Dart::new(-phi, 0., 0);
        let check_placements = || -> Result<(), i32> {
            let d2 = place_dart_edge(3, d1.edge_center(2)?, d1.edge_angle(2)?);
            let k1 = place_kite_edge(1, d2.edge_center(2)?, d2.edge_angle(2)?);
            let k2 = place_kite_edge(1, k1.edge_center(4)?, k1.edge_angle(4)?);
            let d3 = place_dart_edge(3, k2.edge_center(4)?, k2.edge_angle(4)?);

            let h = (5.+2.*s5).sqrt()/2.;
            let k = (2.+s5) / (1.+s5);
            let p = (10. + (20. as f64).sqrt()).sqrt()/4.;

            assert!(dist_func((d1.cx, d1.cy), (-phi, 0.)) < 5e-8);
            assert_eq!(d1.angle, 0);

            assert!(dist_func((d2.cx, d2.cy), (-0.5,-h)) < 5e-8);
            assert_eq!(d2.angle, 72);

            assert!(dist_func((k1.cx, k1.cy), (k,-p)) < 5e-8);
            assert_eq!(k1.angle, 324);

            assert!(dist_func((k2.cx, k2.cy), (k,p)) < 5e-8);
            assert_eq!(k2.angle, 36);

            assert!(dist_func((d3.cx, d3.cy), (-0.5,h)) < 5e-8);
            assert_eq!(d3.angle, 288);

            Ok(())
        };
        if let Err(_) = check_placements() {
            assert!(false);
        }
    }

    #[test]
    fn test_vertex5() {
        let s5 = 5_f64.sqrt();
        let phi = (1.+s5)/2.;

        let k1 = Kite::new(-1., 0., 0);
        let check_placements = || -> Result<(), i32> {
            let d1 = place_dart_edge(4, k1.edge_center(2)?, k1.edge_angle(2)?);
            let k2 = place_kite_edge(1, d1.edge_center(3)?, d1.edge_angle(3)?);
            let k3 = place_kite_edge(1, k2.edge_center(4)?, k2.edge_angle(4)?);
            let d2 = place_dart_edge(2, k3.edge_center(4)?, k3.edge_angle(4)?);

            let h = (5.+2.*s5).sqrt()/2.;
            let k = (2.+s5) / (1.+s5);
            let p = (10. + (20. as f64).sqrt()).sqrt()/4.;

            assert!(dist_func((k1.cx, k1.cy), (-1., 0.)) < 5e-8);
            assert_eq!(k1.angle, 0);

            assert!(dist_func((d1.cx, d1.cy), (-0.5,-h)) < 5e-8);
            assert_eq!(d1.angle, 324);

            assert!(dist_func((k2.cx, k2.cy), (k,-p)) < 5e-8);
            assert_eq!(k2.angle, 324);

            assert!(dist_func((k3.cx, k3.cy), (k,p)) < 5e-8);
            assert_eq!(k3.angle, 36);

            assert!(dist_func((d2.cx, d2.cy), (-0.5,h)) < 5e-8);
            assert_eq!(d2.angle, 36);

            Ok(())
        };
        if let Err(_) = check_placements() {
            assert!(false);
        }
    }

    #[test]
    fn test_vertex6() {
        let s5 = 5_f64.sqrt();
        let phi = (1.+s5)/2.;

        let d1 = Dart::new(-phi, 0., 0);
        let check_placements = || -> Result<(), i32> {
            let k1 = place_kite_edge(4, d1.edge_center(2)?, d1.edge_angle(2)?);
            let k2 = place_kite_edge(2, k1.edge_center(3)?, k1.edge_angle(3)?);
            let k3 = place_kite_edge(4, k2.edge_center(1)?, k2.edge_angle(1)?);
            let k4 = place_kite_edge(2, k3.edge_center(3)?, k3.edge_angle(3)?);

            let h = (5.+2.*s5).sqrt()/2.;
            let k = (2.+s5) / (1.+s5);
            let p = (10. + (20. as f64).sqrt()).sqrt()/4.;

            assert!(dist_func((d1.cx, d1.cy), (-phi, 0.)) < 5e-8);
            assert_eq!(d1.angle, 0);

            assert!(dist_func((k1.cx, k1.cy), (-0.5,-h)) < 5e-8);
            assert_eq!(k1.angle, 0);

            assert!(dist_func((k2.cx, k2.cy), (k,-p)) < 5e-8);
            assert_eq!(k2.angle, 216);

            assert!(dist_func((k3.cx, k3.cy), (k, p)) < 5e-8);
            assert_eq!(k3.angle, 144);

            assert!(dist_func((k4.cx, k4.cy), (-0.5, h)) < 5e-8);
            assert_eq!(k4.angle, 0);

            Ok(())
        };
        if let Err(_) = check_placements() {
            assert!(false);
        }
    }

    #[test]
    fn test_vertex7() {
        let s5 = 5_f64.sqrt();
        let phi = (1.+s5)/2.;
        let k = (2.+s5) / (1.+s5);
        let p = (10. + (20. as f64).sqrt()).sqrt()/4.;

        let k1 = Kite::new(k-1.,-p, 108);
        let check_placements = || -> Result<(), i32> {
            let k2 = place_kite_edge(3, k1.edge_center(2)?, k1.edge_angle(2)?);
            let d1 = place_dart_edge(4, k2.edge_center(2)?, k2.edge_angle(2)?);
            let d2 = place_dart_edge(2, d1.edge_center(3)?, d1.edge_angle(3)?);

            let h = (5.+2.*s5).sqrt()/2.;

            assert!(dist_func((k1.cx, k1.cy), (k-1.,-p)) < 5e-8);
            assert_eq!(k1.angle, 108);

            assert!(dist_func((k2.cx, k2.cy), (k-1., p)) < 5e-8);
            assert_eq!(k2.angle, 252);

            assert!(dist_func((d1.cx, d1.cy), (-k, p)) < 5e-8);
            assert_eq!(d1.angle, 216);

            assert!(dist_func((d2.cx, d2.cy), (-k,-p)) < 5e-8);
            assert_eq!(d2.angle, 144);

            Ok(())
        };
        if let Err(_) = check_placements() {
            assert!(false);
        }
    }
}
