const fheHttp = require("fhe-http-js");
const express = require("express");
const bodyParser = require("body-parser");
const app = express();

app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

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
app.post("/", async (req, res) => {
    const { data } = req.body;

    console.log(data);
    //const result = await fheHttp.interaction(data);
    //res.json(result);
});
