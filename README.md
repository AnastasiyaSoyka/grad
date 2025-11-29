# Introduction

Grad is a command-line interface (CLI) for creating, analyzing, and manipulating arbitrary data.

# Usage

Grad may be invoked on the command-line using a terminal emulator of the user's choice. A help message containing more detailed usage information may be obtained by invoking the `help` command, like so:

```bash
grad help
```

## Generating Random Data

Grad is currently capable of generating the following types of random data:

- Encoded & Unencoded Binary
- Integers
- Personal Identification Numbers
- Passwords
- Passphrases

### Binary

Bear in mind that the first numerical positional argument always refers to the number of bytes to generate, and *not* to the number of characters in an encoded string.

#### Unencoded

To generate two-hundred and fifty-six bytes and write them to standard output:

```bash
grad random bytes 256
```

Note that, on NT hosts, console applications may only write UTF-8 bytes to standard output.

#### Base16

To generate a random Base16 string containing sixteen bytes:

```bash
grad random hex 16
```

For a batch of ten Base16 strings containing sixteen bytes each:

```bash
grad random hex 16 10
```

#### Base64

To generate a random Base64 string containing twelve bytes:

```bash
grad random base64 12
```

For a batch of ten Base64 strings containing twelve bytes each:

```bash
grad random base64 12 10
```

### Integer

To generate a random integer in the interval of one and one thousand:

```bash
grad random integer 1..1000
```

For a batch of ten integers in the interval of one and one thousand:

```bash
grad random integer 1..1000 10
```

### PIN

To generate a PIN containing six digits:

```bash
grad random digits 6
```

For a batch of ten PINs containing six digits each:

```bash
grad random digits 6 10
```

### Password

To generate a single password containing sixteen alphanumeric characters:

```bash
grad random password 16
```

For a batch of ten passwords each containg sixteen alphanumeric characters:

```bash
grad random password 16 10
```

### Passphrase

To generate a single passphrase containing six words:

```bash
grad random passphrase 6
```

For a batch of ten passphrases containing six words each:

```bash
grad random passphrase 6 10
```

## Time & Date

Grad is currently capable of the following chronological functions:

- Timestamp

### Current Time

To print the current date and time with the system-local timezone:

```bash
grad create timestamp
```

# Docker

A Dockerfile is included in this repository which, when built, provides a copy of Grad that can be run in a containerized environment.

## Build

```bash
docker build -t grad:latest .
```

## Run

```bash
docker run --rm -it grad:latest --version
```

# Contributing

Contributions are welcome! Feel free to open an issue or pull request.

## Build

```bash
cargo build --release
```

## Test

```bash
cargo test --workspace
```

## VCS

- `dev` <br> All pull requests are merged into this branch. May be pushed.
- `main` <br> Default branch. May not be pushed or force pushed.
- `stable` <br> Points to the ref of the latest Git tag with a name matching the following regular expression: `/v?\d+\.\d+\.\d+/` <br> May not be pushed or force pushed.
