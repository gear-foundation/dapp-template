<p align="center">
  <a href="https://gitpod.io/#https://github.com/gear-foundation/dapps-app" target="_blank">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="Gitpod">
  </a>
</p>

# Gear Template App

[![Build][build_badge]][build_href]
[![License][lic_badge]][lic_href]
[![Docs][docs_badge]][docs_href]

[build_badge]: https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps-app/build.yml?label=Build
[build_href]: https://github.com/gear-foundation/dapps-app/actions/workflows/build.yml

[lic_badge]: https://img.shields.io/badge/License-MIT-success
[lic_href]: https://github.com/gear-foundation/dapps-app/blob/master/LICENSE

[docs_badge]: https://img.shields.io/badge/Docs-online-5023dd
[docs_href]: https://dapp.rs/dapps-app

<!-- Description starts here -->

Template application. Use this repository as a template when creating a new application repo.

> https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template

<!-- End of description -->

## Initial checklist after creating a new repo

- [ ] Change the app name in `Cargo.toml`
- [ ] Fix Gitpod/badge/releases links in `README.md` (replace `gear-foundation/dapps-app` with `gear-foundation/<my-new-app>`)
- [ ] Add a description in `README.md`
- [ ] Fix dates, links, and initial commit hash in `CHANGELOG.md`
- [ ] Remove this section

## Prebuilt Binaries

Raw, optimized, and meta WASM binaries can be found in the [Releases section](https://github.com/gear-foundation/dapps-app/releases).

## Building Locally

### ⚙️ Install Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 🏗️ Build

```shell
cargo build --release
```

... or ...

```shell
make build
```

### ✅ Run tests

```shell
cargo test --release
```

... or ...

```shell
make test
```

### 🚀 Run everything with one command

```shell
make all
```

... or just ...

```shell
make
```

## License

The source code is licensed under the [MIT license](LICENSE).
