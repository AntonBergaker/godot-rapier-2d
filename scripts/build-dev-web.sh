cd src/rapier2d-wrapper
rmdir target/debug
export RUSTFLAGS="-C target-feature=+atomics,+bulk-memory"
cargo +nightly build --features="single" --target=wasm32-unknown-emscripten -Z build-std=panic_abort,std
mv target/wasm32-unknown-emscripten/debug/librapier2d_wrapper.a target/debug/librapier2d_wrapper.a
mv target/wasm32-unknown-emscripten/debug/librapier2d_wrapper.d target/debug/librapier2d_wrapper.d
cd ../..

scons arch=wasm32 platform=web target=template_debug
rm -rf Godot-Physics-Tests/addons/godot-rapier2d
cp -rf bin/addons/godot-rapier2d Godot-Physics-Tests/addons/godot-rapier2d
