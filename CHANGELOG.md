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

## 0.1.2

- added docker subcommand, usage:
```bash
> pyscan docker -n my-docker-image -p /path/inside/container/to/source
```

by <i>"source"</i> I mean `requirements.txt`, `pyproject.toml` or your python files.

- pyscan will not be using [deps.dev](https://deps.dev) API anymore to retrive latest stable versions. Will be using `pip` instead to get the installed package version from the user. Should've thought of that sooner. [credits to @anotherbridge for #1]
  
-  better error messages, though panics are the main way of displaying them.
  
-  This release was pretty rushed to fix that issue and get the docker feature on. I will be taking my sweet time with the next release to get:
  
- - github actions integration
- - make it easier for other tools to interact with pyscan
- - code complexity analyzer (not doing a linter cuz any respectable python dev already has one)
- - finally get to do tests, and lots of more ideas in my head. Thanks for the awesome support so far!

## 0.1.3

- Fixed a grave error where docker command left remnants and did not perform a complete cleanup.
- This release was made right after the previous release to fix this feature, however, the release page will contain both this message and the previous one so no one will miss out on the new stuff.