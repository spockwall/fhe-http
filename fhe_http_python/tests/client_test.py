# client.py
import json
import requests
import fhe_http_python as py_fhe
from integration import generate_keys, decrypt_i64


def send_post_request(url):
    header = json.loads(py_fhe.create_fhe_header("123"))
    client_key, server_key = generate_keys()
    data = {"a": 123123123, "b": 123}
    data_str = json.dumps(data)
    encrypt_json = py_fhe.encrypt_fhe_body(["a", "b"], data_str, client_key)
    encrypt_json = json.loads(encrypt_json)
    data["server_key"] = server_key
    response = requests.post(url, json=data, headers=header)
    response = response.json()
    encrypted_c = response["result"]
    c = decrypt_i64(encrypted_c, client_key)
    assert c == 123123246


if __name__ == "__main__":
    server_url = "http://localhost:8000"
    send_post_request(server_url)
