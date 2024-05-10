import fhe_http_python as py_fhe
import json

a = py_fhe.create_fhe_header("123")
b = json.loads(a)
print(a)
print(b["fhe-method"])
