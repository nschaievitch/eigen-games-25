use image::{imageops::FilterType, GenericImageView, GrayImage, ImageBuffer};
use base64::{Engine as _, engine::general_purpose};
use eyre::Result;

pub fn load_image(path: String) -> Result<Vec<u8>> {
    // load and resize image
    let img = image::open(path)?;
    let resized = img.resize_exact(32, 32, FilterType::Triangle);
    let grayscale = resized.grayscale();

    // flatten array
    // Create a flat array of u8 values
    let mut pixel_array = Vec::with_capacity(32 * 32);
    
    for y in 0..32 {
        for x in 0..32 {
            let pixel = grayscale.get_pixel(x, y);
            // Just take the first channel since grayscale has identical R/G/B values
            pixel_array.push(pixel[0]);
        }
    }

    Ok(pixel_array)
}

/// Saves a u8 array as a 32x32 grayscale image
pub fn save_image(array: &[u8], output_path: String) -> Result<()> {
    // Ensure we have the right number of pixels (or at least can form a square image)
    let pixel_count = array.len();
    let side_length = (pixel_count as f64).sqrt() as u32;
    
    if side_length * side_length != pixel_count as u32 {
        println!("Warning: Array length {} is not a perfect square", pixel_count);
        println!("Creating a {}x{} image (using {} pixels)", side_length, side_length, side_length * side_length);
    }
    
    // Create a new grayscale image
    let mut img: GrayImage = ImageBuffer::new(side_length, side_length);
    
    // Fill the image with the array data
    for (i, &pixel) in array.iter().enumerate().take((side_length * side_length) as usize) {
        let x = (i as u32) % side_length;
        let y = (i as u32) / side_length;
        img.put_pixel(x, y, image::Luma([pixel]));
    }
    
    // Save the image
    img.save(output_path)?;
    
    Ok(())
}