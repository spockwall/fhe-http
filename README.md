# FHE_HTTP

## About
This project is designed as an HTTP extension for an FHE feature or plan, and integrated with existing HTTP services and standards. It allows users to specify their preferred service feature or plan by appending a specific field to the HTTP header when making requests to web services. It will support the web devtools as a packages.

Based on [tfhe-rs](https://github.com/zama-ai/tfhe-rs), this project is developed in Rust to implement functionalities for HTTP interactions and is bound to Python, Node.js, and ES6. The binding allows users to run fhe program in Python and Javascript. The core logic of fhe computation are defined in the fhe_http_core module.

## Fully homomorphic encryption (FHE)
FHE computation enables clients to delegate computation tasks to servers. Clients use a client key which is generated by themselves to encrypt the values. After encryption, clients send the encrypted values and a server key to the servers. Once the server key is received and set by the servers, the servers can execute the FHE computation on the ciphertexts. The computation result remains encrypted and is supposed to be sent back to the clients, who can then decrypt it to obtain the result in plaintext.

## Bindings
This library is primarily implemented in [Rust](./fhe_http_core), with bindings available for multiple languages:

- **[Python](./fhe_http_python)**: Utilizes [PyO3](https://github.com/PyO3/pyo3) library to seamlessly integrate Rust with Python.
- **[Node.js](./fhe_http_js)**: Utilizes [Neon-rs@1.1.0-alpha.0](https://neon-rs.dev/) to provide fast and efficient Node.js bindings for Rust .
- **[Web (ES6)](./fhe_http_web)**: Compiles to WebAssembly using [wasm-pack](https://github.com/rustwasm/wasm-pack), enabling integration with modern web applications.


## Example usage in Javascript (web and Node.js)

### Client Side (web env)
#### Package installation
```shellscript
$ cd fhe-http/fhe_http_web
$ npm install
$ npm run tfhe-build // wasm pack tfhe-rs
$ npm run build
$ npm link

// change to web application dir
$ cd <web project path>
$ npm link fhe_http_web
```
#### Example of usage
```javascript
import init, { FheUint64, TfheConfigBuilder, TfheClientKey, TfheCompressedServerKey } from "fhe_http_web";
import { uint8ArrayToBase64, base64ToUint8Array, createFheHeader } from "fhe_http_web/extend";

async function IntegerEcryptionTest() {
    await init();

    let config = TfheConfigBuilder.default().build();
    let cks = TfheClientKey.generate(config);
    let sks = TfheCompressedServerKey.new(cks);
    let ct1 = FheUint64.encrypt_with_client_key(BigInt(3), cks);

    let ct2 = FheUint64.encrypt_with_client_key(BigInt(4), cks);
    let raw_ct1 = ct1.serialize();
    let raw_ct2 = ct2.serialize();

    let result = {
        clientKey: uint8ArrayToBase64(cks.serialize()),
        serverKey: uint8ArrayToBase64(sks.serialize()),
        ciphertext1: uint8ArrayToBase64(raw_ct1),
        ciphertext2: uint8ArrayToBase64(raw_ct2),
    };

    return result;
}
async function IntegerDecryptionTest(result: string, cks: string) {
    await init();
    let decoded_cks = base64ToUint8Array(cks);
    let decoded_result = base64ToUint8Array(result);
    let deserialized_cks = TfheClientKey.deserialize(decoded_cks);
    let deserialized_result = FheUint64.deserialize(decoded_result);
    let decrypted_result = deserialized_result.decrypt(deserialized_cks);
    console.log("Decrypted result: ", decrypted_result);
    return decrypted_result;
}

async function execute () {
    const encryption_data = await IntegerEcryptionTest();
    const header = createFheHeader("self-defined", "tfhe:0.7.2");
    let response = await fetch("http://localhost:3001", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            ...header,
        },
        body: JSON.stringify(encryption_data),
    });

    let computed_data = await response.json();
    let decrypted_result = await IntegerDecryptionTest(
        computed_data["result"],
        encryption_data.clientKey
    );
}
```
### Server Side (Node.js env)
#### Package installation
```shellscript
$ cd fhe-http/fhe_http_js
$ npm install
$ npm run build
$ npm link

// change to Node.js application dir
$ cd <nodejs project path>
$ npm link fhe_http_js

// do something here ... 
```
#### Example of usage
```javascript
const fheHttp = require("fhe_http_js");
const express = require("express");
const bodyParser = require("body-parser");
const cors = require("cors");
const app = express();

const corsOptions = {
    origin: "*", //(https://your-client-app.com)
    optionsSuccessStatus: 200,
};

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
    let encrypted_a = fheHttp.base64ToUint8Array(data["ciphertext1"]);
    let encrypted_b = fheHttp.base64ToUint8Array(data["ciphertext2"]);
    let server_key = fheHttp.base64ToUint8Array(data["serverKey"]);
    fheHttp.setServerKey(server_key);
    let encrypted_c = fheOps.add(encrypted_a, encrypted_b, "Uint64");
    let encoded_c = fheHttp.uint8ArrayToBase64(encrypted_c);
    let result = { result: encoded_c, status: "success" };
    res.json(result);
});

app.listen(3001, () => {
    console.log("Server is running on port 3001");
});
```
