# Making a blog web app with SvelteKit and axum

<img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1024px-Rust_programming_language_black_logo.svg.png" width="100px" title="Rustlang logo"> <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/9/9b/Svelte-kit-horizontal.svg/2560px-Svelte-kit-horizontal.svg.png"  height="100px" title="SvelteKit logo"> <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/5/5c/Amazon_Lambda_architecture_logo.svg/1200px-Amazon_Lambda_architecture_logo.svg.png" height="100px" title="AWS Lambda logo">

<hr>

This is my side project.

It consists of two parts:
A [rust](https://www.rust-lang.org/) web server application written with [axum](https://github.com/tokio-rs/axum) that will be deployed to [AWS Lambda](https://aws.amazon.com/lambda/),
and a web client written with [SvelteKit](https://kit.svelte.dev/) and [typescript](https://www.typescriptlang.org/). It uses [PostgreSQL](https://www.postgresql.org/) as a database.

## server

I chose to use [axum](https://github.com/tokio-rs/axum) as a web framework, and [sqlx](https://github.com/launchbadge/sqlx) as a database driver. The app will deployed to [AWS Lambda](https://aws.amazon.com/lambda/) and locally tested with [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda).

## web-client

I chose to use [bun](https://github.com/oven-sh/bun) as a typescript runtime.

## Develop locally

### server

```bash
cargo lambda watch -p 3000
```

and local [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda) server will be running.

### web-client

```bash
yarn
yarn dev
```

and local vite server will be running.
