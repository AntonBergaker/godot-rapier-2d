cd src/rapier2d-wrapper
rmdir target/debug
cargo +nightly build --features="single" --target=wasm32-unknown-emscripten -Z build-std=panic_abort,std
mv target/wasm32-unknown-emscripten/debug/librapier2d_wrapper.a target/debug/librapier2d_wrapper.a
mv target/wasm32-unknown-emscripten/debug/librapier2d_wrapper.d target/debug/librapier2d_wrapper.d
cd ../..

scons arch=wasm32 target=template_debug debug_symbols=yes dev_build=yes
rm -rf Godot-Physics-Tests/addons/godot-rapier2d
cp -rf bin/addons/godot-rapier2d Godot-Physics-Tests/addons/godot-rapier2d

