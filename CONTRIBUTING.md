# Contributing

Spevox uses a single-branch workflow. Keep changes small and reviewable.

## Branches

- `main` is the only long-lived branch. It must always be releasable, and
  release tags are created only from it.
- All work happens on short-lived branches (for example `fix/<short-name>` or
  `feature/<short-name>`) that start from `main` and come back through a pull
  request. Merged branches are deleted automatically.

Do not rewrite published release tags or replace their assets silently.

## Change workflow

1. Update local `main` and create a short-lived branch.
2. Make focused commits and add regression coverage where practical.
3. Run formatting, tests, and Clippy locally.
4. Open a pull request against `main`; CI must pass before merging.
5. Squash-merge once green.

Before tagging a release, run the full local gate on KDE Plasma Wayland:

```bash
CARGO_HOME="$PWD/.cargo-home" ./packaging/release-check.sh
```

## Releases

For each release, update the Cargo version, changelog, AppStream metadata, and
user-facing documentation together. Create an annotated `v<version>` tag from
`main`, then publish a GitHub prerelease containing the binary archive, source
archive, their SHA-256 files, and the generated Arch `PKGBUILD`.

Use semantic versions:

- patch releases for compatible fixes and dependency or packaging maintenance;
- minor releases for compatible features and meaningful behavior changes;
- major releases for incompatible changes after the project reaches stability.

Keep `Cargo.lock` committed. Review third-party notices and redistribution terms
when dependencies, models, icons, or other bundled assets change. Never commit
credentials, private configuration, retained recordings, or internal audit
working documents.
