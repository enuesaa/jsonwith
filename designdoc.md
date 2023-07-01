# designdoc
## Commands
### format
```bash
cat <filename> | jsonwith format
```

### format with indent
```bash
cat <filename> | jsonwith format --indent 2
```

### format with interactive prompt
```bash
$ jsonwith format
filename: ./aaa.json
```

### convert json to yaml
```bash
cat <filename> | jsonwith json2yaml
```

### convert yaml to json
```bash
cat <filename> | jsonwith yaml2json
```

### convert json to hcl
Planning implemention on version 2.
```bash
cat <filename> | jsonwith json2hcl
```
