import json
import fastapi
import uvicorn
from fastapi import Body
from pydantic import BaseModel
import fhe_http_python as py_fhe
from integration import (
    generate_keys,
    set_server_key,
    encrypt_i64,
    decrypt_i64,
    add_i64,
    sub_i64,
)


app = fastapi.FastAPI()


class Addition(BaseModel):
    a: list[str]
    b: list[str]
    server_key: list[str]


@app.post("/")
async def post_request(data: Addition = Body(...)):
    server_key = data.server_key
    a = data.a
    b = data.b
    set_server_key(server_key)
    fhe_ops = py_fhe.FheOps()
    encrypted_c = fhe_ops.add(a, b, "Int64")
    return {"result": encrypted_c}


if __name__ == "__main__":
    uvicorn.run("server_test:app", host="localhost", port=8000, reload=True)
