# FHE-HTTP-WASM

## Build package
```shellscript=
$ wasm-pack build --target web
```

## Example usage
```javascript=ƒ
<!DOCTYPE html>
<html lang="en-US">
    <head>
        <meta charset="utf-8" />
        <title>hello-wasm example</title>
    </head>
    <body>
        <script type="module">
            import init, { Fhe } from "../pkg/fhe_http_wasm.js";
            init().then(() => {
                console.log(Fhe)
            });
        </script>
    </body>
</html>

```