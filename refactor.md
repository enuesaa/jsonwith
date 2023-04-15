# Refactor Plan
## Command Input
```bash
jsonwith format <json>
```
### From file
```bash
cat <filename> | jsonwith format
```

## Commands
### Format
```bash
jsonwith format --indent 2 --compact <json>
```

### Convert
```bash
jsonwith json2yaml --indent 2 <json>
```
```bash
jsonwith json2toml <json>
```
```bash
jsonwith yaml2json <json>
```
