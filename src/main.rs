mod color;
mod framebuffer;
mod bitmap;

use color::Color;
use framebuffer::FrameBuffer;

fn main() {
    let width = 800;
    let height = 600;

    let mut framebuffer = FrameBuffer::new(width, height);

    // Configuraciones de color para cada polígono
    let yellow = Color::new(255, 255, 0);
    let white = Color::new(255, 255, 255);
    let blue = Color::new(0, 0, 255);
    let red = Color::new(255, 0, 0);
    let green = Color::new(0, 255, 0);

    // Polígono 1
    let polygon1 = vec![
        [165, 380], [185, 360], [180, 330], [207, 345], [233, 330],
        [230, 360], [250, 380], [220, 385], [205, 410], [193, 383]
    ];

    // Polígono 2
    let polygon2 = vec![
        [321, 335], [288, 286], [339, 251], [374, 302]
    ];

    // Dibujar polígono 1
    framebuffer.set_current_color(yellow);
    framebuffer.draw_filled_polygon(polygon1.clone());
    framebuffer.set_current_color(white);
    framebuffer.polygon(polygon1.clone());

    #[cfg(feature = "polygon-2")] {
        // Dibujar polígono 2
        framebuffer.set_current_color(blue);
        framebuffer.draw_filled_polygon(polygon2.clone());
        framebuffer.set_current_color(white);
        framebuffer.polygon(polygon2.clone());
    }
     
        
    framebuffer.write_to_bmp("out.bmp").unwrap();
}

