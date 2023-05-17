# Changelog

## 0.1.1

- added package subcommand, here's a quick usage:

```bash
pyscan package -n jinja2 -v 2.4.1
```

- slight logic improvments
- notes for next release:
- - if it detects toml but it doesnt find the dependencies table it panics, no idea how to err handle that for now
- - I should probably start using the `anyhow`  crate.
- - `get_latest_package_version` should become its own function and be moved to `utils.rs` in the next version

That's all for this release!

