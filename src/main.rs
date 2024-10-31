use glam::Vec3;
use image::{ImageBuffer, Rgb};
const R: f32 = 0.5;
const S_X: f32 = 0.;
const S_Y: f32 = 0.5;
const S_Z: f32 = 5.;

struct MyRgb {
    r: u8,
    g: u8, 
    b: u8
}

fn main () {
    camera(1000, 1.)
}

fn camera(res: i32, fdis: f32) {
    let mut image: Vec<u8> = vec![];
    let p_step = 1./(res as f32);
    // Focal point projects through 1x1 clipping plane with resolution x
    // Focal point is 0, 0.5, fdis
    let f_pos = Vec3::new(0., 0.5, -fdis);
    // Clipping plane is always centered around 0 , 0.5, 0
    let first_x = -0.5 + p_step/2.;
    let first_y = 1. - p_step/2.;
    for j in 0..res {
        for i in 0..res {
            // Top left to bottom right
            let p_x = first_x + (i as f32) * p_step;
            let p_y = first_y - (j as f32) * p_step;
            let p_pos = Vec3::new(p_x, p_y, 0.);
            // Get vector pointing from focal point to center of pixel
            let fp2p = p_pos - f_pos;
            // Normalize the vector so it's a unit distance and put it past the pixel
            let norm = fp2p.normalize();
            let c = raycast(p_pos, norm);
            image.push(c.r);
            image.push(c.b);
            image.push(c.g);
        }
    }  
    let img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(res as u32, res as u32, image.clone())
        .expect("Failed to create ImageBuffer");
    img_buffer.save("output.png").expect("Failed to save image");
    println!("image done!");
}

// returns a color
fn raycast(pos: Vec3, dir: Vec3) -> MyRgb {
    let s_pos = Vec3::new(S_X, S_Y, S_Z);
    let l_pos = s_pos + Vec3::new(4., 3., -8.);

    let mut d = 0.;
    let mut new_p = pos;
    while d < 1000. {
        let d_to_s = sdf_sphere(new_p);
        // If it hits the sphere
        if d_to_s < 0.01 {
            // return the normal of sphere
            let color = (new_p - s_pos).abs().normalize() * 255.;
            let lightness = 0.5 + (new_p - s_pos).normalize().dot(l_pos).min(1.);
            return MyRgb{r: (color.x * lightness) as u8, g: (color.y * lightness) as u8, b: (color.z * lightness) as u8}
        }
        new_p = new_p + dir * d_to_s;
        if new_p.y < 0. {
            return MyRgb{r: 255, g: 255, b:255};
        }
        d += d_to_s;
    }
    // Hits the sky
    return MyRgb{r: 135, g: 206, b: 235};
}

fn sdf_sphere(pos: Vec3) -> f32 {
    let s_pos = Vec3::new(S_X, S_Y, S_Z);
    return s_pos.distance(pos) - R;
}
