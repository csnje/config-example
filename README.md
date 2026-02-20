# config-example

A **Rust** example that demonstrates configuration of an application from file
and environment variable overrides using well known crates.
The example is specifically for loading a YAML file but may readily be altered
for other file formats (e.g. JSON).

To show application help:
```sh
cargo run -r -- -h
```
To write a configuration file with the default filename:
```sh
cargo run -r -- --write-config
```
To read a configuration file with the specified filename:
```sh
cargo run -r -- --config config.yaml
```
To override a configuration file value:
```sh
APP__VAL1=21 cargo run -r
```