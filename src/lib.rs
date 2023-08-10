use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

/*
 * build_window
 * builds the window for rendering pixels
 */
pub fn build_window(width: u32, height: u32, event_loop: &EventLoop<()>) -> Window {
    let size = LogicalSize::new((width) as f64, (height) as f64);
    WindowBuilder::new()
        .with_title("chip 8")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap()
}

/*
 * build_pixel_screen
 * builds the pixel screen - a canvas for rendering pixels
 */
pub fn build_pixel_screen(window: &Window, width: u32, height: u32) -> Result<Pixels, Error> {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);

    let viewport = Pixels::new(width, height, surface_texture)?;
    Ok(viewport)
}

pub fn decode(instruction: u8) -> bool {
    // println!("Decoding");
    false
}
pub fn execute(command: bool) -> bool {
    // println!("Executing");
    false
}
