# MLS debug cli

This cli decodes MLS messages and pretty prints them. It expects a base64 encoded message as input.  

It currently supports MLS draft-12 with Wire's customizations. Draft-16 is not ready yet.

## installation

```bash
cargo install --git https://github.com/beltram/mls.git
```

## usage

For example to decode this [msg.txt](data/msg.txt) file:

```bash
cat msg.txt | mls msg
```