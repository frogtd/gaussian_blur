/// Rotates image 90 degrees clockwise.
pub fn rotate_90_clockwise(height: usize, width: usize, vector: Vec<u8>) -> Vec<u8> {
    let (new_width, _new_height) = (height, width);
    let mut output_image = vec![0; vector.len()];

    for y in 0..height {
        for x in 0..width {
            let (new_x, new_y) = (new_width - 1 - y, x);
            let input_pixel = y * width + x;
            let output_pixel = new_y * new_width + new_x;
            for z in 0..3 {
                output_image[output_pixel * 3 + z] = vector[input_pixel * 3 + z];
            }
        }
    }

    output_image
}

/// Rotates image 90 degrees counterclockwise.
pub fn rotate_90_counterclockwise(height: usize, width: usize, vector: Vec<u8>) -> Vec<u8> {
    let (new_width, _new_height) = (height, width);
    let mut output_image = vec![0; vector.len()];
    for y in 0..height {
        for x in 0..width {
            let (new_x, new_y) = (y, _new_height - 1 - x);
            let input_pixel = y * width + x;
            let output_pixel = new_y * new_width + new_x;
            for z in 0..3 {
                output_image[output_pixel * 3 + z] = vector[input_pixel * 3 + z];
            }
        }
    }
    output_image
}

/// Blurs image consisting of a Vec<u8> of RGB values, horizontally.
/// Good kernels:
/// ```text
/// [27, 201, 27]
/// [1, 14, 62, 102, 62, 14, 1]
/// [1, 2, 7, 17, 31, 45, 51, 45, 31, 17, 7, 2, 1]
/// [2, 5, 9, 14, 21, 27, 32, 34, 32, 27, 21, 14, 9, 5, 2]
/// [1, 1, 2, 3, 6, 8, 12, 15, 19, 23, 25, 26, 25, 23, 19, 15, 12, 8, 6, 3, 2, 1, 1]
/// ```
pub fn horz_gauss_blur<const N: usize>(
    _height: usize,
    width: usize,
    middle: usize,
    kernel: [usize; N],
    raw: &[u8],
) -> Vec<u8> {
    let horz_vec: Vec<_> = raw
        .chunks(width * 3)
        .enumerate()
        .map(|(_y, buffer)| {
            let a = buffer[..middle * 3]
                .chunks(3)
                .map(|value| [value[0], value[1], value[2]]);

            let b = (middle..(width - middle - 1)).map(move |x| {
                let mut adding_red = 0;
                let mut adding_green = 0;
                let mut adding_blue = 0;
                unsafe {
                    for (index, z) in kernel.iter().enumerate() {
                        let offset = (middle + x - index) * 3;
                        adding_red += *buffer.get_unchecked(offset) as usize * *z;
                        adding_green += *buffer.get_unchecked(offset + 1) as usize * *z;
                        adding_blue += *buffer.get_unchecked(offset + 2) as usize * *z;
                    }
                }

                adding_red /= 256;
                adding_green /= 256;
                adding_blue /= 256;
                [adding_red as u8, adding_green as u8, adding_blue as u8]
            });
            let c = buffer[(width - middle - 1) * 3..]
                .chunks(3)
                .map(|value| [value[0], value[1], value[2]]);
            a.chain(b).chain(c)
        })
        .flatten()
        .flatten()
        .collect();
    horz_vec
}


/// Blurs image consisting of a Vec<u8> of RGB values.
/// Good kernels:
/// ```text
/// [27, 201, 27]
/// [1, 14, 62, 102, 62, 14, 1]
/// [1, 2, 7, 17, 31, 45, 51, 45, 31, 17, 7, 2, 1]
/// [2, 5, 9, 14, 21, 27, 32, 34, 32, 27, 21, 14, 9, 5, 2]
/// [1, 1, 2, 3, 6, 8, 12, 15, 19, 23, 25, 26, 25, 23, 19, 15, 12, 8, 6, 3, 2, 1, 1]
/// ```
pub fn blur<const N: usize>(
    height: usize,
    width: usize,
    kernel: [usize; N],
    raw: Vec<u8>,
) -> (Vec<u8>, usize, usize) {
    let middle = kernel.len() / 2;
    let raw = horz_gauss_blur(height, width, middle, kernel, &raw);
    let raw = rotate_90_clockwise(height, width, raw);
    let (width, height) = (height, width);
    let raw = horz_gauss_blur(height, width, middle, kernel, &raw);
    let raw = rotate_90_counterclockwise(height, width, raw);
    let (width, height) = (height, width);
    (raw, width, height)
}
