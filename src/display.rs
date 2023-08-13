use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

// /**
//  *  get_screen
//  *  gets a reference to the viewport pixel array. The event loop watches this array. When data is written to this "screen" the event loop will render the pixels to the window.
//  * */
// pub fn get_screen(&self) -> &[u8] {
//     self.viewport.frame_mut()
// }

/**
 * get_viewport
 * returns a reference to the viewport "Pixels"
 */
// pub fn get_viewport(&self) -> Pixels {
//     self.viewport
// }

// pub fn get_window(&self) -> &Window {
//     &self.window
// }

// pub fn get_event_loop(&self) -> &EventLoop<()> {
//     &self.event_loop
// }

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
 * build_pixels
 * builds the pixel screen - a canvas for rendering pixels
 */
pub fn build_pixels(window: &Window, width: u32, height: u32) -> Result<Pixels, Error> {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);

    let viewport = Pixels::new(width, height, surface_texture)?;
    Ok(viewport)
}
