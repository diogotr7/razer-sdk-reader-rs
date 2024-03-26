use criterion::{black_box, criterion_group, criterion_main, Criterion};
use razer_sdk_reader_lib::{color_provider::ColorProvider, keyboard::ChromaKeyboard};

fn get_colors(keyboard: &ChromaKeyboard, colors: &mut [u32]) {
    keyboard.get_colors(colors);
}

fn get_colors_single(keyboard: &ChromaKeyboard, colors: &mut [u32]) {
    for i in 0..ChromaKeyboard::WIDTH * ChromaKeyboard::HEIGHT {
        colors[i] = keyboard.get_color(i);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut colors = [0u32; 132];
    let buff = [0u8; 40088];
    let keyboard = unsafe { buff.align_to::<ChromaKeyboard>().1.get_unchecked(0) };

    c.bench_function("get_colors", |b| {
        b.iter(|| get_colors(black_box(&keyboard), &mut colors[..]))
    });

    c.bench_function("get_colors_single", |b| {
        b.iter(|| get_colors_single(black_box(keyboard), &mut colors[..]))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
