# Lydcuit 

A third-party, mobile-first [Discuit] client written in Rust with [Tauri] and [Leptos], mainly targeting every platform 
that Tauri supports, with the exception of iOS, maybe once third party app stores are a thing, though. 
Should also support the web, but idk cors and stuff might be a problem.

**NOTE:** I'm using a beta version of Tauri that is subject to change and lacks documentation. Is this a good idea? No. 
But I'm doing it anyway, because I'm a bit silly.

## Get Lydcuit

patience

## Development

### Getting Started

First, ensure you have Rust installed, if not, you can get it [here][get-rust].

```bash
git clone https://gitlab.com/lydia-st/lydcuit.git
cd lydcuit
cargo build
```

### Running

```bash
# On desktop
cargo tauri dev

# On Android
cargo tauri android init # Only the first time
cargo tauri android dev
```



[//]: # (Links)
[Discuit]: https://discuit.net/
[Tauri]: https://tauri.app/
[Leptos]: https://leptos.dev/
[Rust]: https://www.rust-lang.org/

[get-rust]: https://www.rust-lang.org/tools/install

