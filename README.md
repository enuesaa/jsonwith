# jsonwith
JSON Parser. This is my hobby project.

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
### Format json
```bash
jsonwith format '{"a":"b"}'
```
Also, jsonwith reads stdin as json string.
```bash
echo '{"a":"b"}' | jsonwith format
```

### Convert json to yaml
```bash
jsonwith json2yaml '{"a":"b"}'
```
Also, jsonwith reads stdin as json string.
```bash
echo '{"a":"b"}' | jsonwith json2yaml
```
