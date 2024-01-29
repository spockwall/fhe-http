use tfhe::{ConfigBuilder, generate_keys, set_server_key, FheUint8, ClientKey, CompressedServerKey};
use tfhe::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn test_gpu() {
    let config = ConfigBuilder::default().build();

    let client_key= ClientKey::generate(config);
    let compressed_server_key = CompressedServerKey::new(&client_key);

    let gpu_key = compressed_server_key.decompress_to_gpu();

    let clear_a = 27u8;
    let clear_b = 128u8;

    let a = FheUint8::encrypt(clear_a, &client_key);
    let b = FheUint8::encrypt(clear_b, &client_key);

    //Server-side

    set_server_key(gpu_key);
    let result = a + b;

    //Client-side
    let decrypted_result: u8 = result.decrypt(&client_key);

    let clear_result = clear_a + clear_b;

    assert_eq!(decrypted_result, clear_result);
}

fn test_cpu() {
    let config = ConfigBuilder::default().build();

    // Client-side
    let (client_key, server_key) = generate_keys(config);

    let clear_a = 27u8;
    let clear_b = 128u8;

    let a = FheUint8::encrypt(clear_a, &client_key);
    let b = FheUint8::encrypt(clear_b, &client_key);

    //Server-side
    set_server_key(server_key);
    let result = a + b;

    //Client-side
    let decrypted_result: u8 = result.decrypt(&client_key);

    let clear_result = clear_a + clear_b;

    assert_eq!(decrypted_result, clear_result);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| test_gpu()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);