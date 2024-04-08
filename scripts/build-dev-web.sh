cd src/rapier2d-wrapper
cargo +nightly build --features="single" --target=wasm32-unknown-emscripten -Z build-std=panic_abort,std
mv target/wasm32-unknown-emscripten/debug/ target/debug/
cd ../..

scons arch=wasm32 target=template_debug debug_symbols=yes dev_build=yes
rm -rf Godot-Physics-Tests/addons/godot-rapier2d
cp -rf bin/addons/godot-rapier2d Godot-Physics-Tests/addons/godot-rapier2d

