import json
import fastapi
import uvicorn
from fastapi import Body
from pydantic import BaseModel
import fhe_http_python as py_fhe
from integration import set_server_key, add_encrypted_i64


app = fastapi.FastAPI()


class Addition(BaseModel):
    a: str
    b: str
    server_key: str


@app.post("/")
async def post_request(data: Addition = Body(...)):
    print("Received request")
    data_json = json.loads(data.model_dump_json())
    a = py_fhe.decode_fhe_value(data_json["a"])
    b = py_fhe.decode_fhe_value(data_json["b"])
    server_key = py_fhe.decode_fhe_value(data_json["server_key"])
    set_server_key(server_key)
    encrypted_c = add_encrypted_i64(a, b)
    encoded_c = py_fhe.encode_fhe_value(encrypted_c)
    return {"result": encoded_c, "status": "success"}


if __name__ == "__main__":
    uvicorn.run("server_test:app", host="localhost", port=8000, reload=True)
