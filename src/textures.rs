use std::io::Cursor;

use image::{self, GenericImageView, ImageBuffer, ImageFormat, Rgba, RgbaImage};

pub mod bytes {
    pub const UNDISCOVERED: &[u8] = include_bytes!("assets/sprites/undiscovered.png");
    pub const OPENED: &[u8] = include_bytes!("assets/sprites/opened.png");
    pub const FLAGGED: &[u8] = include_bytes!("assets/sprites/flagged.png");
    pub const MINE: &[u8] = include_bytes!("assets/sprites/mine.png");

    pub mod numbers {
        pub const ONE: &[u8] = include_bytes!("assets/sprites/number-1.png");
        pub const TWO: &[u8] = include_bytes!("assets/sprites/number-2.png");
        pub const THREE: &[u8] = include_bytes!("assets/sprites/number-3.png");
        pub const FOUR: &[u8] = include_bytes!("assets/sprites/number-4.png");
        pub const FIVE: &[u8] = include_bytes!("assets/sprites/number-5.png");
        pub const SIX: &[u8] = include_bytes!("assets/sprites/number-6.png");
        pub const SEVEN: &[u8] = include_bytes!("assets/sprites/number-7.png");
        pub const EIGHT: &[u8] = include_bytes!("assets/sprites/number-8.png");
    }
}

// Stitch the textures together into a texture atlas.
pub fn stitch() -> (u16, u16, ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let textures = [
        bytes::OPENED,
        bytes::FLAGGED,
        bytes::MINE,
        bytes::UNDISCOVERED,
        bytes::numbers::ONE,
        bytes::numbers::TWO,
        bytes::numbers::THREE,
        bytes::numbers::FOUR,
        bytes::numbers::FIVE,
        bytes::numbers::SIX,
        bytes::numbers::SEVEN,
        bytes::numbers::EIGHT,
    ];

    let mut buffer: RgbaImage = ImageBuffer::new(53 * textures.len() as u32, 53);
    for i in 0..textures.len() {
        let image = image::load(Cursor::new(&textures[i]), ImageFormat::Png).unwrap();
        image.view(0, 0, 53, 53);
        image::imageops::overlay(&mut buffer, &image, i as u32 * 53, 0);
    }

    // return the atlas as a byte slice, together with the width and height
    (
        buffer.width() as u16,
        buffer.height() as u16,
        buffer.to_owned(),
    )
}
