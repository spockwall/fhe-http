
use std::ops::Add;

use tfhe::shortint::parameters::PARAM_MULTI_BIT_MESSAGE_2_CARRY_2_GROUP_3_KS_PBS;

use tfhe::{ConfigBuilder, generate_keys, set_server_key, ClientKey, CompressedServerKey};
use tfhe::{FheUint8, FheUint16, FheUint32, FheUint64, FheUint128, FheUint256};
use tfhe::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};


fn addition_benchmark(c: &mut Criterion) {

    let config = ConfigBuilder::with_custom_parameters(PARAM_MULTI_BIT_MESSAGE_2_CARRY_2_GROUP_3_KS_PBS, None).build();

    let client_key= ClientKey::generate(config);
    let compressed_server_key = CompressedServerKey::new(&client_key);

    let gpu_key = compressed_server_key.decompress_to_gpu();

    let clear_a = 27u8;
    let clear_b = 128u8;

    let cyphera8 = FheUint8::encrypt(clear_a, &client_key);
    let cypherb8 = FheUint8::encrypt(clear_b, &client_key);
    let cyphera16 = FheUint16::encrypt(clear_a, &client_key);
    let cypherb16 = FheUint16::encrypt(clear_b, &client_key);
    let cyphera32 = FheUint32::encrypt(clear_a, &client_key);
    let cypherb32 = FheUint32::encrypt(clear_b, &client_key);
    let cyphera64 = FheUint64::encrypt(clear_a, &client_key);
    let cypherb64 = FheUint64::encrypt(clear_b, &client_key);
    let cyphera128 = FheUint128::encrypt(clear_a, &client_key);
    let cypherb128 = FheUint128::encrypt(clear_b, &client_key);
    let cyphera256 = FheUint256::encrypt(clear_a, &client_key);
    let cypherb256 = FheUint256::encrypt(clear_b, &client_key);

    set_server_key(gpu_key);

    c.bench_function("CDUA FheUint8 Addition", |b| b.iter(|| FheUint8::add(cyphera8.clone(), &cypherb8)));
    c.bench_function("CDUA FheUint16 Addition", |b| b.iter(|| FheUint16::add(cyphera16.clone(), &cypherb16)));
    c.bench_function("CDUA FheUint32 Addition", |b| b.iter(|| FheUint32::add(cyphera32.clone(), &cypherb32)));
    c.bench_function("CDUA FheUint64 Addition", |b| b.iter(|| FheUint64::add(cyphera64.clone(), &cypherb64)));
    c.bench_function("CDUA FheUint128 Addition", |b| b.iter(|| FheUint128::add(cyphera128.clone(), &cypherb128)));
    c.bench_function("CDUA FheUint256 Addition", |b| b.iter(|| FheUint256::add(cyphera256.clone(), &cypherb256)));

}


criterion_group!(benches, addition_benchmark);
criterion_main!(benches);