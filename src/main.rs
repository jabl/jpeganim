//! A demonstration using glutin to provide events and glium for drawing the Ui.
//!
//! Note that the `glium` crate is re-exported via the `conrod::backend::glium` module.

#[macro_use]
extern crate conrod;

mod support;
mod imgloader;

extern crate find_folder;
extern crate image;
//use conrod;
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};
//use support;
//use std;

struct AnimState {
    speed: u32, // Time between successive frames in milliseconds
    direction_forward: bool, // Should the animation go forwards or backwards
    cur_image: usize, // Index of image currently shown
}

    // The initial width and height in "points".
    const WIN_W: u32 = support::WIN_W;
    const WIN_H: u32 = support::WIN_H;

    pub fn main() {

        // Build the window.
        let display = glium::glutin::WindowBuilder::new()
            .with_vsync()
            .with_dimensions(WIN_W, WIN_H)
            .with_title("Conrod with glium!")
            .build_glium()
            .unwrap();


        // Construct our `Ui`.
        let mut ui = conrod::UiBuilder::new([WIN_W as f64, WIN_H as f64]).theme(support::theme()).build();

        // The `widget::Id` of each widget instantiated in `support::gui`.
        let ids = support::Ids::new(ui.widget_id_generator());

        // Add a `Font` to the `Ui`'s `font::Map` from file.
        let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(font_path).unwrap();

        let imgs = imgloader::img_load(std::path::Path::new("."));

        // The animation state
        let mut anim_state = AnimState {
            speed: 128,  // About 8 FPS
            direction_forward: true,
            cur_image: 0,
        };

        // Convert an image to a texture, upload it to GPU
        fn img2tex(display: &glium::Display, rgba_image: &image::RgbaImage) -> glium::texture::Texture2d {
            let image_dimensions = rgba_image.dimensions();
            let r = rgba_image.clone();
            let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(r.into_raw(), image_dimensions);
            let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
            texture
        }

        //let image_map = support::image_map(&ids, load_rust_logo(&display));

        let mut image_map = conrod::image::Map::new();
        let cur_image_id = image_map.insert(img2tex(&display, &imgs[anim_state.cur_image]));


        // A demonstration of some app state that we want to control with the conrod GUI.
        let mut app = support::DemoApp::new(cur_image_id);


        // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
        // for drawing to the glium `Surface`.
        //
        // Internally, the `Renderer` maintains:
        // - a `backend::glium::GlyphCache` for caching text onto a `glium::texture::Texture2d`.
        // - a `glium::Program` to use as the shader program when drawing to the `glium::Surface`.
        // - a `Vec` for collecting `backend::glium::Vertex`s generated when translating the
        // `conrod::render::Primitive`s.
        // - a `Vec` of commands that describe how to draw the vertices.
        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

        let mut nframe = 0;

        // Start the loop:
        //
        // - Render the current state of the `Ui`.
        // - Update the widgets via the `support::gui` fn.
        // - Poll the window for available events.
        // - Repeat.
        let mut event_loop = support::EventLoop::new();
        'main: loop {

            // Handle all events.
            for event in event_loop.next(&display) {

                // Use the `winit` backend feature to convert the winit event to a conrod one.
                if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
                    ui.handle_event(event);
                    event_loop.needs_update();
                }

                match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                    glium::glutin::Event::Closed =>
                        break 'main,
                    _ => {},
                }
            }

            // Instantiate a GUI demonstrating every widget type provided by conrod.
            support::gui(&mut ui.set_widgets(), &ids, &mut app);

            // Draw the `Ui`.
            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }

            // How many frames to we draw before we change image?
            let frames_per_img = anim_state.speed / support::UPDATE_INTERVAL;
            if nframe % frames_per_img == 0 {
                if anim_state.direction_forward {
                    anim_state.cur_image += 1;
                }
                else {
                    if anim_state.cur_image == 0 {
                        anim_state.cur_image = imgs.len() - 1;
                    }
                    else {
                        anim_state.cur_image -= 1;
                    }
                }
                anim_state.cur_image %= imgs.len();
                //println!("cur_image {} of {}", anim_state.cur_image, imgs.len());
                image_map.replace(cur_image_id, img2tex(&display, &imgs[anim_state.cur_image]));
                ui.needs_redraw();
            }
            nframe = (nframe + 1) % frames_per_img;
        }
    }
