import unittest
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


def encrypt(num: int, client_key, data_type: str = "Int64"):
    serailizer = py_fhe.Serializer()
    fhe_value = create_fhe_value_type(data_type)
    fhe = py_fhe.Fhe(client_key)
    return fhe.encrypt(serailizer.from_i64(num), fhe_value)


def decrypt(encrypted_num, client_key, data_type: str = "Int64"):
    serailizer = py_fhe.Serializer()
    fhe_value = create_fhe_value_type(data_type)
    fhe = py_fhe.Fhe(client_key)
    return serailizer.to_i64(fhe.decrypt(encrypted_num, fhe_value))


def perform_binary_operation(op_func, encrypted_a, encrypted_b, value_type: str):
    fhe_ops = py_fhe.FheOps()
    fhe_value = create_fhe_value_type(value_type)
    return op_func(fhe_ops, encrypted_a, encrypted_b, fhe_value)


def perform_unary_operation(op_func, encrypted_a, value_type: str):
    fhe_ops = py_fhe.FheOps()
    fhe_value = create_fhe_value_type(value_type)
    return op_func(fhe_ops, encrypted_a, fhe_value)


def add_encrypted_i64(encrypted_a, encrypted_b):
    return perform_binary_operation(
        lambda ops, a, b, v: ops.add(a, b, v), encrypted_a, encrypted_b, "Int64"
    )


def sub_encrypted_i64(encrypted_a, encrypted_b):
    return perform_binary_operation(
        lambda ops, a, b, v: ops.sub(a, b, v), encrypted_a, encrypted_b, "Int64"
    )


def mul_encrypted_i64(encrypted_a, encrypted_b):
    return perform_binary_operation(
        lambda ops, a, b, v: ops.mul(a, b, v), encrypted_a, encrypted_b, "Int64"
    )


def div_encrypted_i64(encrypted_a, encrypted_b):
    return perform_binary_operation(
        lambda ops, a, b, v: ops.mul(a, b, v), encrypted_a, encrypted_b, "Int64"
    )


def and_encrypted_i64(encrypted_a, encrypted_b):
    return perform_binary_operation(
        lambda ops, a, b, v: ops.and_(a, b, v), encrypted_a, encrypted_b, "Int64"
    )


def or_encrypted_i64(encrypted_a, encrypted_b):
    return perform_binary_operation(
        lambda ops, a, b, v: ops.or_(a, b, v), encrypted_a, encrypted_b, "Int64"
    )


def xor_encrypted_i64(encrypted_a, encrypted_b):
    return perform_binary_operation(
        lambda ops, a, b, v: ops.xor(a, b, v), encrypted_a, encrypted_b, "Int64"
    )


def shr_encrypted_u64(encrypted_a, encrypted_b):
    return perform_binary_operation(
        lambda ops, a, b, v: ops.shr(a, b, v), encrypted_a, encrypted_b
    )


def shl_encrypted_u64(encrypted_a, encrypted_b):
    return perform_binary_operation(
        lambda ops, a, b, v: ops.shl(a, b, v), encrypted_a, encrypted_b
    )


def not_encrypted_i64(encrypted_a):
    return perform_unary_operation(
        lambda ops, a, v: ops.not_(a, v), encrypted_a, "Int64"
    )


def neg_encrypted_i64(encrypted_a):
    return perform_unary_operation(
        lambda ops, a, v: ops.neg(a, v), encrypted_a, "Int64"
    )


