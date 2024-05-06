import fhe_http_python as py_fhe


def generate_keys():
    key_gen = py_fhe.KeyGenerator()
    client_key = key_gen.get_client_key()
    server_key = key_gen.get_server_key()
    return client_key, server_key


def set_server_key(server_key):
    ops = py_fhe.FheOps()
    ops.set_server_key(server_key)


def encrypt_i64(num: int, client_key):
    fhe_types = py_fhe.FheTypes()
    fhe = py_fhe.Fhe(client_key)
    return fhe.encrypt(fhe_types.from_i64(num))


def decrypt_i64(encrypted_num, client_key):
    fhe_types = py_fhe.FheTypes()
    fhe = py_fhe.Fhe(client_key)
    return fhe_types.to_i64(fhe.decrypt(encrypted_num))


def add_i64(a, b, client_key):
    fhe_types = py_fhe.FheTypes()
    fhe = py_fhe.Fhe(client_key)
    fhe_ops = py_fhe.FheOps()
    encrypted_a = fhe.encrypt(fhe_types.from_i64(a))
    encrypted_b = fhe.encrypt(fhe_types.from_i64(b))
    encrypted_c = fhe_ops.add(encrypted_a, encrypted_b, "Int64")
    return encrypted_c


def sub_i64(a, b, client_key):
    fhe_types = py_fhe.FheTypes()
    fhe = py_fhe.Fhe(client_key)
    fhe_ops = py_fhe.FheOps()
    encrypted_a = fhe.encrypt(fhe_types.from_i64(a))
    encrypted_b = fhe.encrypt(fhe_types.from_i64(b))
    encrypted_c = fhe_ops.sub(encrypted_a, encrypted_b, "Int64")
    return encrypted_c


if __name__ == "__main__":
    client_key, server_key = generate_keys()
    set_server_key(server_key)
    encrypted_a = encrypt_i64(5, client_key)
    encrypted_b = encrypt_i64(6, client_key)
    encrypted_c = add_i64(5, 6, client_key)
    c = decrypt_i64(encrypted_c, client_key)
    encrypted_d = sub_i64(5, 6, client_key)
    d = decrypt_i64(encrypted_d, client_key)
    assert c == 11 and d == -1
    print("Integration test passed")
