#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use gaussian_blur::blur;
    use image::{io::Reader, ImageError};
    use test::Bencher;
    #[bench]
    fn bench_image(b: &mut Bencher) -> Result<(), ImageError> {
        let kernel: [usize; 15] = [2, 5, 9, 14, 21, 27, 32, 34, 32, 27, 21, 14, 9, 5, 2];
        let img = Reader::open("test_image.jpg")?.decode()?.to_rgb8();
        let (width, height) = (img.width() as usize, img.height() as usize);
        let raw = img.into_raw();
        b.iter(|| {
            blur(height, width, kernel, raw.clone())
        });
        Ok(())
    }
}
