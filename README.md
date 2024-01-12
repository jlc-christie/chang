# Chang
A TUI for debugging JWTs offline, inspired by [JWT.io](https://jwt.io).

## Usage
```bash
chang <jwt>
```
There are 3 switchable areas: Header, Claims & Signature. They can be switched to by pressing:
- Header: `ctrl` + `h`
- Claims: `ctrl` + `b`
- Signature: `ctrl` + `d`

Type or paste the decoding key into the signature box to validate the signature.

**Note**: Unlike JWT.io, chang doesn't currently support _encoding_ new JWTs by typing into the header 
or claims areas. Although the text will be modified in the TUI, it won't have change anything and 
the signature is still being validated against the original JWT.

## Install
```bash
cargo install --path .
```