### Running and building

1. Install rust https://rustup.rs/
2. Install wasm-pack https://rustwasm.github.io/wasm-pack/installer/
3. Run `npm run build` to build the wasm module
4. Run `wasm-pack test` to run internal rust tests
5. Run `npm run build-test` to build and run JEST tests


### Publishing to NPM

1. Bump of version in `Cargo.toml` and `package.json`
2. Run `npm run build` to build the wasm module
3. Run `npm publish` to publish to NPM