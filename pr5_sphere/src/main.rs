use rand::{thread_rng,Rng,rngs::ThreadRng};
use euler::{Vec2,vec2,Vec3,vec3,Mat3,Mat4};
use raylib::prelude::*;

#[derive(Clone)]
struct pt{
    x : f32,
    y : f32,
}

impl pt{
    fn new() -> pt {
        let p = pt{
            x : 0.0,
            y : 0.0
        };
        p
    }
    fn from(x : f32, y : f32) -> pt {
        let p = pt{
            x : x,
            y : y
        };
        p
    }
}

#[derive(Clone)]
struct line{
    start : pt,
    end   : pt,
    col : Color,
}

impl line{
    fn from(pt1 : pt, pt2 : pt, col : Color) -> line {
        let p = line{
            start : pt1,
            end : pt2,
            col : col
        };
        p
    }
}

#[derive(Clone)]
struct line3d{
    start : Vec3,
    end   : Vec3,
    col : Color,
}

impl line3d{
    fn from(pt1 : Vec3, pt2 : Vec3, col : Color) -> line3d {
        let p = line3d{
            start : pt1,
            end : pt2,
            col : col
        };
        p
    }
}

fn to_pt(v : Vec2) -> pt {
    pt{
    x : v.x,
    y : v.y
    }
}

fn func(t : f32) -> Vec3 {
    let mut v : Vec3 = vec3!();
    v.x = (2.0 + (8.0 * t).cos()) * t.cos();
    v.y = (2.0 + (8.0 * t).cos()) * t.sin();
    v.z = (8.0 * t).sin();
    v
    //(2 + Cos[8 u]) Cos[u], (2 + Cos[8 u]) Sin[u], Sin[8 u]
}

fn proj(V : Vec3, c : Camera) -> pt {
    let mut p = pt::from(0.0,0.0);
    let mut m : Mat3 = Mat3::new(0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0);
    let mut v = vec!(0.0_f32; 3);
    m.m00 = c.base1.x;
    m.m01 = c.base2.x;
    m.m02 = c.dir.x;
     v[0] = V.x - c.pos.x;
    m.m10 = c.base1.y;
    m.m11 = c.base2.y;
    m.m12 = c.dir.y;
     v[1] = V.y - c.pos.y;
    m.m20 = c.base1.z;
    m.m21 = c.base2.z;
    m.m22 = c.dir.z;
     v[2] = V.z - c.pos.z;

    let D = m.determinant();
    let mut M = m;
    M.m00 = v[0];
    M.m10 = v[1];
    M.m20 = v[2];
    let d0 = M.determinant();
    let mut M = m;
    M.m01 = v[0];
    M.m11 = v[1];
    M.m21 = v[2];
    let d1 = M.determinant();
    let mut M = m;
    M.m02 = v[0];
    M.m12 = v[1];
    M.m22 = v[2];
    let d2 = M.determinant();
    //println!("{}",m);
    let x0 = d0 / D;
    let x1 = d1 / D;
    let x2 = d2 / D;
    p.x = x0 / x2;
    p.y = x1 / x2;
    p
}

fn proj_line(l : &line3d, c : Camera) -> line {
    line{
        start : proj(l.start,c),
        end   : proj(l.end  ,c),
        col   : l.col
    }
}

#[derive(Clone,Copy)]
struct Camera {
    pos: Vec3,
    dir: Vec3,
    base1 : Vec3,
    base2 : Vec3,
}

fn Cam(t : f32) -> Camera {
    let C = Camera{
        pos   : vec3!( 5.0 * t.sin(), 5.0 * t.cos(), 5.0),
        dir   : (vec3!() - vec3!( 5.0 * t.sin(), 5.0 * t.cos(), 5.0)).normalize() * 1.5 ,
        base1 : (vec3!( 0.0, 0.0, 10.0) - vec3!( 5.0 * t.sin(), 5.0 * t.cos(), 5.0)).normalize(),
        base2 : vec3!()-(vec3!() - vec3!( 5.0 * t.sin(), 5.0 * t.cos(), 5.0)).cross((vec3!( 0.0, 0.0, 10.0) - vec3!( 5.0 * t.sin(), 5.0 * t.cos(), 5.0))).normalize(),
    };
    C

}

