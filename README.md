# webrpc-gen Rust templates

This repo contains the templates used by the `webrpc-gen` cli to code-generate
webrpc Rust client code.

This generator, from a webrpc schema/design file will code-generate:

1. Client -- a Rust client to speak to a webrpc server. See examples.

2. Server -- a NodeJS Typescript server, to test the Rust client. See examples.

## Usage

```
webrpc-gen -schema=example.ridl -target=rust -client -out=./gen.rs
```

or

```
webrpc-gen -schema=example.ridl -target=github.com/arilotter/gen-rust@v0.8.0 -server -client -out=./gen.rs
```

or

```
webrpc-gen -schema=example.ridl -target=./local-templates-on-disk -server -client -out=./gen.rs
```

As you can see, the `-target` supports default `rust`, any git URI, or a local folder :)

### Set custom template variables

Change any of the following values by passing `-option="Value"` CLI flag to `webrpc-gen`.

| webrpc-gen -option | Description          | Default value   |
| ------------------ | -------------------- | --------------- |
| `-client`          | generate client code | unset (`false`) |

## LICENSE

[MIT LICENSE](./LICENSE)
