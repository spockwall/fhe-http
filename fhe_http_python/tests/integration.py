import fhe_http_python as py_fhe
from fhe_http_python import create_fhe_value_type


def generate_keys():
    key_gen = py_fhe.KeyGenerator()
    key_gen.init_keys()
    client_key = key_gen.get_client_key()
    server_key = key_gen.get_server_key()
    return client_key, server_key


def set_server_key(server_key):
    ops = py_fhe.FheOps()
    decompressed_server_key = ops.decompress_server_key(server_key)
    ops.set_server_key(decompressed_server_key)


def encrypt_i64(num: int, client_key):
    serailizer = py_fhe.Serializer()
    fhe_value = create_fhe_value_type("Int64")
    fhe = py_fhe.Fhe(client_key)
    return fhe.encrypt(serailizer.from_i64(num), fhe_value)


def decrypt_i64(encrypted_num, client_key):
    serailizer = py_fhe.Serializer()
    fhe_value = create_fhe_value_type("Int64")
    fhe = py_fhe.Fhe(client_key)
    return serailizer.to_i64(fhe.decrypt(encrypted_num, fhe_value))


def add_encrypted_i64(encrypted_a, encrypted_b):
    fhe_ops = py_fhe.FheOps()
    fhe_value = create_fhe_value_type("Int64")
    encrypted_c = fhe_ops.add(encrypted_a, encrypted_b, fhe_value)
    return encrypted_c


def sub_encrypted_i64(encrypted_a, encrypted_b):
    fhe_ops = py_fhe.FheOps()
    fhe_value = create_fhe_value_type("Int64")
    encrypted_c = fhe_ops.sub(encrypted_a, encrypted_b, fhe_value)
    return encrypted_c


if __name__ == "__main__":
    client_key, server_key = generate_keys()
    print("client key: ", len(client_key))
    print("server key: ", len(server_key))
    set_server_key(server_key)
    encrypted_a = encrypt_i64(234, client_key)
    encrypted_b = encrypt_i64(456, client_key)
    encrypted_c = add_encrypted_i64(encrypted_a, encrypted_b)
    encrypted_d = sub_encrypted_i64(encrypted_a, encrypted_b)
    c = decrypt_i64(encrypted_c, client_key)
    d = decrypt_i64(encrypted_d, client_key)
    assert c == 690 and d == -222
    print("Integration test passed")
