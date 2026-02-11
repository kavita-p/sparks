# Sparks!

A Discord bot for indie TRPGs.

(she/her)

Sparks uses [serenity.rs' poise framework](https://github.com/serenity-rs/poise) to talk to Discord, and currently supports Forged in the Dark, Sparked by Resistance, Powered by the Apocalypse, and Wild Words rolls. Thanks to [River Ray](https://riverray.itch.io) for the original TypeScript implementation of Wild Words, on which Sparks' current Rust code is based.

(A brief note: the official help text asks users to report issues on Sparks' itch.io page. You're welcome to file them here as well, if you have a Github account; itch is given as the first point of contact only because most of Sparks' users are probably not programmers themselves.)

## Code

The meat of Sparks' code can be found in `src/interpreters`, which is responsible for taking vectors of dice and generating results from them. It uses a struct called `Rolls` to store dice, which can be found in `src/lib.rs`. `src/commands` is responsible for handling command input and returning replies. The code for the `flip` command is in `src/commands/flip.rs`, while `src/commands/misc.rs` contains the code for `buzz`, `flicker`, and `help`.

## Deploy
I use an Alpine Linux container to build Sparks and deploy her on a VM running the same OS. Shout out to [this article](https://medium.com/@kasthor/cross-compiling-rust-from-macos-to-linux-using-podman-f654a49f2288) for teaching me how to do that.

Here's the short version:

0. [Install podman](https://podman.io/docs/installation)
1. Build the container

```shell
podman build -t rust-builder -f Containerfile
```

2. Place the `cargo-podman` script in your ~/.cargo/bin and make it executable:

```shell
mkdir -p ~/.cargo/bin # if it doesn't exist
mv cargo-podman ~/.cargo/bin/
chmod +x ~/.cargo/bin/cargo-podman
```

3. Use it like a cargo subcommand:

```shell
cargo podman build --release
```
The resulting binary, located at `target/release/sparks`, is supposedly ready to run on any Linux system, but I've only tested it on Alpine. Also presumably this works mostly the same on Docker.

## Contributions

I am open to these! If you have ideas for how to improve Sparks, please let me know.

## Privacy Policy

Sparks stores no data on its users.

## Terms of Service

You are free to use this bot in any way you wish. To use it on Discord, you will have to comply with Discord's own [Terms of Service](https://discord.com/terms) and [Community Guidelines](https://discord.com/guidelines).
