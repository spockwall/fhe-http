const fheHttp = require("fhe_http_js");
const express = require("express");
const bodyParser = require("body-parser");
const cors = require("cors");
const app = express();

const corsOptions = {
    origin: "*", //(https://your-client-app.com)
    optionsSuccessStatus: 200,
};
function uint8ArrayToBase64(uint8Array) {
    const buffer = Buffer.from(uint8Array);
    const base64String = buffer.toString("base64");
    return base64String;
}

function base64ToUint8Array(base64String) {
    const buffer = Buffer.from(base64String, "base64");
    const uint8Array = new Uint8Array(buffer);
    return uint8Array;
}

app.use(cors(corsOptions));
app.use(bodyParser.json({ limit: "5000mb" }));
app.use(bodyParser.urlencoded({ extended: true }));
app.use(bodyParser.json());

// route
app.get("/", (req, res) => {
    res.send("Hello World");
});

app.post("/", async (req, res) => {
    let data = req.body;
    let fheOps = new fheHttp.FheOps();
    let encrypted_a = base64ToUint8Array(data["ciphertext1"]);
    let encrypted_b = base64ToUint8Array(data["ciphertext2"]);
    let server_key = base64ToUint8Array(data["serverKey"]);
    fheHttp.setServerKey(server_key);
    let encrypted_c = fheOps.add(encrypted_a, encrypted_b, "Uint64");
    let encoded_c = uint8ArrayToBase64(encrypted_c);
    let result = { result: encoded_c, status: "success" };
    res.json(result);
});

app.listen(3001, () => {
    console.log("Server is running on port 3001");
});