class TestFheOps(unittest.TestCase):

    @classmethod
    def setUpClass(cls):
        cls.client_key, cls.server_key = generate_keys()
        set_server_key(cls.server_key)

    def encrypt_a_and_b(self, a: int, b: int, data_type: str = "Int64"):
        encrypted_a = encrypt(a, self.client_key, data_type)
        encrypted_b = encrypt(b, self.client_key, data_type)
        return encrypted_a, encrypted_b

    def test_add_encrypted_i64(self):
        print("Testing add_encrypted_i64")
        encrypted_a, encrypted_b = self.encrypt_a_and_b(a=12343, b=1243)
        encrypted_c = add_encrypted_i64(encrypted_a, encrypted_b)
        c = decrypt(encrypted_c, self.client_key, "Int64")
        self.assertEqual(c, 12343 + 1243)
        print("Test add_encrypted_i64 passed")

    def test_sub_encrypted_i64(self):
        print("Testing sub_encrypted_i64")
        encrypted_a, encrypted_b = self.encrypt_a_and_b(a=12343, b=1243)
        encrypted_d = sub_encrypted_i64(encrypted_a, encrypted_b)
        d = decrypt(encrypted_d, self.client_key, "Int64")
        self.assertEqual(d, 12343 - 1243)

    def test_mul_encrypted_i64(self):
        print("Testing mul_encrypted_i64")
        encrypted_a, encrypted_b = self.encrypt_a_and_b(a=12343, b=1243)
        encrypted_d = mul_encrypted_i64(encrypted_a, encrypted_b)
        d = decrypt(encrypted_d, self.client_key, "Int64")
        self.assertEqual(d, 12343 * 1243)

    def test_div_encrypted_i64(self):
        print("Testing div_encrypted_i64")
        encrypted_a, encrypted_b = self.encrypt_a_and_b(a=1024, b=256)
        encrypted_d = div_encrypted_i64(encrypted_a, encrypted_b)
        d = decrypt(encrypted_d, self.client_key, "Int64")
        self.assertEqual(d, 1024 // 256)

    def test_and_encrypted_i64(self):
        print("Testing and_encrypted_i64")
        encrypted_a, encrypted_b = self.encrypt_a_and_b(a=12343, b=1243)
        encrypted_d = and_encrypted_i64(encrypted_a, encrypted_b)
        d = decrypt(encrypted_d, self.client_key, "Int64")
        self.assertEqual(d, 12343 & 1243)

    def test_or_encrypted_i64(self):
        print("Testing or_encrypted_i64")
        encrypted_a, encrypted_b = self.encrypt_a_and_b(a=12343, b=1243)
        encrypted_d = or_encrypted_i64(encrypted_a, encrypted_b)
        d = decrypt(encrypted_d, self.client_key, "Int64")
        self.assertEqual(d, 12343 | 1243)

    def test_xor_encrypted_i64(self):
        print("Testing xor_encrypted_i64")
        encrypted_a, encrypted_b = self.encrypt_a_and_b(a=12343, b=1243)
        encrypted_d = xor_encrypted_i64(encrypted_a, encrypted_b)
        d = decrypt(encrypted_d, self.client_key, "Int64")
        self.assertEqual(d, 12343 ^ 1243)

    def test_shr_encrypted_u64(self):
        print("Testing shr_encrypted_u64")
        encrypted_a, encrypted_b = self.encrypt_a_and_b(a=1234, b=2, data_type="Uint64")
        encrypted_d = shr_encrypted_u64(encrypted_a, encrypted_b)
        d = decrypt(encrypted_d, self.client_key, "Uint64")
        self.assertEqual(d, 1234 >> 2)

    def test_shl_encrypted_u64(self):
        print("Testing shl_encrypted_u64")
        encrypted_a, encrypted_b = self.encrypt_a_and_b(a=1234, b=2, data_type="Uint64")
        encrypted_d = shl_encrypted_u64(encrypted_a, encrypted_b)
        d = decrypt(encrypted_d, self.client_key, "Uint64")
        self.assertEqual(d, 1234 << 2)

    def test_not_encrypted_i64(self):
        print("Testing not_encrypted_i64")
        encrypted_a = encrypt(234, self.client_key, "Int64")
        encrypted_d = not_encrypted_i64(encrypted_a)
        d = decrypt(encrypted_d, self.client_key, "Int64")
        self.assertEqual(d, ~234)

    def test_neg_encrypted_i64(self):
        print("Testing neg_encrypted_i64")
        encrypted_a = encrypt(234, self.client_key, "Int64")
        encrypted_d = neg_encrypted_i64(encrypted_a)
        d = decrypt(encrypted_d, self.client_key, "Int64")
        self.assertEqual(d, -234)


if __name__ == "__main__":
    unittest.main()
