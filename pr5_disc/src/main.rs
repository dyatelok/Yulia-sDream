use rand::{thread_rng,Rng,rngs::ThreadRng};
use euler::{Vec2,vec2,Vec3,vec3,Mat3,Mat4};
use raylib::prelude::*;

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
    /*if R.len() == 4 {
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
        return (vec3!(x0,y0,z0),r);
    }*/
    panic!("method can't handle this amount of points");
}

fn welzl(P : Vec<Vec3>, R : Vec<Vec3>) -> (Vec3,f32,Vec<Vec3>) {
    if P.len() == 0 || R.len() == 3 {
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
    vec3!(rng.gen::<f32>(),rng.gen::<f32>(),0.0/*,rng.gen::<f32>()*/)
}

fn main() {
    let screen = vec2!(1000.0,1000.0);
    let origin = vec2!( 250.0, 250.0);
    let scale = 500.0;
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
    //let D = trivial(P.clone());
    println!("{}",D.1);

    rl.set_target_fps(10);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_circle_lines((origin.x + D.0.x * scale) as i32,(origin.y + D.0.y * scale) as i32,D.1 * scale as f32,Color::WHITE);
        for i in 0..n {
            d.draw_circle((origin.x + P[i].x * scale) as i32,(origin.y + P[i].y * scale) as i32,3.0,Color::RED);
        }
        for i in 0..D.2.len() {
            d.draw_circle((origin.x + D.2[i].x * scale) as i32,(origin.y + D.2[i].y * scale) as i32,5.0,Color::GREEN);
        }
    }
}