fn trivial(R : Vec<Vec3>) -> (Vec3,f32,Vec<Vec3>) {
    if R.len() == 0 {return (vec3!(),0.0,R);}
    if R.len() == 1 {return (R[0],0.0,R);}
    if R.len() == 2 {return ((R[0] + R[1]) / 2.0, (R[0] - R[1]).length() / 2.0,R);}
    if R.len() == 3 {
        let alpha = ((R[1]-R[2]).squared_length() * (R[0]-R[1]).dot(R[0]-R[2])) / (2.0 * ((R[0]-R[1]).cross(R[1]-R[2])).squared_length());
        let beta  = ((R[0]-R[2]).squared_length() * (R[1]-R[0]).dot(R[1]-R[2])) / (2.0 * ((R[0]-R[1]).cross(R[1]-R[2])).squared_length());
        let gamma = ((R[0]-R[1]).squared_length() * (R[2]-R[0]).dot(R[2]-R[1])) / (2.0 * ((R[0]-R[1]).cross(R[1]-R[2])).squared_length()); 
        let C = alpha * R[0] + beta * R[1] + gamma * R[2];
        return (C,(C-R[0]).length(),R);
    }
    if R.len() == 4 {
        let a =  Mat4::new(R[0].x,R[0].y,R[0].z,1.0,
                           R[1].x,R[1].y,R[1].z,1.0,
                           R[2].x,R[2].y,R[2].z,1.0,
                           R[3].x,R[3].y,R[3].z,1.0).determinant();
   
        let Dx = Mat4::new(R[0].x.powi(2)+R[0].y.powi(2)+R[0].z.powi(2),R[0].y,R[0].z,1.0,
                           R[1].x.powi(2)+R[1].y.powi(2)+R[1].z.powi(2),R[1].y,R[1].z,1.0,
                           R[2].x.powi(2)+R[2].y.powi(2)+R[2].z.powi(2),R[2].y,R[2].z,1.0,
                           R[3].x.powi(2)+R[3].y.powi(2)+R[3].z.powi(2),R[3].y,R[3].z,1.0).determinant();
 
        let Dy =-Mat4::new(R[0].x.powi(2)+R[0].y.powi(2)+R[0].z.powi(2),R[0].x,R[0].z,1.0,
                           R[1].x.powi(2)+R[1].y.powi(2)+R[1].z.powi(2),R[1].x,R[1].z,1.0,
                           R[2].x.powi(2)+R[2].y.powi(2)+R[2].z.powi(2),R[2].x,R[2].z,1.0,
                           R[3].x.powi(2)+R[3].y.powi(2)+R[3].z.powi(2),R[3].x,R[3].z,1.0).determinant();

        let Dz = Mat4::new(R[0].x.powi(2)+R[0].y.powi(2)+R[0].z.powi(2),R[0].x,R[0].y,1.0,
                           R[1].x.powi(2)+R[1].y.powi(2)+R[1].z.powi(2),R[1].x,R[1].y,1.0,
                           R[2].x.powi(2)+R[2].y.powi(2)+R[2].z.powi(2),R[2].x,R[2].y,1.0,
                           R[3].x.powi(2)+R[3].y.powi(2)+R[3].z.powi(2),R[3].x,R[3].y,1.0).determinant();

        let c  = Mat4::new(R[0].x.powi(2)+R[0].y.powi(2)+R[0].z.powi(2),R[0].x,R[0].y,R[0].z,
                           R[1].x.powi(2)+R[1].y.powi(2)+R[1].z.powi(2),R[1].x,R[1].y,R[1].z,
                           R[2].x.powi(2)+R[2].y.powi(2)+R[2].z.powi(2),R[2].x,R[2].y,R[2].z,
                           R[3].x.powi(2)+R[3].y.powi(2)+R[3].z.powi(2),R[3].x,R[3].y,R[3].z).determinant();
        let x0 = Dx / (2.0 * a);
        let y0 = Dy / (2.0 * a);
        let z0 = Dz / (2.0 * a);
        let r = (Dx.powi(2) + Dy.powi(2) + Dz.powi(2) - 4.0 * a * c).sqrt() / (2.0 * a.abs());
        return (vec3!(x0,y0,z0),r,R);
    }
    panic!("method can't handle this amount of points");
}

