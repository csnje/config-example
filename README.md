## About

Example implementation of using a formatted file for configuration in **Rust**.

This is for a YAML file, but can readily be altered for other formats (e.g. JSON).

Makes use of several common packages:
* [clap](https://crates.io/crates/clap) for command line parsing
* [serde](https://crates.io/crates/serde) for serialisation and deserialisation
* [serde_yaml](https://crates.io/crates/serde_yaml) for **serde** YAML support

## Usage

Command line help:
```bash
config-example -h
```

Generate a template configuration file and exit:
```bash
config-example -c config.yaml --genconfig
```

Use a configuration file:
```bash
config-example -c config.yaml
```