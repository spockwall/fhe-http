import fhe_http_python as py_fhe


def test_integration():
    key_gen = py_fhe.KeyGenerator(True)
    client_key = key_gen.get_client_key()
    server_key = key_gen.get_server_key()
    key_gen.set_server_key()
    fhe_types = py_fhe.FheTypes()
    fhe = py_fhe.Fhe(bytes(client_key))
    fhe_ops = py_fhe.FheOps()
    a = fhe_types.from_i64(5)
    b = fhe_types.from_i64(6)
    encrypted_a = fhe.encrypt(bytes(a))
    encrypted_b = fhe.encrypt(bytes(b))

    encrypted_c = fhe_ops.add(bytes(encrypted_a), bytes(encrypted_b), "Int64")
    decrypted_c = fhe.decrypt(encrypted_c)
    c = fhe_types.to_i64(bytes(decrypted_c))
    print(f"Decrypted C: {c}")


test_integration()
