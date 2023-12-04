
# OHOLEO KEYBOARD  


## Prerequisites  
    https://tauri.app/v1/guides/getting-started/prerequisites  
	Setup your udev rules to use hidapi.

## Usage  
```
rustup toolchain install nightly  
rustup default nightly  
```
or only for this project
```
rustup toolchain install nightly  
cd <into your project>  
rustup override set nightly  
```
Make sure you've added the wasm32-unknown-unknown target so that Rust can compile your code to WebAssembly to run in the browser.  
```
rustup target add wasm32-unknown-unknown
```
then run
```
npm install -D tailwindcss
npx tailwindcss -i ./input.css -o ./style/output.css
cargo tauri dev
```


## Build  
```
cargo tauri build
```
