# `MP4 Inspector`
> A Web-based MP4 File Inspector. Powered by Rust, Vue and Web Assembly! 🦀.

https://alfg.github.io/mp4-inspector/

## Development
`mp4-inspector` is a [Vue.js](https://vuejs.org/) application importing a [Web Assembly](https://webassembly.org/) module built with [Rust](https://www.rust-lang.org) and [mp4-rust](https://github.com/alfg/mp4-rust) via [wasm-pack](https://rustwasm.github.io/wasm-pack).

### Requirements
* `rust` - https://www.rust-lang.org/learn/get-started
* `wasm-pack` - https://rustwasm.github.io/wasm-pack/installer/
* `nodejs` - https://nodejs.org/en/download/

### Setup 
* Clone project and build the Wasm module:
```
git clone https://github.com/alfg/mp4-inspector.git && cd mp4-inspector
wasm-pack build
```

* Install and run web:
```
cd www
npm install
npm run serve
```

* Load `http://localhost:8080/` in the web browser.

### Compiles and minifies for production
```
npm run build
```

### Deploy
Deploys to [Github Pages](https://pages.github.com/)
```
npm run deploy
```

### Docker
```
docker build -t mp4-inspector .
docker run -it -p 8080:80 --rm mp4-inspector
```

http://localhost:8080/mp4-inspector/

### TODO
* Examples
* Load by URL
* Tests

### Resources
* https://rust-lang.org
* https://github.com/alfg/mp4-rust
* https://rustwasm.github.io/wasm-pack
* https://vuejs.org
* https://bootstrap-vue.org

## License
MIT
