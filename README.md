**Work in progress...**

# jsonwith-formatter

## Installation
~~~sh
$ cargo install --git https://github.com/enuesaa/jsonwith-formatter
$ which jsonwith
~~~

## Usage
### format json (pretty)
~~~sh
$ jsonwith --input sample.json
{
  "a": "b"
}
~~~

### convert json to yaml
~~~sh
$ jsonwith --input sample.json --format yaml
a: b
~~~
