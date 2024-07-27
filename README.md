# jsonwith
Toy JSON Parser & Formatter 🎨

[![ci](https://github.com/enuesaa/jsonwith/actions/workflows/ci.yml/badge.svg)](https://github.com/enuesaa/jsonwith/actions/workflows/ci.yml)

## Install
```bash
cargo install --git https://github.com/enuesaa/jsonwith
```

## Usage
```console
$ jsonwith --help
Toy JSON Parser & Formatter

Usage: jsonwith [COMMAND]

Commands:
  format     format json
  json2yaml  convert json to yaml
  yaml2json  convert yaml to json [under development]

Options:
  -v, --version  Print Version
  -h, --help     Print help
```

## SubCommands
### format
```console
$ jsonwith format '{"a":"b"}'
{
  "a": "b"
}
```
Also, jsonwith reads stdin as json string.
```console
$ cat aa.json | jsonwith format
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
$ cat aa.json | jsonwith json2yaml
a: b
```
