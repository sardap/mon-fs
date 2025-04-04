use std::{any::type_name, io::Read};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mon_fs_box::{
    box_mon::BoxMon, box_mon_full::BoxMonFull, box_mon_lite::BoxMonLite, file_pc::FilePc,
    mon_field::ByteCount, pc::PC,
};

fn write_and_pc<T: BoxMon>(data: Vec<u8>) -> PC<T> {
    let mut file_pc = FilePc::new();

    file_pc.add_file_raw("test", data).unwrap();

    file_pc.as_pc().unwrap()
}

fn read_entire_pc<T: BoxMon>(mut pc: PC<T>) {
    let mut data = Vec::new();
    pc.read_to_end(&mut data).unwrap();
}

fn bench_pc<T: BoxMon>(c: &mut Criterion) {
    let mut input_data = vec![0; PC::<T>::byte_count()];
    for i in 0..PC::<T>::byte_count() {
        input_data[i] = (i % 255) as u8;
    }

    let pc_name = format!("PC({})", type_name::<T>());

    let pc = write_and_pc::<T>(input_data.clone());

    let mut group = c.benchmark_group(pc_name.clone());
    group.sample_size(2000);

    group.bench_function("write to file", |b| {
        b.iter(|| write_and_pc::<T>(black_box(input_data.clone())))
    });

    group.bench_function("read entire", |b| {
        b.iter(|| read_entire_pc(black_box(pc.clone())))
    });

    group.finish();
}

fn bench(c: &mut Criterion) {
    bench_pc::<BoxMonLite>(c);
    bench_pc::<BoxMonFull>(c);
}

criterion_group!(benches, bench);
criterion_main!(benches);
