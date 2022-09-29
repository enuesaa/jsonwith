**DEVELOPING**

# rust-json2yaml-practice

## Installation
~~~sh
$ cargo install --git https://github.com/enuesaa/rust-json2yaml-practice
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
