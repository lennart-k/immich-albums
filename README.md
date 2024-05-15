# immich-albums

This is a tool to automatically create Immich albums from your folder structure
when using an external library for storage.
For each album immich-albums expects an `album.toml` file like

```toml
[metadata]
name = "My album"
description = "The album contains images"
```

This file is then also used to link your local album to the Immich album.

## Todo

- [ ] Packaging
- [ ] Maybe discovery without the `album.toml` file?
- [ ] Configuring the thumbnail
- [ ] Moving existing albums to the external library file structure
