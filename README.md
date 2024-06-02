## Daisy-RSX

## To Create a Release

To create a new release, use the following command locally:

```sh
cargo release patch
```

This will:

- Bump the version number.
- Create a git tag.
- Push changes to the remote repository.
- Trigger the GitHub Actions workflow to publish the crate.