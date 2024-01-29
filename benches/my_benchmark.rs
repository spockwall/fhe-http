
use std::ops::Add;

use tfhe::shortint::parameters::PARAM_MULTI_BIT_MESSAGE_2_CARRY_2_GROUP_3_KS_PBS;

use tfhe::{ConfigBuilder, generate_keys, set_server_key, ClientKey, CompressedServerKey};
use tfhe::{FheUint8, FheUint16, FheUint32, FheUint64, FheUint128, FheUint256};
use tfhe::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};


fn arithmatic_benchmark(c: &mut Criterion) {

    let config = ConfigBuilder::with_custom_parameters(PARAM_MULTI_BIT_MESSAGE_2_CARRY_2_GROUP_3_KS_PBS, None).build();

    let client_key= ClientKey::generate(config);
    let compressed_server_key = CompressedServerKey::new(&client_key);

    let gpu_key = compressed_server_key.decompress_to_gpu();

    let clear_a = 27u8;
    let clear_b = 128u8;

    let cyphera = FheUint8::encrypt(clear_a, &client_key);
    let cypherb = FheUint8::encrypt(clear_b, &client_key);

    set_server_key(gpu_key);

    c.bench_function("Addition", |b| b.iter(|| FheUint8::add(cyphera.clone(), &cypherb)));
    // c.bench_function("encrypt_uint16", |b| b.iter(|| FheUint16::encrypt(black_box(123) as u8, &gpu_key)));
    // c.bench_function("encrypt_uint32", |b| b.iter(|| FheUint32::encrypt(black_box(123) as u8, &gpu_key)));
    // c.bench_function("encrypt_uint64", |b| b.iter(|| FheUint64::encrypt(black_box(123) as u8, &gpu_key)));
    // c.bench_function("encrypt_uint128", |b| b.iter(|| FheUint128::encrypt(black_box(123) as u8, &gpu_key)));
    // c.bench_function("encrypt_uint256", |b| b.iter(|| FheUint256::encrypt(black_box(123) as u8, &gpu_key)));
}

// fn decrypt_benckmark(c: &mut Criterion) {
//     let config = ConfigBuilder::default().build();

//     let client_key= ClientKey::generate(config);

    
//     let cypher8 = FheUint8::encrypt(123 as u8, &client_key);
//     let cypher16 = FheUint16::encrypt(123 as u8, &client_key);
//     let cypher32 = FheUint32::encrypt(123 as u8, &client_key);
//     let cypher64 = FheUint64::encrypt(123 as u8, &client_key);
//     let cypher128 = FheUint128::encrypt(123 as u8, &client_key);
//     let cypher256 = FheUint256::encrypt(123 as u8, &client_key);

//     c.bench_function("decrypt_uint8", |b| b.iter(|| -> u8 {FheUint8::decrypt(&cypher8, &client_key)}));
//     c.bench_function("decrypt_uint16", |b| b.iter(|| -> u8 {FheUint16::decrypt(&cypher16, &client_key)}));
//     c.bench_function("decrypt_uint32", |b| b.iter(|| -> u8 {FheUint32::decrypt(&cypher32, &client_key)}));
//     c.bench_function("decrypt_uint64", |b| b.iter(|| -> u8 {FheUint64::decrypt(&cypher64, &client_key)}));
//     c.bench_function("decrypt_uint128", |b| b.iter(|| -> u8 {FheUint128::decrypt(&cypher128, &client_key)}));
//     c.bench_function("decrypt_uint256", |b| b.iter(|| -> u8 {FheUint256::decrypt(&cypher256, &client_key)}));
// }

criterion_group!(benches, arithmatic_benchmark);
criterion_main!(benches);