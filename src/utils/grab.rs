use screenshots::Screen;
use std::ffi::OsString;

pub fn get_host() -> OsString {
    return gethostname::gethostname();
}

pub fn screenshot_desktop()
-> Vec<screenshots::image::ImageBuffer<screenshots::image::Rgba<u8>, Vec<u8>>> {
    let screens = Screen::all().unwrap();
    let mut captures = Vec::new();

    for screen in screens {
        let image = screen.capture().unwrap();
        captures.push(image);
    }

    captures
}
