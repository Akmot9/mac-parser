use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mac_parser::MACAddress;
use scroll::{Pread, Pwrite};

fn criterion_benchmark(c: &mut Criterion) {
    let mac_bytes = [0x00, 0x80, 0x41, 0x13, 0x37, 0xff];
    c.bench_function("read_mac", |b| {
        b.iter(|| {
            let _ = mac_bytes.pread::<MACAddress>(0);
        })
    });
    let mac = <[u8; 6] as Into<MACAddress>>::into(mac_bytes);
    let mut buf = [0x00; 6];
    c.bench_function("write_mac", |b| {
        b.iter(|| {
            let _ = buf.pwrite(black_box(mac), 0);
        })
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
