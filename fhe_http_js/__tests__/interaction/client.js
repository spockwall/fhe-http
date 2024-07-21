const fheHttp = require("fhe_http_js");
const https = require("https");

// send http request to server

function generate_keys() {
    let keyGen = new fheHttp.KeyGen();
    let config = keyGen.createConfig();
    let clientKey = keyGen.generateClientKey(config);
    let serverKey = keyGen.generateServerKey(clientKey);
    return { clientKey, serverKey };
}

function decrypt(encryptedNum, clientKey, dataType = "Int64") {
    let serializer = new fheHttp.Serializer();
    let fhe = new fheHttp.Fhe();
    let res = fhe.decrypt(encryptedNum, clientKey, dataType);
    return serializer.toI64(res);
}

function send_post_request(url) {
    let header = fheHttp.createFheHeader("123");
    let { clientKey, serverKey } = generate_keys();
    let data = { a: 1223123, b: 123123 };
    let payload_str = fheHttp.encryptFheBody(["a", "b"], "Int64", data, clientKey);
    let payload = JSON.parse(payload_str);
    console.log(payload);
    //// send post request
    //let options = {
    //    hostname: "localhost",
    //    port: 3000,
    //    header: header,
    //};

    //let req = https.request(options, (res) => {
    //    console.log(`statusCode: ${res.statusCode}`);
    //    res.on("data", (d) => {
    //        console.log(d);
    //    });
    //});
}

send_post_request();