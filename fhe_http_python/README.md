# FHE-HTTP-Python

## Introduce
This package offers a Python interface for using [tfhe-rs](https://github.com/zama-ai/tfhe-rs). Additionally, a zk-experimental version is supported, enabling the server to verify encrypted values before computation begins. For more details about FHE, please refer to the  [tfhe-rs documentation](https://docs.zama.ai/tfhe-rs).

## Environment Required
- rust^1.77.1 stable
- maturin
- python^3.10

## How to use
### Supported
- operations: 
  - `add`, `sub`, `mul`, `div`, `rem`, `and`, `or`, `xor`, `shr`, `shl`, `not`, `and`, `neg`
  - `neg` is temporarily not supported in assembly code execution.
- Type: 
  - `Int64`
  - `Uint64`
- Type (zk-experimental): 
  - `ProvenInt64`
  - `ProvenUint64` 

### Example code
```python=
import fhe_http_python.fhe_http_python as py_fhe

# initialize keys
key_gen = py_fhe.KeyGenerator()
key_gen.init_keys()
client_key = key_gen.get_client_key()
server_key = key_gen.get_server_key()

# Server Side:
# set server key for ciphertext computation
sk_setter = py_fhe.ServerKeySetter()
decompressed_server_key = sk_setter.decompress_server_key(server_key)
sk_setter.set_server_key(decompressed_server_key)

# Client Side:
# use Fhe module to encrypt
fhe = py_fhe.Fhe(client_key)

# Client Side:
# encryt value
serailizer = py_fhe.Serializer()
value_type = py_fhe.create_fhe_value_type("Int64")
encrypted_a = fhe.encrypt(serailizer.from_i64(123), value_type)
encrypted_b = fhe.encrypt(serailizer.from_i64(456), value_type)

# Server Side:
# using FheOps module to have ciphertext computation
fhe_ops = py_fhe.FheOps()
encrypted_c = fhe_ops.add(encrypted_a, encrypted_b, value_type)

# Client side:
# decrypt computation result
decrypted_c = fhe.decrypt(encrypted_c, value_type)
c = serailizer.to_i64(decrypted_c)
assert c == 123 + 456
```

### Example code for zk-experimental
```python=
import fhe_http_python.fhe_http_python as py_fhe

# initialize keys
key_gen = py_fhe.KeyGenerator()
key_gen.init_keys()
client_key = key_gen.get_client_key()
server_key = key_gen.get_server_key()
public_key = key_gen.get_public_key()

# initialize zk params
public_zk_params = py_fhe.get_public_zk_params(msg=2, carry=2)

# Server Side:
# set server key for ciphertext computation
sk_setter = py_fhe.ServerKeySetter()
decompressed_server_key = sk_setter.decompress_server_key(server_key)
sk_setter.set_server_key(decompressed_server_key)

# Client Side:
# use Fhe module to encrypt
fhe = py_fhe.Fhe(client_key, public_key)

# Client Side:
# encryt value with public_zk_params
serailizer = py_fhe.Serializer()
proven_fhe_type = py_fhe.create_proven_fhe_value_type("ProvenInt64")
encrypted_a = fhe.proven_encrypt(serailizer.from_i64(123), proven_fhe_type, public_zk_params)
encrypted_b = fhe.proven_encrypt(serailizer.from_i64(456), proven_fhe_type, public_zk_params)

# Server Side:
# using ProvenFheOps module to have ciphertext computation
proven_ops = py_fhe.ProvenFheOps()
encrypted_c = proven_ops.add(encrypted_a, encrypted_b, proven_fhe_type, public_zk_params, public_key)

# Client Side:
decrypted_c = fhe.decrypt(encrypted_c, proven_fhe_type)
c = serailizer.to_u64(decrypted_c)
assert c == 123 + 456
```

### Example for assembly code execution
```python=
import fhe_http_python.fhe_http_python as py_fhe
from assembler.assembler import Assembler


def generate_keys():
    key_gen = py_fhe.KeyGenerator()
    key_gen.init_keys()
    client_key = key_gen.get_client_key()
    server_key = key_gen.get_server_key()
    return client_key, server_key


def set_server_key(server_key):
    sk_setter = py_fhe.ServerKeySetter()
    decompressed_server_key = sk_setter.decompress_server_key(server_key)
    sk_setter.set_server_key(decompressed_server_key)


def encrypt(num: int, client_key, data_type: str = "Uint64"):
    serailizer = py_fhe.Serializer()
    fhe_value = py_fhe.create_fhe_value_type(data_type)
    fhe = py_fhe.Fhe(client_key)
    return fhe.encrypt(serailizer.from_i64(num), fhe_value)


def decrypt(encrypted_num, client_key, data_type: str = "Uint64"):
    serailizer = py_fhe.Serializer()
    fhe_value = py_fhe.create_fhe_value_type(data_type)
    fhe = py_fhe.Fhe(client_key)
    return serailizer.to_i64(fhe.decrypt(encrypted_num, fhe_value))


def get_asm_code():
    assembler = Assembler()

    @assembler.code_wrapper
    def operation(i, j, two, three):
        a = i + j
        b = a >> two
        c = two << three
        d = c - b
        return d

    return "\n".join(operation.assembly)


if __name__ == "__main__":
    assembly = get_asm_code()
    client_key, server_key = generate_keys()
    set_server_key(server_key)

    encrypted = py_fhe.execute_assembly(
        assembly,
        {
            "i": encrypt(25, client_key),
            "j": encrypt(21, client_key),
            "two": encrypt(2, client_key),
            "three": encrypt(3, client_key),
        },
        py_fhe.create_fhe_value_type("Uint64"),
    )
    decrypted = decrypt(encrypted, client_key)
    print(decrypted)

```
