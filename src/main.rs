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

    // Polígono 3
    let polygon3 = vec![
        [377, 249], [411, 197], [436, 249]
    ];

    // Polígono 4
    let polygon4 = vec![
        [413, 177], [448, 159], [502, 88], [553, 53], [535, 36], [676, 37], [660, 52],
        [750, 145], [761, 179], [672, 192], [659, 214], [615, 214], [632, 230], [580, 230],
        [597, 215], [552, 214], [517, 144], [466, 180]
    ];

    // Polígono 5 
    let polygon5 = vec![
        [682, 175], [708, 120], [735, 148], [739, 170]
    ];

    // Dibujar polígono 1
    framebuffer.set_current_color(yellow);
    framebuffer.draw_filled_polygon(polygon1.clone());
    framebuffer.set_current_color(white);
    framebuffer.polygon(polygon1.clone());
    
    // Decidí incluir los demas poligonos como feature para que se pueda ejecutar cada poligono
    // independientemente, al usar: cargo run --features "polygon-n" donde n es el numero que se
    // desea ejecutar. Se pueden concatenar poligonos.

    #[cfg(feature = "polygon-2")] {
        // Dibujar polígono 2
        framebuffer.set_current_color(blue);
        framebuffer.draw_filled_polygon(polygon2.clone());
        framebuffer.set_current_color(white);
        framebuffer.polygon(polygon2.clone());
    }

    #[cfg(feature = "polygon-3")] {
        // Dibujar polígono 3
        framebuffer.set_current_color(red);
        framebuffer.draw_filled_polygon(polygon3.clone());
        framebuffer.set_current_color(white);
        framebuffer.polygon(polygon3.clone());
    }

    #[cfg(feature = "polygon-4")] {
        // Dibujar polígono 4
        framebuffer.set_current_color(green);
        framebuffer.draw_filled_polygon(polygon4.clone());
        framebuffer.set_current_color(white);
        framebuffer.polygon(polygon4.clone());

        // Dibujar agujero
        framebuffer.set_current_color(framebuffer.background_color);
        framebuffer.draw_filled_polygon(polygon5.clone());
    }
     
        
    framebuffer.write_to_bmp("out.bmp").unwrap();
}

