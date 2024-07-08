# client.py
import json
import requests
import fhe_http_python as py_fhe
from ..operations import generate_keys, decrypt_i64


def send_post_request(url):
    header = json.loads(py_fhe.create_fhe_header("123"))
    client_key, server_key = generate_keys()
    data = {"a": 123123123, "b": 123}
    data_type = py_fhe.create_fhe_value_type("Int64")
    encrypt_json = py_fhe.encrypt_fhe_body(
        [("a", data_type), ("b", data_type)], data, client_key
    )
    encrypt_json = json.loads(encrypt_json)
    payload_str = py_fhe.set_server_key_to_json(server_key, encrypt_json)
    payload = json.loads(payload_str)
    response = requests.post(url, json=payload, headers=header)
    response = response.json()
    encrypted_c = py_fhe.decode_fhe_value(response["result"])
    c = decrypt_i64(encrypted_c, client_key)
    assert c == 123123246


if __name__ == "__main__":
    server_url = "http://localhost:8000"
    send_post_request(server_url)
