use bin_utils::{ReadFixed, WriteFixed};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mac_parser::MACAddress;

fn criterion_benchmark(c: &mut Criterion) {
    let mac_bytes = [0x00, 0x80, 0x41, 0x13, 0x37, 0xff];
    c.bench_function("read_mac", |b| {
        b.iter(|| {
            let _ = MACAddress::from_bytes(black_box(&mac_bytes));
        })
    });
    let mac = <[u8; 6] as Into<MACAddress>>::into(mac_bytes);
    c.bench_function("write_mac", |b| {
        b.iter(|| {
            let _ = mac.to_bytes();
        })
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
