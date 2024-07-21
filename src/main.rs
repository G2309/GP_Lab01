mod color;
mod framebuffer;

use color::Color;
use framebuffer::Framebuffer;

fn main() {
    let red = Color::new(255, 0, 0);
    let green = Color::new(0, 255, 0);
    let black = Color::new(0, 0, 0);

    let mut fb = Framebuffer::new(3, 3, black, red);

    fb.point(1, 1, green);
    fb.point(-1, -1, green); 
    fb.point(4, 4, green); 

    fb.print();

    fb.clear();
    fb.print();
    
    if let Some(color) = fb.get_pixel(1, 1) {
        color.print();
    } else {
        println!("Pixel fuera de rango");
    }
}

