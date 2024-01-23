use num::Complex;

use std::fs::File;

use image::png::PNGEncoder;
use image::ColorType;
use std::str::FromStr;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    if args.len() != 5 {
        eprintln!(
            "Usage: {} mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT",
            args[0]
        );
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );

        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");

    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");

    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

// 處理 bounds 字串 並傳回一個 Option<(usize, usize)>
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    // 如果 參數 s 找到 separator 這個字元
    match s.find(separator) {
        // match 會傳回一個 Option<usize> 這個值會是 Some(usize) 或是 None
        None => None,
        Some(index) => {
            // parse 會傳回一個 Option<T> 這個值會是 Some(T) 或是 None
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                // match 會傳回一個 Option<(f64, f64)> 這個值會是 Some((f64, f64)) 或是 None
                (Ok(n1), Ok(n2)) => Some((n1, n2)),
                _ => None,
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im + pixel.1 as f64 * height / bounds.1 as f64,
    }
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);

            pixels[row * bounds.0 + col] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            }
        }
    }
}

fn write_image(filename: &str, pixel: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);

    encoder.encode(&pixel, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

    Ok(())
}
