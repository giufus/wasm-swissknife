## KLICAM v2 
Rust, Wasm, Vite, Svelte..just an experiment. Updated version of klicam, using a more modern toolkit.

![sshot](./sshot.png)

### Resources
- https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm
- https://www.youtube.com/watch?v=8zDYoprO358&ab_channel=FriendlyTL

### Setup
- install node.js v20
- install rust
- install wasm-pack with cargo
- build wasm
```
wasm-pack build
```
- install node project
```
npm install
```

- run vite for dev
```
npm run dev
```

- build vite for prod and preview
```
npx vite build
npx vite preview --host 0.0.0.0
```