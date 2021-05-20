# Usage example with `image`
```rust
use image::{io::Reader, DynamicImage, ImageBuffer, ImageOutputFormat};
let kernel: [usize; 15] = [2, 5, 9, 14, 21, 27, 32, 34, 32, 27, 21, 14, 9, 5, 2];
let img = Reader::open("railway.jpg")?.decode()?.to_rgb8();
let (width, height) = (img.width() as usize, img.height() as usize);
let raw = img.into_raw();
let (raw, width, height) = blur(height, width, kernel, raw);
let img = DynamicImage::ImageRgb8(ImageBuffer::from_raw(width as u32, height as u32, raw).unwrap());
let mut writer = File::create("out.png").unwrap();
img.write_to(&mut writer, ImageOutputFormat::Png).unwrap();
```

# License 
Dual licensed under the Unlicense and MIT License.