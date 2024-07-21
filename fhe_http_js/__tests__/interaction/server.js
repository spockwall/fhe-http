const fheHttp = require("fhe_http_js");
const express = require("express");
const bodyParser = require("body-parser");
const app = express();

app.use(bodyParser.json({ limit: "5000mb" }));
app.use(bodyParser.urlencoded({ extended: true }));
app.use(bodyParser.json());

//print("Received request")
//    data_type = py_fhe.create_fhe_type("Int64")
//    data_json = json.loads(data.model_dump_json())
//    encrypted_a = py_fhe.decode_fhe_value(data_json["a"])
//    encrypted_b = py_fhe.decode_fhe_value(data_json["b"])
//    server_key = py_fhe.decode_fhe_value(data_json["server_key"])
//    set_server_key(server_key)
//    fhe_ops = py_fhe.FheOps()
//    encrypted_c = fhe_ops.add(encrypted_a, encrypted_b, data_type)
//    encoded_c = py_fhe.encode_fhe_value(encrypted_c)
//    return {"result": encoded_c, "status": "success"}

// route
app.get("/", (req, res) => {
    res.send("Hello World");
});
app.post("/", async (req, res) => {
    let data = req.body;
    let base64 = new fheHttp.Base64();
    let fheOps = new fheHttp.FheOps();
    let encrypted_a = base64.decodeFheValue(data["a"]);
    let encrypted_b = base64.decodeFheValue(data["b"]);
    let server_key = base64.decodeFheValue(data["server_key"]);
    fheHttp.setServerKey(server_key);
    let encrypted_c = fheOps.add(encrypted_a, encrypted_b, "Int64");
    let encoded_c = base64.encodeFheValue(encrypted_c);
    let result = { result: encoded_c, status: "success" };
    res.json(result);
});

app.listen(3000, () => {
    console.log("Server is running on port 3000");
});
