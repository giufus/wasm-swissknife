use image::{DynamicImage, ImageBuffer, Rgba};
use wasm_bindgen::prelude::*;

use crate::art::generate_ascii;

mod art;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    //alert(&format!("Hello, {}!", name));
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub struct FilteredImage {
    width: u32,
    height: u32,
    cells: Vec<u8>,
    chars: Vec<u8>,
    chars_size: Vec<u32>,
}

#[wasm_bindgen]
impl FilteredImage {
    pub fn render(&self) -> String {
        todo!()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u8 {
        self.cells.as_ptr()
    }

    pub fn chars(&self) -> *const u8 {
        self.chars.as_ptr()
    }

    pub fn chars_size(&self) -> *const u32 {
        self.chars_size.as_ptr()
    }

    pub fn chars_length(&self) -> u32 {
        self.chars.len() as u32
    }

    pub fn new(width: u32, height: u32) -> Self {
        FilteredImage {
            width,
            height,
            cells: vec![0 as u8; (width * height * 4) as usize],
            chars: vec![],
            chars_size: vec![0 as u32; 2],
        }
    }

    pub fn art(&mut self, rate: u32, _array: &[u8]) {
        let mut dyn_img =
            ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(self.width, self.height, _array.to_vec())
                .map(|i| DynamicImage::ImageRgba8(i))
                .expect("Failed to create image from raw data");

        let characters = " .:-=+*#%@".to_string();

        generate_ascii(
            dyn_img,
            rate,
            characters,
            &mut self.chars,
            &mut self.chars_size,
        );
    }
}
