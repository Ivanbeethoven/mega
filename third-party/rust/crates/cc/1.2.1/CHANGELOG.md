# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.2.1](https://github.com/rust-lang/cc-rs/compare/cc-v1.2.0...cc-v1.2.1) - 2024-11-14

### Other

- When invoking `cl -?`, set stdin to null ([#1288](https://github.com/rust-lang/cc-rs/pull/1288))

## [1.2.0](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.37...cc-v1.2.0) - 2024-11-11

### Added

- add i686-pc-windows-gnullvm prefix detection ([#1283](https://github.com/rust-lang/cc-rs/pull/1283))

### Other

- Allow only specifying the architecture ([#1285](https://github.com/rust-lang/cc-rs/pull/1285))
- Fix WASM vs. WASI options ([#1284](https://github.com/rust-lang/cc-rs/pull/1284))

## [1.1.37](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.36...cc-v1.1.37) - 2024-11-08

### Other

- Use relative directory for obj files hash ([#1270](https://github.com/rust-lang/cc-rs/pull/1270))
- Regenerate target info ([#1280](https://github.com/rust-lang/cc-rs/pull/1280))

## [1.1.36](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.35...cc-v1.1.36) - 2024-11-05

### Other

- Fix CUDA build with clang++. ([#1273](https://github.com/rust-lang/cc-rs/pull/1273))

## [1.1.35](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.34...cc-v1.1.35) - 2024-11-04

### Other

- Remove support for FRC ([#1268](https://github.com/rust-lang/cc-rs/pull/1268))
- Do not add -fPIC by default on UEFI targets ([#1263](https://github.com/rust-lang/cc-rs/pull/1263))
- Use -windows-gnu for all UEFI targets ([#1264](https://github.com/rust-lang/cc-rs/pull/1264))

## [1.1.34](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.33...cc-v1.1.34) - 2024-11-02

### Other

- Remove redundant flags ([#1256](https://github.com/rust-lang/cc-rs/pull/1256))

## [1.1.33](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.32...cc-v1.1.33) - 2024-11-02

### Other

- Reduce size of `cc::Build`  and size of generated targets ([#1257](https://github.com/rust-lang/cc-rs/pull/1257))

## [1.1.32](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.31...cc-v1.1.32) - 2024-11-02

### Other

- Use `rustc`'s knowledge of LLVM/Clang target triples ([#1252](https://github.com/rust-lang/cc-rs/pull/1252))
- Use Cargo's target information when possible ([#1225](https://github.com/rust-lang/cc-rs/pull/1225))

## [1.1.31](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.30...cc-v1.1.31) - 2024-10-19

### Other

- Add comment explaining why cc does not rebuild on env PATH change ([#1247](https://github.com/rust-lang/cc-rs/pull/1247))

## [1.1.30](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.29...cc-v1.1.30) - 2024-10-11

### Other

- Don't pass -fPIC by default on wasm ([#1245](https://github.com/rust-lang/cc-rs/pull/1245))

## [1.1.29](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.28...cc-v1.1.29) - 2024-10-11

### Other

- Regenerate target info ([#1243](https://github.com/rust-lang/cc-rs/pull/1243))

## [1.1.28](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.27...cc-v1.1.28) - 2024-10-06

### Other

- Environment variables: For one accepting boolean, treat "0", "false" and empty env as false ([#1238](https://github.com/rust-lang/cc-rs/pull/1238))

## [1.1.27](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.26...cc-v1.1.27) - 2024-10-06

### Other

- Revert "Use debug version of MSVC runtime library on debug ([#1231](https://github.com/rust-lang/cc-rs/pull/1231))" ([#1237](https://github.com/rust-lang/cc-rs/pull/1237))
- Disable `CC_ENABLE_DEBUG_OUTPUT` if it is set to "0" ([#1234](https://github.com/rust-lang/cc-rs/pull/1234))

## [1.1.26](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.25...cc-v1.1.26) - 2024-10-06

### Other

- Use debug version of MSVC runtime library on debug ([#1231](https://github.com/rust-lang/cc-rs/pull/1231))

## [1.1.25](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.24...cc-v1.1.25) - 2024-10-05

### Other

- Remove incorrect "lib" prefixes in CXXSTDLIB doc comments ([#1228](https://github.com/rust-lang/cc-rs/pull/1228))

## [1.1.24](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.23...cc-v1.1.24) - 2024-10-01

### Other

- Fix wasm32-wasip1-threads:  shared-memory disallowed due to not compiled with 'atomics' or 'bulk-memory' features ([#1221](https://github.com/rust-lang/cc-rs/pull/1221))
- Reduce the need for the host target triple ([#1224](https://github.com/rust-lang/cc-rs/pull/1224))
- Add auto cancellation for CI jobs ([#1222](https://github.com/rust-lang/cc-rs/pull/1222))

## [1.1.23](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.22...cc-v1.1.23) - 2024-09-30

### Other

- Update doc for detecting changes/upgrades of compilers ([#1218](https://github.com/rust-lang/cc-rs/pull/1218))

## [1.1.22](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.21...cc-v1.1.22) - 2024-09-27

### Other

- Don't rerun if PATH changes ([#1215](https://github.com/rust-lang/cc-rs/pull/1215))

## [1.1.21](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.20...cc-v1.1.21) - 2024-09-18

### Other

- disable pic for targets that end in `-none` ([#1212](https://github.com/rust-lang/cc-rs/pull/1212))

## [1.1.20](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.19...cc-v1.1.20) - 2024-09-17

### Other

- Add buildcache as known Rust and C/C++ compiler wrapper ([#1209](https://github.com/rust-lang/cc-rs/pull/1209))

## [1.1.19](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.18...cc-v1.1.19) - 2024-09-15

### Other

- Add support arm64e-apple-darwin ([#1207](https://github.com/rust-lang/cc-rs/pull/1207))

## [1.1.18](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.17...cc-v1.1.18) - 2024-09-07

### Other
- Fixed unsoundness in `StderrForwarder::forward_available` ([#1203](https://github.com/rust-lang/cc-rs/pull/1203))

## [1.1.17](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.16...cc-v1.1.17) - 2024-09-06

### Fixed
- fix finding toolchains when invoked by msbuild ([#1201](https://github.com/rust-lang/cc-rs/pull/1201))

## [1.1.16](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.15...cc-v1.1.16) - 2024-09-04

### Other
- Treat VxWorks wr-cc as a Gnu compiler ([#1198](https://github.com/rust-lang/cc-rs/pull/1198))

## [1.1.15](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.14...cc-v1.1.15) - 2024-08-26

### Other
- Add -mfloat-abi=hard as a default argument when using any arm/thumb-none-eabihf target ([#1194](https://github.com/rust-lang/cc-rs/pull/1194))

## [1.1.14](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.13...cc-v1.1.14) - 2024-08-23

### Other
- allow finding tools from path if VisualStudioDir is set

## [1.1.13](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.12...cc-v1.1.13) - 2024-08-16

### Other
- Fix detect family: should detect emscripten as clang, closes [#1185](https://github.com/rust-lang/cc-rs/pull/1185) ([#1186](https://github.com/rust-lang/cc-rs/pull/1186))

## [1.1.12](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.11...cc-v1.1.12) - 2024-08-15

### Other
- improve docs ([#1183](https://github.com/rust-lang/cc-rs/pull/1183))

## [1.1.11](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.10...cc-v1.1.11) - 2024-08-14

### Other
- Add support for parsing shell encoded `*FLAGS` ([#1181](https://github.com/rust-lang/cc-rs/pull/1181))
- Replace vector of tuples with BTreeMap which already is sorted and free of duplicates ([#1177](https://github.com/rust-lang/cc-rs/pull/1177))

## [1.1.10](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.9...cc-v1.1.10) - 2024-08-11

### Other
- Remap Windows targets triples to their LLVM counterparts ([#1176](https://github.com/rust-lang/cc-rs/pull/1176))

## [1.1.9](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.8...cc-v1.1.9) - 2024-08-11

### Other
- Add custom CC wrapper to the wrapper whitelist ([#1175](https://github.com/rust-lang/cc-rs/pull/1175))

## [1.1.8](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.7...cc-v1.1.8) - 2024-08-06

### Other
- Fix broken link in docs.rs ([#1173](https://github.com/rust-lang/cc-rs/pull/1173))

## [1.1.7](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.6...cc-v1.1.7) - 2024-07-29

### Other
- add `.objects` ([#1166](https://github.com/rust-lang/cc-rs/pull/1166))

## [1.1.6](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.5...cc-v1.1.6) - 2024-07-19

### Other
- Clippy fixes ([#1163](https://github.com/rust-lang/cc-rs/pull/1163))

## [1.1.5](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.4...cc-v1.1.5) - 2024-07-15

### Other
- Fix cyclic compilation: Use vendored once_cell ([#1154](https://github.com/rust-lang/cc-rs/pull/1154))

## [1.1.4](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.3...cc-v1.1.4) - 2024-07-14

### Other
- Support compiling on wasm targets (Supersede [#1068](https://github.com/rust-lang/cc-rs/pull/1068)) ([#1160](https://github.com/rust-lang/cc-rs/pull/1160))

## [1.1.3](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.2...cc-v1.1.3) - 2024-07-14

### Other
- Reduce msrv to 1.63 ([#1158](https://github.com/rust-lang/cc-rs/pull/1158))
- Revert "Use raw-dylib for windows-sys ([#1137](https://github.com/rust-lang/cc-rs/pull/1137))" ([#1157](https://github.com/rust-lang/cc-rs/pull/1157))
- Fix typos ([#1152](https://github.com/rust-lang/cc-rs/pull/1152))
- Fix `doc_lazy_continuation` lints ([#1153](https://github.com/rust-lang/cc-rs/pull/1153))

## [1.1.2](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.1...cc-v1.1.2) - 2024-07-12

### Other
- Add empty `jobserver` feature. ([#1150](https://github.com/rust-lang/cc-rs/pull/1150))

## [1.1.1](https://github.com/rust-lang/cc-rs/compare/cc-v1.1.0...cc-v1.1.1) - 2024-07-12

### Other
- Fix is_flag_supported not respecting emit_rerun_if_env_changed ([#1147](https://github.com/rust-lang/cc-rs/pull/1147)) ([#1148](https://github.com/rust-lang/cc-rs/pull/1148))

## [1.1.0](https://github.com/rust-lang/cc-rs/compare/cc-v1.0.106...cc-v1.1.0) - 2024-07-08

### Added
- add cargo_output to eliminate last vestiges of stdout pollution ([#1141](https://github.com/rust-lang/cc-rs/pull/1141))

## [1.0.106](https://github.com/rust-lang/cc-rs/compare/cc-v1.0.105...cc-v1.0.106) - 2024-07-08

### Other
- Drop support for Visual Studio 12 (2013) ([#1046](https://github.com/rust-lang/cc-rs/pull/1046))
- Use raw-dylib for windows-sys ([#1137](https://github.com/rust-lang/cc-rs/pull/1137))
- Bump msrv to 1.67 ([#1143](https://github.com/rust-lang/cc-rs/pull/1143))
- Bump msrv to 1.65 ([#1140](https://github.com/rust-lang/cc-rs/pull/1140))
- Fix clippy warnings ([#1138](https://github.com/rust-lang/cc-rs/pull/1138))

## [1.0.105](https://github.com/rust-lang/cc-rs/compare/cc-v1.0.104...cc-v1.0.105) - 2024-07-07

### Other
- Regenerate windows sys bindings ([#1132](https://github.com/rust-lang/cc-rs/pull/1132))
- Fix generate-windows-sys-bindings ([#1133](https://github.com/rust-lang/cc-rs/pull/1133))
- Fix gen-windows-sys-binding ([#1130](https://github.com/rust-lang/cc-rs/pull/1130))
- Fix gen-windows-sys-binding ([#1127](https://github.com/rust-lang/cc-rs/pull/1127))
- Update windows-bindgen requirement from 0.57 to 0.58 ([#1123](https://github.com/rust-lang/cc-rs/pull/1123))

## [1.0.104](https://github.com/rust-lang/cc-rs/compare/cc-v1.0.103...cc-v1.0.104) - 2024-07-01

### Other
- Fixed link break about compile-time-requirements ([#1118](https://github.com/rust-lang/cc-rs/pull/1118))

## [1.0.103](https://github.com/rust-lang/cc-rs/compare/cc-v1.0.102...cc-v1.0.103) - 2024-06-30

### Other
- Fix compilation for wasm: env WASI_SYSROOT should be optional ([#1114](https://github.com/rust-lang/cc-rs/pull/1114))

## [1.0.102](https://github.com/rust-lang/cc-rs/compare/cc-v1.0.101...cc-v1.0.102) - 2024-06-29

### Other
- Fix invalid wasi targets compatibility ([#1105](https://github.com/rust-lang/cc-rs/pull/1105))
- Speedup regenerate-target-info and regenerate-windows-sys ([#1110](https://github.com/rust-lang/cc-rs/pull/1110))

## [1.0.101](https://github.com/rust-lang/cc-rs/compare/cc-v1.0.100...cc-v1.0.101) - 2024-06-25

### Other
- Use `Build::getenv` instead of `env::var*` in anywhere that makes sense ([#1103](https://github.com/rust-lang/cc-rs/pull/1103))

## [1.0.100](https://github.com/rust-lang/cc-rs/compare/cc-v1.0.99...cc-v1.0.100) - 2024-06-23

### Other
- Update publish.yml to use release-plz ([#1101](https://github.com/rust-lang/cc-rs/pull/1101))
- Accept `OsStr` instead of `str` for flags ([#1100](https://github.com/rust-lang/cc-rs/pull/1100))
- Use `dep:` syntax to avoid implicit features. ([#1099](https://github.com/rust-lang/cc-rs/pull/1099))
- Minor clippy fixes. ([#1098](https://github.com/rust-lang/cc-rs/pull/1098))
- Fix WASI compilation for C++ ([#1083](https://github.com/rust-lang/cc-rs/pull/1083))
- Regenerate windows sys bindings ([#1096](https://github.com/rust-lang/cc-rs/pull/1096))
- Rename regenerate-windows-sys to regenerate-windows-sys.yml ([#1095](https://github.com/rust-lang/cc-rs/pull/1095))
- Create regenerate-windows-sys.yml ([#1094](https://github.com/rust-lang/cc-rs/pull/1094))
- Update windows-bindgen requirement from 0.56 to 0.57 ([#1091](https://github.com/rust-lang/cc-rs/pull/1091))
- Eagerly close tempfile to fix [#1082](https://github.com/rust-lang/cc-rs/pull/1082) ([#1087](https://github.com/rust-lang/cc-rs/pull/1087))
- Output msvc.exe in the output directory ([#1090](https://github.com/rust-lang/cc-rs/pull/1090))
- Fix clippy warnings on Windows ([#1088](https://github.com/rust-lang/cc-rs/pull/1088))
- Don't try to free DLL on drop ([#1089](https://github.com/rust-lang/cc-rs/pull/1089))
- Fix panic safety issue in StderrForwarder ([#1079](https://github.com/rust-lang/cc-rs/pull/1079))
