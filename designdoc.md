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

## Development Plan
### ~ v0.1.0
add tests.

### v0.1.0
v0.1.0 will release when command output are almostly good.
- format
- json2yaml
