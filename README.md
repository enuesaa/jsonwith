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
```bash
jsonwith format '{"a":"b"}'
```
Also, jsonwith reads stdin as json string.
```bash
echo '{"a":"b"}' | jsonwith format
```

### Format JSON with indent size
```bash
jsonwith format '{"a":"b"}' --indent 2
```

### Convert JSON to YAML
```bash
jsonwith json2yaml '{"a":"b"}'
```
Also, jsonwith reads stdin as json string.
```bash
echo '{"a":"b"}' | jsonwith json2yaml
```

### Convert JSON to YAML with indent size
```bash
jsonwith json2yaml '{"a":"b"}' --indent 2
```
