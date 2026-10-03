# Third-Party Software Attribution & Open-Source Notices

> **Product:** AlphaForge  
> **Generation Date:** 2026-10-03  
> **Document:** `THIRD_PARTY_NOTICES.md`  
> **License Compliance:** All third-party software licenses are honored according to their terms.

---

## 1. Primary Upstream Components

### 1.1 Goose Agentic Runtime
- **Component:** Goose (aaif-goose)
- **Governing Body:** Agentic AI Foundation (Linux Foundation)
- **License:** Apache License 2.0
- **Upstream Repository:** `https://github.com/aaif-goose/goose`
- **Notice:**
```text
Copyright 2024-2026 Agentic AI Foundation (AAIF), a Linux Foundation project.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

### 1.2 Wealthfolio Architectural Reference
- **Project:** Wealthfolio (Local Reference)
- **License:** GNU Affero General Public License v3.0 (AGPL-3.0)
- **Notice:** AlphaForge references domain modeling patterns from Wealthfolio. All financial services, SQLx migrations, Tauri commands, and React interfaces in AlphaForge are clean-room, panic-free reimplementations maintaining safe typing and local desktop isolation.

---

## 2. Bundled Internal Plugins
The following internal capability modules are built-in and distributed under the AlphaForge project license (MIT):
- **company-comparison:** Comparative multi-company fundamental metrics analysis plugin.
- **financial-analysis:** Portfolio asset breakdown, returns, and scenario analysis plugin.

---

## 3. Rust Dependencies (624 crates)

| Crate | Version | Ecosystem | Common License |
|-------|---------|-----------|----------------|
| `adler2` | `2.0.1` | Cargo | MIT / Apache-2.0 |
| `adobe-cmap-parser` | `0.4.1` | Cargo | MIT / Apache-2.0 |
| `aes` | `0.8.4` | Cargo | MIT / Apache-2.0 |
| `agent-worker` | `0.1.0` | Cargo | MIT / Apache-2.0 |
| `ahash` | `0.7.8` | Cargo | MIT / Apache-2.0 |
| `aho-corasick` | `1.1.4` | Cargo | MIT / Apache-2.0 |
| `alloc-no-stdlib` | `2.0.4` | Cargo | MIT / Apache-2.0 |
| `alloc-stdlib` | `0.2.4` | Cargo | MIT / Apache-2.0 |
| `allocator-api2` | `0.2.21` | Cargo | MIT / Apache-2.0 |
| `android_system_properties` | `0.1.5` | Cargo | MIT / Apache-2.0 |
| `anes` | `0.1.6` | Cargo | MIT / Apache-2.0 |
| `anstream` | `1.0.0` | Cargo | MIT / Apache-2.0 |
| `anstyle` | `1.0.14` | Cargo | MIT / Apache-2.0 |
| `anstyle-parse` | `1.0.0` | Cargo | MIT / Apache-2.0 |
| `anstyle-query` | `1.1.5` | Cargo | MIT / Apache-2.0 |
| `anstyle-wincon` | `3.0.11` | Cargo | MIT / Apache-2.0 |
| `anyhow` | `1.0.104` | Cargo | MIT / Apache-2.0 |
| `approx` | `0.5.1` | Cargo | MIT / Apache-2.0 |
| `arrayvec` | `0.7.8` | Cargo | MIT / Apache-2.0 |
| `async-broadcast` | `0.7.2` | Cargo | MIT / Apache-2.0 |
| `async-channel` | `2.5.0` | Cargo | MIT / Apache-2.0 |
| `async-executor` | `1.14.0` | Cargo | MIT / Apache-2.0 |
| `async-io` | `2.6.0` | Cargo | MIT / Apache-2.0 |
| `async-lock` | `3.4.2` | Cargo | MIT / Apache-2.0 |
| `async-process` | `2.5.0` | Cargo | MIT / Apache-2.0 |
| `async-recursion` | `1.1.1` | Cargo | MIT / Apache-2.0 |
| `async-signal` | `0.2.14` | Cargo | MIT / Apache-2.0 |
| `async-task` | `4.7.1` | Cargo | MIT / Apache-2.0 |
| `async-trait` | `0.1.91` | Cargo | MIT / Apache-2.0 |
| `atk` | `0.18.2` | Cargo | MIT / Apache-2.0 |
| `atk-sys` | `0.18.2` | Cargo | MIT / Apache-2.0 |
| `atoi` | `2.0.0` | Cargo | MIT / Apache-2.0 |
| `atomic-waker` | `1.1.2` | Cargo | MIT / Apache-2.0 |
| `autocfg` | `1.5.1` | Cargo | MIT / Apache-2.0 |
| `base64` | `0.21.7` | Cargo | MIT / Apache-2.0 |
| `base64ct` | `1.8.3` | Cargo | MIT / Apache-2.0 |
| `bit-set` | `0.8.0` | Cargo | MIT / Apache-2.0 |
| `bit-vec` | `0.8.0` | Cargo | MIT / Apache-2.0 |
| `bitflags` | `1.3.2` | Cargo | MIT / Apache-2.0 |
| `bitvec` | `1.1.1` | Cargo | MIT / Apache-2.0 |
| `block-buffer` | `0.10.4` | Cargo | MIT / Apache-2.0 |
| `block-padding` | `0.3.3` | Cargo | MIT / Apache-2.0 |
| `block2` | `0.6.2` | Cargo | MIT / Apache-2.0 |
| `blocking` | `1.6.2` | Cargo | MIT / Apache-2.0 |
| `borsh` | `1.8.0` | Cargo | MIT / Apache-2.0 |
| `borsh-derive` | `1.8.0` | Cargo | MIT / Apache-2.0 |
| `brotli` | `8.0.4` | Cargo | MIT / Apache-2.0 |
| `brotli-decompressor` | `5.0.3` | Cargo | MIT / Apache-2.0 |
| `bs58` | `0.5.1` | Cargo | MIT / Apache-2.0 |
| `bumpalo` | `3.20.3` | Cargo | MIT / Apache-2.0 |
| `bytecheck` | `0.6.12` | Cargo | MIT / Apache-2.0 |
| `bytecheck_derive` | `0.6.12` | Cargo | MIT / Apache-2.0 |
| `bytemuck` | `1.25.2` | Cargo | MIT / Apache-2.0 |
| `byteorder` | `1.5.0` | Cargo | MIT / Apache-2.0 |
| `bytes` | `1.12.1` | Cargo | MIT / Apache-2.0 |
| `cairo-rs` | `0.18.5` | Cargo | MIT / Apache-2.0 |
| `cairo-sys-rs` | `0.18.2` | Cargo | MIT / Apache-2.0 |
| `camino` | `1.2.5` | Cargo | MIT / Apache-2.0 |
| `cargo_metadata` | `0.19.2` | Cargo | MIT / Apache-2.0 |
| `cargo_toml` | `0.22.3` | Cargo | MIT / Apache-2.0 |
| `cargo-platform` | `0.1.9` | Cargo | MIT / Apache-2.0 |
| `cast` | `0.3.0` | Cargo | MIT / Apache-2.0 |
| `cbc` | `0.1.2` | Cargo | MIT / Apache-2.0 |
| `cc` | `1.4.0` | Cargo | MIT / Apache-2.0 |
| `cesu8` | `1.1.0` | Cargo | MIT / Apache-2.0 |
| `cfb` | `0.7.3` | Cargo | MIT / Apache-2.0 |
| `cff-parser` | `0.2.0` | Cargo | MIT / Apache-2.0 |
| `cfg_aliases` | `0.2.2` | Cargo | MIT / Apache-2.0 |
| `cfg-expr` | `0.15.8` | Cargo | MIT / Apache-2.0 |
| `cfg-if` | `1.0.4` | Cargo | MIT / Apache-2.0 |
| `chacha20` | `0.10.1` | Cargo | MIT / Apache-2.0 |
| `chrono` | `0.4.45` | Cargo | MIT / Apache-2.0 |
| `ciborium` | `0.2.2` | Cargo | MIT / Apache-2.0 |
| `ciborium-io` | `0.2.2` | Cargo | MIT / Apache-2.0 |
| `ciborium-ll` | `0.2.2` | Cargo | MIT / Apache-2.0 |
| `cipher` | `0.4.4` | Cargo | MIT / Apache-2.0 |
| `clap` | `4.6.5` | Cargo | MIT / Apache-2.0 |
| `clap_builder` | `4.6.5` | Cargo | MIT / Apache-2.0 |
| `clap_derive` | `4.6.4` | Cargo | MIT / Apache-2.0 |
| `clap_lex` | `1.1.0` | Cargo | MIT / Apache-2.0 |
| `colorchoice` | `1.0.5` | Cargo | MIT / Apache-2.0 |
| `combine` | `4.6.7` | Cargo | MIT / Apache-2.0 |
| `concurrent-queue` | `2.5.0` | Cargo | MIT / Apache-2.0 |
| `const-oid` | `0.9.6` | Cargo | MIT / Apache-2.0 |
| `cookie` | `0.18.1` | Cargo | MIT / Apache-2.0 |
| `cookie_store` | `0.22.1` | Cargo | MIT / Apache-2.0 |
| `core-foundation` | `0.9.4` | Cargo | MIT / Apache-2.0 |
| `core-foundation-sys` | `0.8.7` | Cargo | MIT / Apache-2.0 |
| `core-graphics` | `0.25.0` | Cargo | MIT / Apache-2.0 |
| `core-graphics-types` | `0.2.0` | Cargo | MIT / Apache-2.0 |
| `cpufeatures` | `0.2.17` | Cargo | MIT / Apache-2.0 |
| `crc` | `3.4.0` | Cargo | MIT / Apache-2.0 |
| `crc-catalog` | `2.5.0` | Cargo | MIT / Apache-2.0 |
| `crc32fast` | `1.5.0` | Cargo | MIT / Apache-2.0 |
| `criterion` | `0.5.1` | Cargo | MIT / Apache-2.0 |
| `criterion-plot` | `0.5.0` | Cargo | MIT / Apache-2.0 |
| `crossbeam-channel` | `0.5.16` | Cargo | MIT / Apache-2.0 |
| `crossbeam-deque` | `0.8.7` | Cargo | MIT / Apache-2.0 |
| `crossbeam-epoch` | `0.9.20` | Cargo | MIT / Apache-2.0 |
| `crossbeam-queue` | `0.3.13` | Cargo | MIT / Apache-2.0 |
| `crossbeam-utils` | `0.8.22` | Cargo | MIT / Apache-2.0 |
| `crunchy` | `0.2.4` | Cargo | MIT / Apache-2.0 |
| `crypto-common` | `0.1.7` | Cargo | MIT / Apache-2.0 |
| `cssparser` | `0.36.0` | Cargo | MIT / Apache-2.0 |
| `cssparser-macros` | `0.6.1` | Cargo | MIT / Apache-2.0 |
| `csv` | `1.4.0` | Cargo | MIT / Apache-2.0 |
| `csv-core` | `0.1.13` | Cargo | MIT / Apache-2.0 |
| `ctor` | `0.8.0` | Cargo | MIT / Apache-2.0 |
| `ctor-proc-macro` | `0.0.7` | Cargo | MIT / Apache-2.0 |
| `darling` | `0.23.0` | Cargo | MIT / Apache-2.0 |
| `darling_core` | `0.23.0` | Cargo | MIT / Apache-2.0 |
| `darling_macro` | `0.23.0` | Cargo | MIT / Apache-2.0 |
| `dbus` | `0.9.12` | Cargo | MIT / Apache-2.0 |
| `der` | `0.7.10` | Cargo | MIT / Apache-2.0 |
| `deranged` | `0.5.8` | Cargo | MIT / Apache-2.0 |
| `derive_more` | `2.1.1` | Cargo | MIT / Apache-2.0 |
| `derive_more-impl` | `2.1.1` | Cargo | MIT / Apache-2.0 |
| `digest` | `0.10.7` | Cargo | MIT / Apache-2.0 |
| `dirs` | `6.0.0` | Cargo | MIT / Apache-2.0 |
| `dirs-sys` | `0.5.0` | Cargo | MIT / Apache-2.0 |
| `dispatch2` | `0.3.1` | Cargo | MIT / Apache-2.0 |
| `displaydoc` | `0.2.7` | Cargo | MIT / Apache-2.0 |
| `dlib` | `0.5.3` | Cargo | MIT / Apache-2.0 |
| `dlopen2` | `0.8.2` | Cargo | MIT / Apache-2.0 |
| `dlopen2_derive` | `0.4.3` | Cargo | MIT / Apache-2.0 |
| `document-features` | `0.2.12` | Cargo | MIT / Apache-2.0 |
| `dom_query` | `0.27.0` | Cargo | MIT / Apache-2.0 |
| `dotenvy` | `0.15.7` | Cargo | MIT / Apache-2.0 |
| `downcast-rs` | `1.2.1` | Cargo | MIT / Apache-2.0 |
| `dpi` | `0.1.2` | Cargo | MIT / Apache-2.0 |
| `dtoa` | `1.0.11` | Cargo | MIT / Apache-2.0 |
| `dtoa-short` | `0.3.5` | Cargo | MIT / Apache-2.0 |
| `dtor` | `0.3.0` | Cargo | MIT / Apache-2.0 |
| `dtor-proc-macro` | `0.0.6` | Cargo | MIT / Apache-2.0 |
| `dunce` | `1.0.5` | Cargo | MIT / Apache-2.0 |
| `dyn-clone` | `1.0.20` | Cargo | MIT / Apache-2.0 |
| `ecb` | `0.1.2` | Cargo | MIT / Apache-2.0 |
| `either` | `1.17.0` | Cargo | MIT / Apache-2.0 |
| `embed_plist` | `1.2.2` | Cargo | MIT / Apache-2.0 |
| `embed-resource` | `3.0.11` | Cargo | MIT / Apache-2.0 |
| `encoding_rs` | `0.8.35` | Cargo | MIT / Apache-2.0 |
| `endi` | `1.1.1` | Cargo | MIT / Apache-2.0 |
| `enumflags2` | `0.7.12` | Cargo | MIT / Apache-2.0 |
| `enumflags2_derive` | `0.7.12` | Cargo | MIT / Apache-2.0 |
| `equivalent` | `1.0.2` | Cargo | MIT / Apache-2.0 |
| `erased-serde` | `0.4.10` | Cargo | MIT / Apache-2.0 |
| `errno` | `0.3.14` | Cargo | MIT / Apache-2.0 |
| `etcetera` | `0.8.0` | Cargo | MIT / Apache-2.0 |
| `euclid` | `0.20.14` | Cargo | MIT / Apache-2.0 |
| `event-listener` | `5.4.2` | Cargo | MIT / Apache-2.0 |
| `event-listener-strategy` | `0.5.4` | Cargo | MIT / Apache-2.0 |
| `fastrand` | `2.5.0` | Cargo | MIT / Apache-2.0 |
| `fdeflate` | `0.3.7` | Cargo | MIT / Apache-2.0 |
| `field-offset` | `0.3.6` | Cargo | MIT / Apache-2.0 |
| `find-msvc-tools` | `0.1.9` | Cargo | MIT / Apache-2.0 |
| `flate2` | `1.1.9` | Cargo | MIT / Apache-2.0 |
| `flume` | `0.11.1` | Cargo | MIT / Apache-2.0 |
| `fnv` | `1.0.7` | Cargo | MIT / Apache-2.0 |
| `foldhash` | `0.1.5` | Cargo | MIT / Apache-2.0 |
| `foreign-types` | `0.3.2` | Cargo | MIT / Apache-2.0 |
| `foreign-types-macros` | `0.2.4` | Cargo | MIT / Apache-2.0 |
| `foreign-types-shared` | `0.1.1` | Cargo | MIT / Apache-2.0 |
| `form_urlencoded` | `1.2.2` | Cargo | MIT / Apache-2.0 |
| `funty` | `2.0.0` | Cargo | MIT / Apache-2.0 |
| `futures` | `0.3.33` | Cargo | MIT / Apache-2.0 |
| `futures-channel` | `0.3.33` | Cargo | MIT / Apache-2.0 |
| `futures-core` | `0.3.33` | Cargo | MIT / Apache-2.0 |
| `futures-executor` | `0.3.33` | Cargo | MIT / Apache-2.0 |
| `futures-intrusive` | `0.5.0` | Cargo | MIT / Apache-2.0 |
| `futures-io` | `0.3.33` | Cargo | MIT / Apache-2.0 |
| `futures-lite` | `2.6.1` | Cargo | MIT / Apache-2.0 |
| `futures-macro` | `0.3.33` | Cargo | MIT / Apache-2.0 |
| `futures-sink` | `0.3.33` | Cargo | MIT / Apache-2.0 |
| `futures-task` | `0.3.33` | Cargo | MIT / Apache-2.0 |
| `futures-util` | `0.3.33` | Cargo | MIT / Apache-2.0 |
| `gdk` | `0.18.2` | Cargo | MIT / Apache-2.0 |
| `gdk-pixbuf` | `0.18.5` | Cargo | MIT / Apache-2.0 |
| `gdk-pixbuf-sys` | `0.18.0` | Cargo | MIT / Apache-2.0 |
| `gdk-sys` | `0.18.2` | Cargo | MIT / Apache-2.0 |
| `gdkwayland-sys` | `0.18.2` | Cargo | MIT / Apache-2.0 |
| `gdkx11` | `0.18.2` | Cargo | MIT / Apache-2.0 |
| `gdkx11-sys` | `0.18.2` | Cargo | MIT / Apache-2.0 |
| `generic-array` | `0.14.7` | Cargo | MIT / Apache-2.0 |
| `getrandom` | `0.2.17` | Cargo | MIT / Apache-2.0 |
| `gio` | `0.18.4` | Cargo | MIT / Apache-2.0 |
| `gio-sys` | `0.18.1` | Cargo | MIT / Apache-2.0 |
| `glib` | `0.18.5` | Cargo | MIT / Apache-2.0 |
| `glib-macros` | `0.18.5` | Cargo | MIT / Apache-2.0 |
| `glib-sys` | `0.18.1` | Cargo | MIT / Apache-2.0 |
| `glob` | `0.3.4` | Cargo | MIT / Apache-2.0 |
| `gobject-sys` | `0.18.0` | Cargo | MIT / Apache-2.0 |
| `gtk` | `0.18.2` | Cargo | MIT / Apache-2.0 |
| `gtk-sys` | `0.18.2` | Cargo | MIT / Apache-2.0 |
| `gtk3-macros` | `0.18.2` | Cargo | MIT / Apache-2.0 |
| `h2` | `0.4.15` | Cargo | MIT / Apache-2.0 |
| `half` | `2.7.1` | Cargo | MIT / Apache-2.0 |
| `hashbrown` | `0.12.3` | Cargo | MIT / Apache-2.0 |
| `hashlink` | `0.10.0` | Cargo | MIT / Apache-2.0 |
| `heck` | `0.4.1` | Cargo | MIT / Apache-2.0 |
| `hermit-abi` | `0.5.2` | Cargo | MIT / Apache-2.0 |
| `hex` | `0.4.3` | Cargo | MIT / Apache-2.0 |
| `hkdf` | `0.12.4` | Cargo | MIT / Apache-2.0 |
| `hmac` | `0.12.1` | Cargo | MIT / Apache-2.0 |
| `home` | `0.5.12` | Cargo | MIT / Apache-2.0 |
| `html5ever` | `0.38.0` | Cargo | MIT / Apache-2.0 |
| `http` | `1.5.0` | Cargo | MIT / Apache-2.0 |
| `http-body` | `1.1.0` | Cargo | MIT / Apache-2.0 |
| `http-body-util` | `0.1.4` | Cargo | MIT / Apache-2.0 |
| `httparse` | `1.10.1` | Cargo | MIT / Apache-2.0 |
| `hyper` | `1.11.0` | Cargo | MIT / Apache-2.0 |
| `hyper-rustls` | `0.27.9` | Cargo | MIT / Apache-2.0 |
| `hyper-tls` | `0.6.0` | Cargo | MIT / Apache-2.0 |
| `hyper-util` | `0.1.20` | Cargo | MIT / Apache-2.0 |
| `iana-time-zone` | `0.1.65` | Cargo | MIT / Apache-2.0 |
| `iana-time-zone-haiku` | `0.1.2` | Cargo | MIT / Apache-2.0 |
| `ico` | `0.5.0` | Cargo | MIT / Apache-2.0 |
| `icu_collections` | `2.2.0` | Cargo | MIT / Apache-2.0 |
| `icu_locale_core` | `2.2.0` | Cargo | MIT / Apache-2.0 |
| `icu_normalizer` | `2.2.0` | Cargo | MIT / Apache-2.0 |
| `icu_normalizer_data` | `2.2.0` | Cargo | MIT / Apache-2.0 |
| `icu_properties` | `2.2.0` | Cargo | MIT / Apache-2.0 |
| `icu_properties_data` | `2.2.0` | Cargo | MIT / Apache-2.0 |
| `icu_provider` | `2.2.0` | Cargo | MIT / Apache-2.0 |
| `ident_case` | `1.0.1` | Cargo | MIT / Apache-2.0 |
| `idna` | `1.1.0` | Cargo | MIT / Apache-2.0 |
| `idna_adapter` | `1.2.2` | Cargo | MIT / Apache-2.0 |
| `indexmap` | `1.9.3` | Cargo | MIT / Apache-2.0 |
| `infer` | `0.19.0` | Cargo | MIT / Apache-2.0 |
| `inout` | `0.1.4` | Cargo | MIT / Apache-2.0 |
| `ipnet` | `2.12.0` | Cargo | MIT / Apache-2.0 |
| `is_terminal_polyfill` | `1.70.2` | Cargo | MIT / Apache-2.0 |
| `is-docker` | `0.2.0` | Cargo | MIT / Apache-2.0 |
| `is-terminal` | `0.4.17` | Cargo | MIT / Apache-2.0 |
| `is-wsl` | `0.4.0` | Cargo | MIT / Apache-2.0 |
| `itertools` | `0.10.5` | Cargo | MIT / Apache-2.0 |
| `itoa` | `1.0.18` | Cargo | MIT / Apache-2.0 |
| `javascriptcore-rs` | `1.1.2` | Cargo | MIT / Apache-2.0 |
| `javascriptcore-rs-sys` | `1.1.1` | Cargo | MIT / Apache-2.0 |
| `jni` | `0.21.1` | Cargo | MIT / Apache-2.0 |
| `jni-sys` | `0.3.1` | Cargo | MIT / Apache-2.0 |
| `jni-sys-macros` | `0.4.1` | Cargo | MIT / Apache-2.0 |
| `js-sys` | `0.3.103` | Cargo | MIT / Apache-2.0 |
| `json-patch` | `3.0.1` | Cargo | MIT / Apache-2.0 |
| `jsonptr` | `0.6.3` | Cargo | MIT / Apache-2.0 |
| `keyboard-types` | `0.7.0` | Cargo | MIT / Apache-2.0 |
| `keyring` | `3.6.3` | Cargo | MIT / Apache-2.0 |
| `lazy_static` | `1.5.0` | Cargo | MIT / Apache-2.0 |
| `libappindicator` | `0.9.0` | Cargo | MIT / Apache-2.0 |
| `libappindicator-sys` | `0.9.0` | Cargo | MIT / Apache-2.0 |
| `libc` | `0.2.189` | Cargo | MIT / Apache-2.0 |
| `libdbus-sys` | `0.2.7` | Cargo | MIT / Apache-2.0 |
| `libloading` | `0.7.4` | Cargo | MIT / Apache-2.0 |
| `libm` | `0.2.16` | Cargo | MIT / Apache-2.0 |
| `libredox` | `0.1.18` | Cargo | MIT / Apache-2.0 |
| `libsqlite3-sys` | `0.30.1` | Cargo | MIT / Apache-2.0 |
| `linux-raw-sys` | `0.12.1` | Cargo | MIT / Apache-2.0 |
| `litemap` | `0.8.2` | Cargo | MIT / Apache-2.0 |
| `litrs` | `1.0.0` | Cargo | MIT / Apache-2.0 |
| `lock_api` | `0.4.14` | Cargo | MIT / Apache-2.0 |
| `log` | `0.4.33` | Cargo | MIT / Apache-2.0 |
| `lopdf` | `0.42.0` | Cargo | MIT / Apache-2.0 |
| `lru-slab` | `0.1.2` | Cargo | MIT / Apache-2.0 |
| `markup5ever` | `0.38.0` | Cargo | MIT / Apache-2.0 |
| `matchers` | `0.2.0` | Cargo | MIT / Apache-2.0 |
| `matrixmultiply` | `0.3.11` | Cargo | MIT / Apache-2.0 |
| `md-5` | `0.10.6` | Cargo | MIT / Apache-2.0 |
| `memchr` | `2.8.3` | Cargo | MIT / Apache-2.0 |
| `memoffset` | `0.9.1` | Cargo | MIT / Apache-2.0 |
| `mime` | `0.3.17` | Cargo | MIT / Apache-2.0 |
| `miniz_oxide` | `0.8.9` | Cargo | MIT / Apache-2.0 |
| `mio` | `1.2.2` | Cargo | MIT / Apache-2.0 |
| `muda` | `0.19.3` | Cargo | MIT / Apache-2.0 |
| `nalgebra` | `0.32.6` | Cargo | MIT / Apache-2.0 |
| `nalgebra-macros` | `0.2.2` | Cargo | MIT / Apache-2.0 |
| `native-tls` | `0.2.18` | Cargo | MIT / Apache-2.0 |
| `ndk` | `0.9.0` | Cargo | MIT / Apache-2.0 |
| `ndk-sys` | `0.6.0+11769913` | Cargo | MIT / Apache-2.0 |
| `new_debug_unreachable` | `1.0.6` | Cargo | MIT / Apache-2.0 |
| `nom` | `8.0.0` | Cargo | MIT / Apache-2.0 |
| `nu-ansi-term` | `0.50.3` | Cargo | MIT / Apache-2.0 |
| `num_enum` | `0.7.6` | Cargo | MIT / Apache-2.0 |
| `num_enum_derive` | `0.7.6` | Cargo | MIT / Apache-2.0 |
| `num-bigint-dig` | `0.8.6` | Cargo | MIT / Apache-2.0 |
| `num-complex` | `0.4.6` | Cargo | MIT / Apache-2.0 |
| `num-conv` | `0.2.2` | Cargo | MIT / Apache-2.0 |
| `num-integer` | `0.1.46` | Cargo | MIT / Apache-2.0 |
| `num-iter` | `0.1.46` | Cargo | MIT / Apache-2.0 |
| `num-rational` | `0.4.2` | Cargo | MIT / Apache-2.0 |
| `num-traits` | `0.2.19` | Cargo | MIT / Apache-2.0 |
| `objc2` | `0.6.4` | Cargo | MIT / Apache-2.0 |
| `objc2-app-kit` | `0.3.2` | Cargo | MIT / Apache-2.0 |
| `objc2-cloud-kit` | `0.3.2` | Cargo | MIT / Apache-2.0 |
| `objc2-core-data` | `0.3.2` | Cargo | MIT / Apache-2.0 |
| `objc2-core-foundation` | `0.3.2` | Cargo | MIT / Apache-2.0 |
| `objc2-core-graphics` | `0.3.2` | Cargo | MIT / Apache-2.0 |
| `objc2-core-image` | `0.3.2` | Cargo | MIT / Apache-2.0 |
| `objc2-core-location` | `0.3.2` | Cargo | MIT / Apache-2.0 |
| `objc2-core-text` | `0.3.2` | Cargo | MIT / Apache-2.0 |
| `objc2-encode` | `4.1.0` | Cargo | MIT / Apache-2.0 |
| `objc2-exception-helper` | `0.1.1` | Cargo | MIT / Apache-2.0 |
| `objc2-foundation` | `0.3.2` | Cargo | MIT / Apache-2.0 |
| `objc2-io-surface` | `0.3.2` | Cargo | MIT / Apache-2.0 |
| `objc2-quartz-core` | `0.3.2` | Cargo | MIT / Apache-2.0 |
| `objc2-ui-kit` | `0.3.2` | Cargo | MIT / Apache-2.0 |
| `objc2-user-notifications` | `0.3.2` | Cargo | MIT / Apache-2.0 |
| `objc2-web-kit` | `0.3.2` | Cargo | MIT / Apache-2.0 |
| `once_cell` | `1.21.4` | Cargo | MIT / Apache-2.0 |
| `once_cell_polyfill` | `1.70.2` | Cargo | MIT / Apache-2.0 |
| `oorandom` | `11.1.5` | Cargo | MIT / Apache-2.0 |
| `open` | `5.4.0` | Cargo | MIT / Apache-2.0 |
| `openssl` | `0.10.81` | Cargo | MIT / Apache-2.0 |
| `openssl-macros` | `0.1.1` | Cargo | MIT / Apache-2.0 |
| `openssl-probe` | `0.2.1` | Cargo | MIT / Apache-2.0 |
| `openssl-sys` | `0.9.117` | Cargo | MIT / Apache-2.0 |
| `option-ext` | `0.2.0` | Cargo | MIT / Apache-2.0 |
| `ordered-stream` | `0.2.0` | Cargo | MIT / Apache-2.0 |
| `pango` | `0.18.3` | Cargo | MIT / Apache-2.0 |
| `pango-sys` | `0.18.0` | Cargo | MIT / Apache-2.0 |
| `parking` | `2.2.1` | Cargo | MIT / Apache-2.0 |
| `parking_lot` | `0.12.5` | Cargo | MIT / Apache-2.0 |
| `parking_lot_core` | `0.9.12` | Cargo | MIT / Apache-2.0 |
| `paste` | `1.0.15` | Cargo | MIT / Apache-2.0 |
| `pdf-extract` | `0.12.0` | Cargo | MIT / Apache-2.0 |
| `pem-rfc7468` | `0.7.0` | Cargo | MIT / Apache-2.0 |
| `percent-encoding` | `2.3.2` | Cargo | MIT / Apache-2.0 |
| `phf` | `0.13.1` | Cargo | MIT / Apache-2.0 |
| `phf_codegen` | `0.13.1` | Cargo | MIT / Apache-2.0 |
| `phf_generator` | `0.13.1` | Cargo | MIT / Apache-2.0 |
| `phf_macros` | `0.13.1` | Cargo | MIT / Apache-2.0 |
| `phf_shared` | `0.13.1` | Cargo | MIT / Apache-2.0 |
| `pin-project-lite` | `0.2.17` | Cargo | MIT / Apache-2.0 |
| `piper` | `0.2.5` | Cargo | MIT / Apache-2.0 |
| `pkcs1` | `0.7.5` | Cargo | MIT / Apache-2.0 |
| `pkcs8` | `0.10.2` | Cargo | MIT / Apache-2.0 |
| `pkg-config` | `0.3.33` | Cargo | MIT / Apache-2.0 |
| `plain` | `0.2.3` | Cargo | MIT / Apache-2.0 |
| `plist` | `1.10.0` | Cargo | MIT / Apache-2.0 |
| `plotters` | `0.3.7` | Cargo | MIT / Apache-2.0 |
| `plotters-backend` | `0.3.7` | Cargo | MIT / Apache-2.0 |
| `plotters-svg` | `0.3.7` | Cargo | MIT / Apache-2.0 |
| `png` | `0.17.16` | Cargo | MIT / Apache-2.0 |
| `polling` | `3.11.0` | Cargo | MIT / Apache-2.0 |
| `pollster` | `0.4.0` | Cargo | MIT / Apache-2.0 |
| `pom` | `1.1.0` | Cargo | MIT / Apache-2.0 |
| `postscript` | `0.14.1` | Cargo | MIT / Apache-2.0 |
| `potential_utf` | `0.1.5` | Cargo | MIT / Apache-2.0 |
| `powerfmt` | `0.2.0` | Cargo | MIT / Apache-2.0 |
| `ppv-lite86` | `0.2.21` | Cargo | MIT / Apache-2.0 |
| `precomputed-hash` | `0.1.1` | Cargo | MIT / Apache-2.0 |
| `proc-macro-crate` | `1.3.1` | Cargo | MIT / Apache-2.0 |
| `proc-macro-error` | `1.0.4` | Cargo | MIT / Apache-2.0 |
| `proc-macro-error-attr` | `1.0.4` | Cargo | MIT / Apache-2.0 |
| `proc-macro2` | `1.0.107` | Cargo | MIT / Apache-2.0 |
| `psl-types` | `2.0.11` | Cargo | MIT / Apache-2.0 |
| `ptr_meta` | `0.1.4` | Cargo | MIT / Apache-2.0 |
| `ptr_meta_derive` | `0.1.4` | Cargo | MIT / Apache-2.0 |
| `publicsuffix` | `2.3.0` | Cargo | MIT / Apache-2.0 |
| `quick-xml` | `0.41.0` | Cargo | MIT / Apache-2.0 |
| `quinn` | `0.11.11` | Cargo | MIT / Apache-2.0 |
| `quinn-proto` | `0.11.16` | Cargo | MIT / Apache-2.0 |
| `quinn-udp` | `0.5.15` | Cargo | MIT / Apache-2.0 |
| `quote` | `1.0.47` | Cargo | MIT / Apache-2.0 |
| `r-efi` | `5.3.0` | Cargo | MIT / Apache-2.0 |
| `radium` | `0.7.0` | Cargo | MIT / Apache-2.0 |
| `rand` | `0.8.7` | Cargo | MIT / Apache-2.0 |
| `rand_chacha` | `0.3.1` | Cargo | MIT / Apache-2.0 |
| `rand_core` | `0.6.4` | Cargo | MIT / Apache-2.0 |
| `rand_distr` | `0.4.3` | Cargo | MIT / Apache-2.0 |
| `rand_pcg` | `0.10.2` | Cargo | MIT / Apache-2.0 |
| `rangemap` | `1.7.1` | Cargo | MIT / Apache-2.0 |
| `raw-window-handle` | `0.6.2` | Cargo | MIT / Apache-2.0 |
| `rawpointer` | `0.2.1` | Cargo | MIT / Apache-2.0 |
| `rayon` | `1.12.0` | Cargo | MIT / Apache-2.0 |
| `rayon-core` | `1.13.0` | Cargo | MIT / Apache-2.0 |
| `redox_syscall` | `0.5.18` | Cargo | MIT / Apache-2.0 |
| `redox_users` | `0.5.2` | Cargo | MIT / Apache-2.0 |
| `ref-cast` | `1.0.26` | Cargo | MIT / Apache-2.0 |
| `ref-cast-impl` | `1.0.26` | Cargo | MIT / Apache-2.0 |
| `regex` | `1.13.1` | Cargo | MIT / Apache-2.0 |
| `regex-automata` | `0.4.16` | Cargo | MIT / Apache-2.0 |
| `regex-syntax` | `0.8.11` | Cargo | MIT / Apache-2.0 |
| `rend` | `0.4.2` | Cargo | MIT / Apache-2.0 |
| `reqwest` | `0.12.28` | Cargo | MIT / Apache-2.0 |
| `rfd` | `0.17.2` | Cargo | MIT / Apache-2.0 |
| `ring` | `0.17.14` | Cargo | MIT / Apache-2.0 |
| `rkyv` | `0.7.46` | Cargo | MIT / Apache-2.0 |
| `rkyv_derive` | `0.7.46` | Cargo | MIT / Apache-2.0 |
| `rsa` | `0.9.10` | Cargo | MIT / Apache-2.0 |
| `rust_decimal` | `1.42.1` | Cargo | MIT / Apache-2.0 |
| `rust_decimal_macros` | `1.40.0` | Cargo | MIT / Apache-2.0 |
| `rustc_version` | `0.4.1` | Cargo | MIT / Apache-2.0 |
| `rustc-hash` | `2.1.3` | Cargo | MIT / Apache-2.0 |
| `rustix` | `1.1.4` | Cargo | MIT / Apache-2.0 |
| `rustls` | `0.23.43` | Cargo | MIT / Apache-2.0 |
| `rustls-pki-types` | `1.15.1` | Cargo | MIT / Apache-2.0 |
| `rustls-webpki` | `0.103.13` | Cargo | MIT / Apache-2.0 |
| `rustversion` | `1.0.23` | Cargo | MIT / Apache-2.0 |
| `ryu` | `1.0.23` | Cargo | MIT / Apache-2.0 |
| `safe_arch` | `0.7.4` | Cargo | MIT / Apache-2.0 |
| `same-file` | `1.0.6` | Cargo | MIT / Apache-2.0 |
| `schannel` | `0.1.29` | Cargo | MIT / Apache-2.0 |
| `schemars` | `0.8.22` | Cargo | MIT / Apache-2.0 |
| `schemars_derive` | `0.8.22` | Cargo | MIT / Apache-2.0 |
| `scoped-tls` | `1.0.1` | Cargo | MIT / Apache-2.0 |
| `scopeguard` | `1.2.0` | Cargo | MIT / Apache-2.0 |
| `seahash` | `4.1.0` | Cargo | MIT / Apache-2.0 |
| `security-framework` | `3.7.0` | Cargo | MIT / Apache-2.0 |
| `security-framework-sys` | `2.17.0` | Cargo | MIT / Apache-2.0 |
| `selectors` | `0.36.1` | Cargo | MIT / Apache-2.0 |
| `semver` | `1.0.28` | Cargo | MIT / Apache-2.0 |
| `serde` | `1.0.229` | Cargo | MIT / Apache-2.0 |
| `serde_core` | `1.0.229` | Cargo | MIT / Apache-2.0 |
| `serde_derive` | `1.0.229` | Cargo | MIT / Apache-2.0 |
| `serde_derive_internals` | `0.29.1` | Cargo | MIT / Apache-2.0 |
| `serde_json` | `1.0.151` | Cargo | MIT / Apache-2.0 |
| `serde_repr` | `0.1.21` | Cargo | MIT / Apache-2.0 |
| `serde_spanned` | `0.6.9` | Cargo | MIT / Apache-2.0 |
| `serde_urlencoded` | `0.7.1` | Cargo | MIT / Apache-2.0 |
| `serde_with` | `3.21.0` | Cargo | MIT / Apache-2.0 |
| `serde_with_macros` | `3.21.0` | Cargo | MIT / Apache-2.0 |
| `serde_yaml` | `0.9.34+deprecated` | Cargo | MIT / Apache-2.0 |
| `serde-untagged` | `0.1.9` | Cargo | MIT / Apache-2.0 |
| `serialize-to-javascript` | `0.1.2` | Cargo | MIT / Apache-2.0 |
| `serialize-to-javascript-impl` | `0.1.2` | Cargo | MIT / Apache-2.0 |
| `servo_arc` | `0.4.3` | Cargo | MIT / Apache-2.0 |
| `sha1` | `0.10.7` | Cargo | MIT / Apache-2.0 |
| `sha2` | `0.10.9` | Cargo | MIT / Apache-2.0 |
| `sharded-slab` | `0.1.7` | Cargo | MIT / Apache-2.0 |
| `shlex` | `2.0.1` | Cargo | MIT / Apache-2.0 |
| `signal-hook-registry` | `1.4.8` | Cargo | MIT / Apache-2.0 |
| `signature` | `2.2.0` | Cargo | MIT / Apache-2.0 |
| `simba` | `0.8.1` | Cargo | MIT / Apache-2.0 |
| `simd-adler32` | `0.3.10` | Cargo | MIT / Apache-2.0 |
| `simdutf8` | `0.1.5` | Cargo | MIT / Apache-2.0 |
| `siphasher` | `1.0.3` | Cargo | MIT / Apache-2.0 |
| `slab` | `0.4.12` | Cargo | MIT / Apache-2.0 |
| `smallvec` | `1.15.2` | Cargo | MIT / Apache-2.0 |
| `socket2` | `0.6.5` | Cargo | MIT / Apache-2.0 |
| `softbuffer` | `0.4.8` | Cargo | MIT / Apache-2.0 |
| `soup3` | `0.5.0` | Cargo | MIT / Apache-2.0 |
| `soup3-sys` | `0.5.0` | Cargo | MIT / Apache-2.0 |
| `spin` | `0.9.9` | Cargo | MIT / Apache-2.0 |
| `spki` | `0.7.3` | Cargo | MIT / Apache-2.0 |
| `sqlx` | `0.8.6` | Cargo | MIT / Apache-2.0 |
| `sqlx-core` | `0.8.6` | Cargo | MIT / Apache-2.0 |
| `sqlx-macros` | `0.8.6` | Cargo | MIT / Apache-2.0 |
| `sqlx-macros-core` | `0.8.6` | Cargo | MIT / Apache-2.0 |
| `sqlx-mysql` | `0.8.6` | Cargo | MIT / Apache-2.0 |
| `sqlx-postgres` | `0.8.6` | Cargo | MIT / Apache-2.0 |
| `sqlx-sqlite` | `0.8.6` | Cargo | MIT / Apache-2.0 |
| `stable_deref_trait` | `1.2.1` | Cargo | MIT / Apache-2.0 |
| `statrs` | `0.17.1` | Cargo | MIT / Apache-2.0 |
| `string_cache` | `0.9.0` | Cargo | MIT / Apache-2.0 |
| `string_cache_codegen` | `0.6.1` | Cargo | MIT / Apache-2.0 |
| `stringprep` | `0.1.5` | Cargo | MIT / Apache-2.0 |
| `strsim` | `0.11.1` | Cargo | MIT / Apache-2.0 |
| `subtle` | `2.6.1` | Cargo | MIT / Apache-2.0 |
| `swift-rs` | `1.0.7` | Cargo | MIT / Apache-2.0 |
| `syn` | `1.0.109` | Cargo | MIT / Apache-2.0 |
| `sync_wrapper` | `1.0.2` | Cargo | MIT / Apache-2.0 |
| `synstructure` | `0.13.2` | Cargo | MIT / Apache-2.0 |
| `system-configuration` | `0.7.0` | Cargo | MIT / Apache-2.0 |
| `system-configuration-sys` | `0.6.0` | Cargo | MIT / Apache-2.0 |
| `system-deps` | `6.2.2` | Cargo | MIT / Apache-2.0 |
| `tao` | `0.35.3` | Cargo | MIT / Apache-2.0 |
| `tao-macros` | `0.1.4` | Cargo | MIT / Apache-2.0 |
| `tap` | `1.0.1` | Cargo | MIT / Apache-2.0 |
| `target-lexicon` | `0.12.16` | Cargo | MIT / Apache-2.0 |
| `tauri` | `2.11.5` | Cargo | MIT / Apache-2.0 |
| `tauri-build` | `2.6.3` | Cargo | MIT / Apache-2.0 |
| `tauri-codegen` | `2.6.3` | Cargo | MIT / Apache-2.0 |
| `tauri-macros` | `2.6.3` | Cargo | MIT / Apache-2.0 |
| `tauri-plugin` | `2.6.3` | Cargo | MIT / Apache-2.0 |
| `tauri-plugin-opener` | `2.5.4` | Cargo | MIT / Apache-2.0 |
| `tauri-plugin-store` | `2.4.4` | Cargo | MIT / Apache-2.0 |
| `tauri-runtime` | `2.11.3` | Cargo | MIT / Apache-2.0 |
| `tauri-runtime-wry` | `2.11.4` | Cargo | MIT / Apache-2.0 |
| `tauri-utils` | `2.9.3` | Cargo | MIT / Apache-2.0 |
| `tauri-winres` | `0.3.6` | Cargo | MIT / Apache-2.0 |
| `tempfile` | `3.27.0` | Cargo | MIT / Apache-2.0 |
| `tendril` | `0.5.1` | Cargo | MIT / Apache-2.0 |
| `thiserror` | `1.0.69` | Cargo | MIT / Apache-2.0 |
| `thiserror-impl` | `1.0.69` | Cargo | MIT / Apache-2.0 |
| `thread_local` | `1.1.10` | Cargo | MIT / Apache-2.0 |
| `time` | `0.3.54` | Cargo | MIT / Apache-2.0 |
| `time-core` | `0.1.9` | Cargo | MIT / Apache-2.0 |
| `time-macros` | `0.2.32` | Cargo | MIT / Apache-2.0 |
| `tinystr` | `0.8.3` | Cargo | MIT / Apache-2.0 |
| `tinytemplate` | `1.2.1` | Cargo | MIT / Apache-2.0 |
| `tinyvec` | `1.12.0` | Cargo | MIT / Apache-2.0 |
| `tinyvec_macros` | `0.1.1` | Cargo | MIT / Apache-2.0 |
| `tokio` | `1.53.1` | Cargo | MIT / Apache-2.0 |
| `tokio-macros` | `2.7.2` | Cargo | MIT / Apache-2.0 |
| `tokio-native-tls` | `0.3.1` | Cargo | MIT / Apache-2.0 |
| `tokio-rustls` | `0.26.4` | Cargo | MIT / Apache-2.0 |
| `tokio-stream` | `0.1.19` | Cargo | MIT / Apache-2.0 |
| `tokio-util` | `0.7.19` | Cargo | MIT / Apache-2.0 |
| `toml` | `0.8.2` | Cargo | MIT / Apache-2.0 |
| `toml_datetime` | `0.6.3` | Cargo | MIT / Apache-2.0 |
| `toml_edit` | `0.19.15` | Cargo | MIT / Apache-2.0 |
| `toml_parser` | `1.1.3+spec-1.1.0` | Cargo | MIT / Apache-2.0 |
| `toml_writer` | `1.1.2+spec-1.1.0` | Cargo | MIT / Apache-2.0 |
| `tower` | `0.5.3` | Cargo | MIT / Apache-2.0 |
| `tower-http` | `0.6.11` | Cargo | MIT / Apache-2.0 |
| `tower-layer` | `0.3.3` | Cargo | MIT / Apache-2.0 |
| `tower-service` | `0.3.3` | Cargo | MIT / Apache-2.0 |
| `tracing` | `0.1.44` | Cargo | MIT / Apache-2.0 |
| `tracing-attributes` | `0.1.31` | Cargo | MIT / Apache-2.0 |
| `tracing-core` | `0.1.36` | Cargo | MIT / Apache-2.0 |
| `tracing-log` | `0.2.0` | Cargo | MIT / Apache-2.0 |
| `tracing-subscriber` | `0.3.23` | Cargo | MIT / Apache-2.0 |
| `tray-icon` | `0.24.2` | Cargo | MIT / Apache-2.0 |
| `try-lock` | `0.2.5` | Cargo | MIT / Apache-2.0 |
| `ttf-parser` | `0.25.1` | Cargo | MIT / Apache-2.0 |
| `type1-encoding-parser` | `0.1.1` | Cargo | MIT / Apache-2.0 |
| `typeid` | `1.0.3` | Cargo | MIT / Apache-2.0 |
| `typenum` | `1.20.1` | Cargo | MIT / Apache-2.0 |
| `uds_windows` | `1.2.1` | Cargo | MIT / Apache-2.0 |
| `unic-char-property` | `0.9.0` | Cargo | MIT / Apache-2.0 |
| `unic-char-range` | `0.9.0` | Cargo | MIT / Apache-2.0 |
| `unic-common` | `0.9.0` | Cargo | MIT / Apache-2.0 |
| `unic-ucd-ident` | `0.9.0` | Cargo | MIT / Apache-2.0 |
| `unic-ucd-version` | `0.9.0` | Cargo | MIT / Apache-2.0 |
| `unicode-bidi` | `0.3.18` | Cargo | MIT / Apache-2.0 |
| `unicode-ident` | `1.0.24` | Cargo | MIT / Apache-2.0 |
| `unicode-normalization` | `0.1.25` | Cargo | MIT / Apache-2.0 |
| `unicode-properties` | `0.1.4` | Cargo | MIT / Apache-2.0 |
| `unicode-segmentation` | `1.13.3` | Cargo | MIT / Apache-2.0 |
| `unsafe-libyaml` | `0.2.11` | Cargo | MIT / Apache-2.0 |
| `untrusted` | `0.9.0` | Cargo | MIT / Apache-2.0 |
| `url` | `2.5.8` | Cargo | MIT / Apache-2.0 |
| `urlencoding` | `2.1.3` | Cargo | MIT / Apache-2.0 |
| `urlpattern` | `0.3.0` | Cargo | MIT / Apache-2.0 |
| `utf8_iter` | `1.0.4` | Cargo | MIT / Apache-2.0 |
| `utf8parse` | `0.2.2` | Cargo | MIT / Apache-2.0 |
| `uuid` | `1.24.0` | Cargo | MIT / Apache-2.0 |
| `valuable` | `0.1.1` | Cargo | MIT / Apache-2.0 |
| `vcpkg` | `0.2.15` | Cargo | MIT / Apache-2.0 |
| `version_check` | `0.9.5` | Cargo | MIT / Apache-2.0 |
| `version-compare` | `0.2.1` | Cargo | MIT / Apache-2.0 |
| `vswhom` | `0.1.0` | Cargo | MIT / Apache-2.0 |
| `vswhom-sys` | `0.1.3` | Cargo | MIT / Apache-2.0 |
| `walkdir` | `2.5.0` | Cargo | MIT / Apache-2.0 |
| `want` | `0.3.1` | Cargo | MIT / Apache-2.0 |
| `wasi` | `0.11.1+wasi-snapshot-preview1` | Cargo | MIT / Apache-2.0 |
| `wasip2` | `1.0.4+wasi-0.2.12` | Cargo | MIT / Apache-2.0 |
| `wasite` | `0.1.0` | Cargo | MIT / Apache-2.0 |
| `wasm-bindgen` | `0.2.126` | Cargo | MIT / Apache-2.0 |
| `wasm-bindgen-futures` | `0.4.76` | Cargo | MIT / Apache-2.0 |
| `wasm-bindgen-macro` | `0.2.126` | Cargo | MIT / Apache-2.0 |
| `wasm-bindgen-macro-support` | `0.2.126` | Cargo | MIT / Apache-2.0 |
| `wasm-bindgen-shared` | `0.2.126` | Cargo | MIT / Apache-2.0 |
| `wasm-streams` | `0.4.2` | Cargo | MIT / Apache-2.0 |
| `wayland-backend` | `0.3.16` | Cargo | MIT / Apache-2.0 |
| `wayland-client` | `0.31.15` | Cargo | MIT / Apache-2.0 |
| `wayland-protocols` | `0.32.13` | Cargo | MIT / Apache-2.0 |
| `wayland-scanner` | `0.31.11` | Cargo | MIT / Apache-2.0 |
| `wayland-sys` | `0.31.11` | Cargo | MIT / Apache-2.0 |
| `web_atoms` | `0.2.5` | Cargo | MIT / Apache-2.0 |
| `web-sys` | `0.3.103` | Cargo | MIT / Apache-2.0 |
| `web-time` | `1.1.0` | Cargo | MIT / Apache-2.0 |
| `webkit2gtk` | `2.0.2` | Cargo | MIT / Apache-2.0 |
| `webkit2gtk-sys` | `2.0.2` | Cargo | MIT / Apache-2.0 |
| `webpki-roots` | `1.0.9` | Cargo | MIT / Apache-2.0 |
| `webview2-com` | `0.38.2` | Cargo | MIT / Apache-2.0 |
| `webview2-com-macros` | `0.8.1` | Cargo | MIT / Apache-2.0 |
| `webview2-com-sys` | `0.38.2` | Cargo | MIT / Apache-2.0 |
| `weezl` | `0.1.12` | Cargo | MIT / Apache-2.0 |
| `whoami` | `1.6.1` | Cargo | MIT / Apache-2.0 |
| `wide` | `0.7.33` | Cargo | MIT / Apache-2.0 |
| `winapi` | `0.3.9` | Cargo | MIT / Apache-2.0 |
| `winapi-i686-pc-windows-gnu` | `0.4.0` | Cargo | MIT / Apache-2.0 |
| `winapi-util` | `0.1.11` | Cargo | MIT / Apache-2.0 |
| `winapi-x86_64-pc-windows-gnu` | `0.4.0` | Cargo | MIT / Apache-2.0 |
| `window-vibrancy` | `0.6.0` | Cargo | MIT / Apache-2.0 |
| `windows` | `0.61.3` | Cargo | MIT / Apache-2.0 |
| `windows_aarch64_gnullvm` | `0.42.2` | Cargo | MIT / Apache-2.0 |
| `windows_aarch64_msvc` | `0.42.2` | Cargo | MIT / Apache-2.0 |
| `windows_i686_gnu` | `0.42.2` | Cargo | MIT / Apache-2.0 |
| `windows_i686_gnullvm` | `0.52.6` | Cargo | MIT / Apache-2.0 |
| `windows_i686_msvc` | `0.42.2` | Cargo | MIT / Apache-2.0 |
| `windows_x86_64_gnu` | `0.42.2` | Cargo | MIT / Apache-2.0 |
| `windows_x86_64_gnullvm` | `0.42.2` | Cargo | MIT / Apache-2.0 |
| `windows_x86_64_msvc` | `0.42.2` | Cargo | MIT / Apache-2.0 |
| `windows-collections` | `0.2.0` | Cargo | MIT / Apache-2.0 |
| `windows-core` | `0.61.2` | Cargo | MIT / Apache-2.0 |
| `windows-future` | `0.2.1` | Cargo | MIT / Apache-2.0 |
| `windows-implement` | `0.60.2` | Cargo | MIT / Apache-2.0 |
| `windows-interface` | `0.59.3` | Cargo | MIT / Apache-2.0 |
| `windows-link` | `0.1.3` | Cargo | MIT / Apache-2.0 |
| `windows-numerics` | `0.2.0` | Cargo | MIT / Apache-2.0 |
| `windows-registry` | `0.6.1` | Cargo | MIT / Apache-2.0 |
| `windows-result` | `0.3.4` | Cargo | MIT / Apache-2.0 |
| `windows-strings` | `0.4.2` | Cargo | MIT / Apache-2.0 |
| `windows-sys` | `0.45.0` | Cargo | MIT / Apache-2.0 |
| `windows-targets` | `0.42.2` | Cargo | MIT / Apache-2.0 |
| `windows-threading` | `0.1.0` | Cargo | MIT / Apache-2.0 |
| `windows-version` | `0.1.7` | Cargo | MIT / Apache-2.0 |
| `winnow` | `0.5.40` | Cargo | MIT / Apache-2.0 |
| `winreg` | `0.55.0` | Cargo | MIT / Apache-2.0 |
| `wit-bindgen` | `0.57.1` | Cargo | MIT / Apache-2.0 |
| `writeable` | `0.6.3` | Cargo | MIT / Apache-2.0 |
| `wry` | `0.55.1` | Cargo | MIT / Apache-2.0 |
| `wyz` | `0.5.1` | Cargo | MIT / Apache-2.0 |
| `x11` | `2.21.0` | Cargo | MIT / Apache-2.0 |
| `x11-dl` | `2.21.0` | Cargo | MIT / Apache-2.0 |
| `yahoo_finance_api` | `4.1.0` | Cargo | MIT / Apache-2.0 |
| `yoke` | `0.8.3` | Cargo | MIT / Apache-2.0 |
| `yoke-derive` | `0.8.2` | Cargo | MIT / Apache-2.0 |
| `zbus` | `5.18.0` | Cargo | MIT / Apache-2.0 |
| `zbus_macros` | `5.18.0` | Cargo | MIT / Apache-2.0 |
| `zbus_names` | `4.3.4` | Cargo | MIT / Apache-2.0 |
| `zerocopy` | `0.8.55` | Cargo | MIT / Apache-2.0 |
| `zerocopy-derive` | `0.8.55` | Cargo | MIT / Apache-2.0 |
| `zerofrom` | `0.1.8` | Cargo | MIT / Apache-2.0 |
| `zerofrom-derive` | `0.1.7` | Cargo | MIT / Apache-2.0 |
| `zeroize` | `1.9.0` | Cargo | MIT / Apache-2.0 |
| `zerotrie` | `0.2.4` | Cargo | MIT / Apache-2.0 |
| `zerovec` | `0.11.6` | Cargo | MIT / Apache-2.0 |
| `zerovec-derive` | `0.11.3` | Cargo | MIT / Apache-2.0 |
| `zmij` | `1.0.23` | Cargo | MIT / Apache-2.0 |
| `zvariant` | `5.13.1` | Cargo | MIT / Apache-2.0 |
| `zvariant_derive` | `5.13.1` | Cargo | MIT / Apache-2.0 |
| `zvariant_utils` | `3.5.0` | Cargo | MIT / Apache-2.0 |

---

## 4. Node.js & Web Dependencies (42 packages)

| Package | Version | Ecosystem | Common License |
|---------|---------|-----------|----------------|
| `@eslint/js` | `^9` | npm | MIT / Apache-2.0 / ISC |
| `@radix-ui/react-dialog` | `^1.1` | npm | MIT / Apache-2.0 / ISC |
| `@radix-ui/react-dropdown-menu` | `^2.1` | npm | MIT / Apache-2.0 / ISC |
| `@radix-ui/react-popover` | `^1.1` | npm | MIT / Apache-2.0 / ISC |
| `@radix-ui/react-select` | `^2.1` | npm | MIT / Apache-2.0 / ISC |
| `@radix-ui/react-slot` | `^1.1` | npm | MIT / Apache-2.0 / ISC |
| `@radix-ui/react-toast` | `^1.2` | npm | MIT / Apache-2.0 / ISC |
| `@radix-ui/react-toggle` | `^1.1` | npm | MIT / Apache-2.0 / ISC |
| `@radix-ui/react-tooltip` | `^1.1` | npm | MIT / Apache-2.0 / ISC |
| `@tailwindcss/vite` | `^4` | npm | MIT / Apache-2.0 / ISC |
| `@tanstack/react-query` | `^5` | npm | MIT / Apache-2.0 / ISC |
| `@tauri-apps/api` | `^2` | npm | MIT / Apache-2.0 / ISC |
| `@tauri-apps/cli` | `^2` | npm | MIT / Apache-2.0 / ISC |
| `@tauri-apps/plugin-opener` | `^2` | npm | MIT / Apache-2.0 / ISC |
| `@tauri-apps/plugin-store` | `^2` | npm | MIT / Apache-2.0 / ISC |
| `@testing-library/jest-dom` | `^6` | npm | MIT / Apache-2.0 / ISC |
| `@testing-library/react` | `^16` | npm | MIT / Apache-2.0 / ISC |
| `@types/react` | `^19` | npm | MIT / Apache-2.0 / ISC |
| `@types/react-dom` | `^19` | npm | MIT / Apache-2.0 / ISC |
| `@vitejs/plugin-react` | `^4` | npm | MIT / Apache-2.0 / ISC |
| `class-variance-authority` | `^0.7` | npm | MIT / Apache-2.0 / ISC |
| `clsx` | `^2` | npm | MIT / Apache-2.0 / ISC |
| `eslint` | `^9` | npm | MIT / Apache-2.0 / ISC |
| `eslint-plugin-react-hooks` | `^5` | npm | MIT / Apache-2.0 / ISC |
| `eslint-plugin-react-refresh` | `^0.4` | npm | MIT / Apache-2.0 / ISC |
| `jsdom` | `^25` | npm | MIT / Apache-2.0 / ISC |
| `lucide-react` | `^0.400` | npm | MIT / Apache-2.0 / ISC |
| `next-themes` | `^0.4.6` | npm | MIT / Apache-2.0 / ISC |
| `prettier` | `^3` | npm | MIT / Apache-2.0 / ISC |
| `react` | `^19` | npm | MIT / Apache-2.0 / ISC |
| `react-dom` | `^19` | npm | MIT / Apache-2.0 / ISC |
| `react-hook-form` | `^7` | npm | MIT / Apache-2.0 / ISC |
| `react-router-dom` | `^7` | npm | MIT / Apache-2.0 / ISC |
| `recharts` | `^3.10.1` | npm | MIT / Apache-2.0 / ISC |
| `tailwind-merge` | `^2` | npm | MIT / Apache-2.0 / ISC |
| `tailwindcss` | `^4` | npm | MIT / Apache-2.0 / ISC |
| `typescript` | `^5.7` | npm | MIT / Apache-2.0 / ISC |
| `typescript-eslint` | `^8` | npm | MIT / Apache-2.0 / ISC |
| `vite` | `^6` | npm | MIT / Apache-2.0 / ISC |
| `vitest` | `^3` | npm | MIT / Apache-2.0 / ISC |
| `zod` | `^3` | npm | MIT / Apache-2.0 / ISC |
| `zustand` | `^5` | npm | MIT / Apache-2.0 / ISC |

---

## 5. Standard Permissive License Text (MIT / Apache 2.0)

Most bundled third-party libraries are provided under the MIT or Apache 2.0 licenses. Full copies of upstream license declarations can be found in their respective source distributions.
