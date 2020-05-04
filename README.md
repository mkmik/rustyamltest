# rust-vs-go yaml parsing demo

## Context

I'm learning some rust and I'm trying to figure out whether the rust std+community libraries fare against
Go std+community libraries on the basic task of parsing compliant YAML sources.

In particular I'd like to see how they compare with the case of a utf-8 encoded input file
that begins with a BOM sequence.

## Codebase

This repo contains two code bases, one written in Rust and one written in Go.
Both programs take a filename on the commandline and parse all the yaml documents contained in it.
They then re-serialize them back to stdout.

## Input

There are two files `input-ascii.yaml` and `input-bom.yaml`. Both contain two simple yaml documents:

```
---
- a
- b
- c
---
other: document
```

but the latter (`input-bom.yaml`) contains a Byte-Order-Marker at the beginning of the file, that clearly marks the file as UTF-8.

```
$ file input-*.yaml
input-ascii.yaml: ASCII text
input-bom.yaml:   UTF-8 Unicode (with BOM) text
```

According to the YAML-1.2 standard this character can be optionally present at the beginning of the file.
(moreover it must be tolerated also between documents, to allow of dumb concatenation of documents, but this simple test doesn't go so far as to excercise that aspect).

## Output

### Go

The popular Go library `gopkg.in/yaml.v3` has an API that works on io.Readers.
It parses both files correctly:

```
$ go run . -- ./input-ascii.yaml
- a
- b
- c
---
other: document
$ go run . -- ./input-bom.yaml
- a
- b
- c
---
other: document
```


A minor detail also: it's trivial to reuse the same serializer when emitting YAML so the `---` document separator
is only emitted when needed.

### Rust

Not knowing any better, I used the `yaml-rust` library. As far as I can tell it parses and emits in-memory strings,
so the library itself appears to be oblivious to eventual encodings of such a string into files.

Unfortunately, as mentioned in the previous section, the YAML-1.2 standard does define which string encodings offer valid YAML "wire encodings".

I tried to use the dumbest method to load a file into a string I could find (`fs::read_to_string(filename)`)
just to see how it works:

```
$ target/debug/yamldemo ./input-ascii.yaml
---
- a
- b
- c
---
other: document
$ target/debug/yamldemo ./input-bom.yaml
read ./input-bom.yaml into string; ascii: false
---
--- - a - b - c
---
other: document
```

I added a debug dump to see what was `--- - a - b - c`:

```
String("\u{feff}--- - a - b - c")
```

BTW, `\ufeff` is the codepoint for the zero width non-break space (aka the BOM).

## Conclusion

The rust `fs::read_to_string` function preserves the BOM when reading UTF-8 (no surprises here).

The yaml-rust library chokes when parsing a document that starts with the BOM, incorrectly treating
the rest of a the line as a string literal.
