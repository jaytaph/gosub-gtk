use std::time::Duration;
use gtk::gdk_pixbuf::{Colorspace, Pixbuf};
use gtk::Image;
use reqwest::blocking::Client;

pub fn download_favicon(url: &str) -> Option<Image> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:89.0) Gecko/20100101 Firefox/89.0")
        .timeout(Duration::from_secs(5))
        .build().ok()?;

    let response = client.get(format!("{}{}", url, "/favicon.ico")).send().ok()?;
    let buf = response.bytes().ok()?;

    let Ok(img) = image::load_from_memory(&buf) else {
        println!("Failed to load favicon into buffer (image)");
        return None;
    };

    // Convert to RGBA format if not already
    let rgba_image = img.to_rgba8();
    let (width, height) = rgba_image.dimensions();
    let pixels = rgba_image.into_raw();

    // Create a Pixbuf from the raw RGBA data
    let pixbuf = Pixbuf::from_mut_slice(
        pixels,
        Colorspace::Rgb,
        true, // Has alpha channel
        8,    // Bits per channel
        width as i32,
        height as i32,
        width as i32 * 4,
    );

    Some(Image::from_pixbuf(Some(&pixbuf)))
}
