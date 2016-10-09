# emergency compliment
> a tutorial for beginner web development in Rust

Hi! Do you want to learn Rust? Do you come from a web programming background?
Traditionally, Rust is considered a systems language, but you can do web development
in it, too. This tutorial will help teach you how to build a very simple web
application is Rust using the [pencil] library.

## Getting Started

To get your environment set up, we'll have you follow the instructions for
getting the pre-written application up and running. Then we'll write it from scratch,
together!

### Install Rust

The best way to install Rust is with [rustup](https://www.rustup.rs/). rustup is a Rust
version manager. To install it type:

```
curl https://sh.rustup.rs -sSf | sh
```

To keep your rust up-to-date with the latest stable version of rust,
type:

```
rustup update
```

To check which version of Rust you have type:

```
rustc -- version
```

### Run the Application

Now let's get the app up and running. To do so, follow these steps:

1. Fork and clone this repository
2. cd `rust-emergency-compliment`
3. `cargo run`
4. Visit [`http://localhost:7878`](http://localhost:7878)

You should see something that looks like this:

![preview](public/img/preview.png)

### Creating a New Rust Project

There are many ways to setup a project in Rust, but in this tutorial, I'd recommend you
follow these instructions:

1. Create a repository on Github
2. Clone the repository
3. `cd` into the repository directory
4. Type `cargo init .`

This will create several files and folders for you automatically:

- `Cargo.toml`: metadata about your project and it's dependencies
- `.gitignore`: ignores compiled files built by Rust
- `src/lib.rs`: where your Rust code goes

We aren't writing a library, however. So to fix this, remove the `src/lib.rs` file
and create a new file called `src/main.rs`. This is where we'll write our app.

In order for this project to build, we need to put a `main` function in our `src/main.rs`.
Open you `src/main.rs` and put this code in it:

```rust
fn main() {}
```

This is a function called `main`. It's what `cargo run` looks for when it is called.

To be sure we set our project up correctly, let's see if it builds and runs. Since we
haven't told it to do anything yet, it won't! But, we will know that we have set up the
project correctly as the compiler won't yell at us :)
