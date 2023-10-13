# jsonwith
JSON Parser

> **Note**
> `jsonwith` is toy app.

## Install
```bash
cargo install --git https://github.com/enuesaa/jsonwith
```

## Commands
```bash
jsonwith format [JSON]
jsonwith json2yaml [JSON]
```

## Usage
### Format JSON
```console
$ jsonwith format '{"a":"b"}'
{
  "a": "b"
}
```
Also, jsonwith reads stdin as json string.
```console
$ echo '{"a":"b"}' | jsonwith format
{
  "a": "b"
}
```

### Format JSON with indent size
```console
$ jsonwith format '{"a":"b"}' --indent 4
{
    "a": "b"
}
```

### Convert JSON to YAML
```console
$ jsonwith json2yaml '{"a":"b"}'
a: b
```
Also, jsonwith reads stdin as json string.
```console
$ echo '{"a":"b"}' | jsonwith json2yaml
a: b
```
