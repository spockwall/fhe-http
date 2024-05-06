# FHE_HTTP

## About
This project is designed as an HTTP extension for an FHE feature or plan, and integrated with existing HTTP services and standards. It allows users to specify their preferred service feature or plan by appending a specific field to the HTTP header when making requests to web services. It will support the web devtools as a packages.

## Example Diagram
1. User sends an http requests with header specifying 2 columns:
    ```
    - FHE-Type: <library>:<version>
    - FHE-Method: <Defined by server>
    ```
    and send encrypted payload text to a server.
2. A server receives the http request, and parse the header. If the server recognizes the header, then the server will use the library for computation.
    ```
    - success: 
        status code 209 for FHE computation successed.
    - error: 
        status code 405 for FHE-Method not allowed.
        status code 406 for FHE-Type not acceptable.
        status code 422 for unprocessable content.
    - response: 
        sever's computation result

## Tentative supporting
- rust
- python binding
- rust -> wasm, javascript

## References
- [Rust compression crate benchmark](https://blog.logrocket.com/rust-compression-libraries/)
