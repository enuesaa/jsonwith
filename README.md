# jsonwith
Toy JSON Parser & Formatter

## Install
```bash
cargo install --git https://github.com/enuesaa/jsonwith
```

## Commands
```console
$ jsonwith --help
JSON Parser

Usage: jsonwith <COMMAND>

Commands:
  format     format json
  json2yaml  convert json to yaml
  yaml2json  convert yaml to json [under development]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Usage
### format
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
To specify tab size, pass `--indent` flag.
```console
$ jsonwith format '{"a":"b"}' --indent 4
{
    "a": "b"
}
```

### json2yaml
```console
$ jsonwith json2yaml '{"a":"b"}'
a: b
```
Also, jsonwith reads stdin as json string.
```console
$ echo '{"a":"b"}' | jsonwith json2yaml
a: b
```