fn welzl(P : Vec<Vec3>, R : Vec<Vec3>) -> (Vec3,f32,Vec<Vec3>) {
    if P.len() == 0 || R.len() == 4 {
        trivial(R)
    } else {
        let mut Pn = P.clone();
        let _ = Pn.remove(0);
        let D = welzl(Pn.clone(),R.clone());
        if (P[0] - D.0).length() <= D.1 {
            return D;
        }
        let mut Rn = R.clone();
        Rn.push(P[0]);
        welzl(Pn,Rn)
    }
}

fn get_vec3(rng : &mut ThreadRng) -> Vec3 {
    vec3!(rng.gen::<f32>()-0.5,rng.gen::<f32>()-0.5,rng.gen::<f32>()-0.5)
}

fn main() {
    let screen = vec2!(1000.0,1000.0);
    let origin = vec2!( 500.0, 500.0);
    let scale = 3000.0;
    let (mut rl, thread) = raylib::init()
        .size(screen.x as i32, screen.y as i32)
        .title("Yulia's Dream pr5")
        .build();

    let mut rng = thread_rng();
    let n : usize = 10;
    let mut P : Vec<Vec3> = Vec::new();
    for _ in 0..n {
        P.push(get_vec3(&mut rng));
    }

    let D = welzl(P.clone(),Vec::new());
    
    //let D = (vec3!(),1.0);
    let C = D.0;
    let Rad = D.1;
    let R = D.2;

    let mut LL : Vec<line3d> = Vec::new();
    let mut w;
    let mut dw = 2.0 * PI as f32 / 20.0;
    let mut r : f32;

    let mut z : f32 = C.z - Rad;
    for k in 0..16 {
        w = 0f32;
        r = (1.0 - ((k as f32 - 7.5) / 7.5).powi(2)).sqrt() * Rad;
        for i in 0..20 {
            LL.push(line3d{
                start : vec3!(C.x+w.sin() * r,C.y+w.cos() * r,z),
                end : vec3!(C.x+(w+dw).sin() * r,C.y+(w+dw).cos() * r,z),
                col : Color::WHITE,
            });
            w += dw;
        }
        z+= 2.0 * Rad / 15.0;
    }


    let mut tau = 0.0;
    let mut c;
    let mut L;
    let mut pr;
    let mut r;
    rl.set_target_fps(10);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        c = Cam(tau);
        tau += 0.1;
        L = Vec::new();
        for l in &LL {
            L.push(proj_line(l,c));
        }
        //Рисуем линии сферы
        for l in &L {
            d.draw_line((origin.y + scale * l.start.y) as i32, (origin.x - scale * l.start.x) as i32, (origin.y + scale * l.end.y) as i32, (origin.x - scale * l.end.x) as i32 ,l.col);
        }

        //d.draw_circle_lines((origin.x + D.0.x * scale) as i32,(origin.y + D.0.y * scale) as i32,D.1 * scale as f32,Color::WHITE);
        for i in 0..n {
            pr = proj(P[i],c);
            r = 400.0 / (c.pos - P[i]).squared_length();
            d.draw_circle((origin.y + pr.y * scale) as i32,(origin.x - pr.x * scale) as i32,r,Color::RED);
        }
        for i in 0..R.len() {
            pr = proj(R[i],c);
            r = 400.0 / (c.pos - R[i]).squared_length();
            d.draw_circle((origin.y + pr.y * scale) as i32,(origin.x - pr.x * scale) as i32,r,Color::GREEN);
        }
    }
}
