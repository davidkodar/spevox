# Contributing

Spevox uses a lightweight release-branch workflow. Keep changes small,
reviewable, and based on the branch appropriate to their purpose.

## Branches

- `main` contains release-ready code. Release tags are created only from this
  branch.
- `develop` is the integration branch for the next version.
- `feature/<short-name>` branches start from `develop` and return to `develop`.
- `fix/<short-name>` branches start from `develop` for unreleased defects.
- `release/<version>` branches start from `develop` only when a stabilization
  period is useful. They receive release-only fixes, then merge into both
  `main` and `develop` before `v<version>` is tagged.
- `hotfix/<version>` branches start from `main` for urgent released defects and
  merge into both `main` and `develop`.

Delete short-lived branches after they are merged. Do not rewrite published
release tags or replace their assets silently.

## Change workflow

1. Update local `develop` and create a short-lived feature or fix branch.
2. Make focused commits and add regression coverage where practical.
3. Run formatting, tests, Clippy, and any checks relevant to the change.
4. Merge the reviewed branch into `develop` and test the integrated result.
5. Promote to `main` only after the complete release gate passes.

The local release gate is:

```bash
CARGO_HOME="$PWD/.cargo-home" ./packaging/release-check.sh
```

Hosted workflows are intentionally manual while CI capacity is limited. A
successful local gate is therefore required before promotion to `main`.

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
