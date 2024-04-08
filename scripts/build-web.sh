cd src/rapier2d-wrapper
rmdir target/release
export RUSTFLAGS="-C target-feature=+atomics,+bulk-memory"
#export EMCC_FORCE_STDLIBS=1
cargo +nightly build --release --features="single,wasm-bindgen" --target=wasm32-unknown-emscripten -Z build-std=panic_abort,std
mv target/wasm32-unknown-emscripten/release/librapier2d_wrapper.a target/release/librapier2d_wrapper.a
mv target/wasm32-unknown-emscripten/release/librapier2d_wrapper.d target/release/librapier2d_wrapper.d
cd ../..

scons arch=wasm32 platform=web target=template_release
rm -rf Godot-Physics-Tests/addons/godot-rapier2d
cp -rf bin/addons/godot-rapier2d Godot-Physics-Tests/addons/godot-rapier2d

