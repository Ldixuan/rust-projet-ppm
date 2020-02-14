extern crate ppm;

use std::path::Path;
use std::fs;

#[macro_use]
extern crate bencher;

use bencher::Bencher;

// bench function

fn bench_new_with_file(b: &mut Bencher) {
    let mut pixels = Vec::new();
    pixels.push(ppm::Pixels::new(7, 91, 43));
    pixels.push(ppm::Pixels::new(14, 32, 56));
    pixels.push(ppm::Pixels::new(23, 43, 32));
    let image = ppm::Image::new(pixels, 1, 3, "P3".to_string(), 91);
    image.save(Path::new("bench_new_with_file_image.ppm")).unwrap();

    b.iter(|| {
        ppm::Image::new_with_file(Path::new("bench_new_with_file_image.ppm")).unwrap();
    });

    fs::remove_file(Path::new("bench_new_with_file_image.ppm")).unwrap();
}

fn bench_save(b: &mut Bencher) {
    let mut pixels = Vec::new();
    pixels.push(ppm::Pixels::new(7, 91, 43));
    pixels.push(ppm::Pixels::new(14, 32, 56));
    pixels.push(ppm::Pixels::new(23, 43, 32));
    let image = ppm::Image::new(pixels, 1, 3, "P3".to_string(), 91);

    b.iter(|| image.save(Path::new("bench_save_image.ppm")).unwrap());

    fs::remove_file(Path::new("bench_save_image.ppm")).unwrap();

}

fn bench_read_ppm_libc(b : &mut Bencher){
    let mut pixels = Vec::new();
    pixels.push(ppm::Pixels::new(7, 91, 43));
    pixels.push(ppm::Pixels::new(14, 32, 56));
    pixels.push(ppm::Pixels::new(23, 43, 32));
    let image = ppm::Image::new(pixels, 1, 3, "P3".to_string(), 91);
    image.save(Path::new("bench_read_image.ppm")).unwrap();

    b.iter(|| {unsafe{
        ppm::readPPM_libc("bench_read_image.ppm".to_string());
    }});

    fs::remove_file(Path::new("bench_read_image.ppm")).unwrap();

}

fn bench_write_ppm_libc(b : &mut Bencher){
    let mut pixels = Vec::new();
    pixels.push(ppm::Pixels::new(7, 91, 43));
    pixels.push(ppm::Pixels::new(14, 32, 56));
    pixels.push(ppm::Pixels::new(23, 43, 32));
    let image = ppm::Image::new(pixels, 1, 3, "P3".to_string(), 91);

    b.iter(|| {
        unsafe{
            ppm::writePPM_libc("bench_write_image.ppm".to_string(), &image);
        }
    });

    fs::remove_file(Path::new("bench_write_image.ppm")).unwrap();
}

benchmark_group!(benches, bench_new_with_file, bench_save, bench_read_ppm_libc, bench_write_ppm_libc);
benchmark_main!(benches);

