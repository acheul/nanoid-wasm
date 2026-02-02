# VERSIONs

### v0.1.0

- v.0.1.1.
  - Yanked v.0.1.0 with introducing macro in v.0.1.1;
- v.0.1.3
  - Fixed wrong readme in previous versions;

### v.0.2.0

- Fix `nanoid` macro bug by [Macs1324](https://github.com/acheul/nanoid-wasm/pull/1)
  - default use of `nanoid!()` must return 21-length nanoid now.
- **Removed all features**. Using `target_family` inside of the source code, instead of explicit features.
