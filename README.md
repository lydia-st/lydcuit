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

### Building

To actually build binaries, you can use the following commands:

> **ℹ️ Note**\
> I'm fairly certain that you can only build for the platform you're on, eg. you can only build a `.deb`, `.rpm` or `.appimage` 
> on Linux, and a `.exe` on Windows. This is except for Android (and maybe iOS? but probably only if on a Mac).
> I'll test this at some point, maybe. [Relevant documentation](https://tauri.app/v1/guides/building/cross-platform)

```bash
# For desktop
cargo tauri build

# For Android
cargo tauri android build # i think
```

## Contributing

Any contributions are welcome. 

> **⚠️ Warning**\
> On GitHub? Please head over to the [GitLab] repository to contribute! This is just a mirror.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


# 

[//]: # (Links)
[Discuit]: https://discuit.net/
[Tauri]: https://tauri.app/
[Leptos]: https://leptos.dev/
[Rust]: https://www.rust-lang.org/
[GitLab]: https://gitlab.com/lydia-st/lydcuit

[get-rust]: https://www.rust-lang.org/tools/install

