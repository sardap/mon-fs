use std::io::Read;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mon_fs_box::{file_pc::FilePc, mon_field::ByteCount, pc::PC};

fn write_and_pc(data: Vec<u8>) -> PC {
    let mut file_pc = FilePc::new();

    file_pc.add_file_raw("test", data).unwrap();

    file_pc.as_pc().unwrap()
}

fn read_entire_pc(mut pc: PC) {
    let mut data = Vec::new();
    pc.read_to_end(&mut data).unwrap();
}

fn bench(c: &mut Criterion) {
    let mut input_data = vec![0; PC::byte_count()];
    for i in 0..PC::byte_count() {
        input_data[i] = (i % 255) as u8;
    }

    let pc = write_and_pc(input_data.clone());

    let mut group = c.benchmark_group("pc");
    group.sample_size(1000);

    group.bench_function("write to file pc", |b| {
        b.iter(|| write_and_pc(black_box(input_data.clone())))
    });

    group.bench_function("read entire pc", |b| {
        b.iter(|| read_entire_pc(black_box(pc.clone())))
    });

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
