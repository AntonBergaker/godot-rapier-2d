#!/usr/bin/env python
import os
import sys

env = SConscript("godot-cpp/SConstruct")

## Libs
if env["dev_build"]:
    lib_folder = "src/rapier2d-wrapper/target/debug"
else:
    lib_folder = "src/rapier2d-wrapper/target/release"
if env["platform"] == "ios":
	env.Append(LINKFLAGS='-framework Security')
	
if env["platform"] == "windows":
	lib_file = "rapier2d_wrapper{}"
	lib = lib_file.format(env["LIBSUFFIX"])
else:
	lib_file = "librapier2d_wrapper{}"
	lib = lib_file.format(env["LIBSUFFIX"])
env.Append(LIBPATH=[lib_folder])
env.Append(LIBS=[lib])

## Sources
env.Append(CPPPATH=["src/"])
sources = [Glob("src/*.cpp"),Glob("src/bodies/*.cpp"),Glob("src/joints/*.cpp"),Glob("src/servers/*.cpp"),Glob("src/shapes/*.cpp"),Glob("src/spaces/*.cpp")]

if env["platform"] == "windows":
    env.Append(CPPDEFINES="WINDOWS_ENABLED")

if env["precision"] == "double":
	env.Append(CPPDEFINES=["REAL_T_IS_DOUBLE"])
else:
	env.Append(CPPDEFINES=["REAL_T_IS_FLOAT"])
if env["platform"] == "web":
    env.Append(CPPFLAGS=["-s", "EMULATE_FUNCTION_POINTER_CASTS=1"])
    env.Append(LINKFLAGS=["-s", "EMULATE_FUNCTION_POINTER_CASTS=1"])
    env.Append(CPPFLAGS=["-s", "SAFE_HEAP=1"])
    env.Append(LINKFLAGS=["-s", "SAFE_HEAP=1"])
    #env.Append(CPPFLAGS=["-s", "ASSERTIONS=0"])
    #env.Append(LINKFLAGS=["-s", "ASSERTIONS=0"])
    env.Append(CPPFLAGS=["-s", "WARN_UNALIGNED=1"])
    env.Append(LINKFLAGS=["-s", "WARN_UNALIGNED=1"])
    #env.Append(CPPFLAGS=["-s", "EXPORT_ALL=1"])
    #env.Append(LINKFLAGS=["-s", "EXPORT_ALL=1"])
    env.Append(CPPFLAGS=["-s", "BINARYEN_EXTRA_PASSES=\"--pass-arg=max-func-params@18\""])
    env.Append(LINKFLAGS=["-s", "BINARYEN_EXTRA_PASSES=\"--pass-arg=max-func-params@18\""])
    #env.Append(CPPFLAGS=["-W", "bad-function-cast"])
    #env.Append(CPPFLAGS=["-W", "cast-function-type"])
    #env.Append(CPPFLAGS=["-s", "ALLOW_MEMORY_GROWTH=1"])
    #env.Append(LINKFLAGS=["-s", "ALLOW_MEMORY_GROWTH=1"])
	
if env["platform"] == "macos":
	library = env.SharedLibrary(
		"bin/addons/godot-rapier2d/bin/libphysics_server_rapier2d.{}.{}.framework/libphysics_server_rapier2d.{}.{}.dylib".format(
			env["platform"], env["target"], env["platform"], env["target"]
		),
		source=sources,
	)
elif env["platform"] == "ios":
	library = env.SharedLibrary(
		"bin/addons/godot-rapier2d/bin/libphysics_server_rapier2d.{}.{}.framework/libphysics_server_rapier2d.{}.{}.dylib".format(
			env["platform"], env["target"], env["platform"], env["target"]
		),
		source=sources,
	)
else:
	library = env.SharedLibrary(
		"bin/addons/godot-rapier2d/bin/libphysics_server_rapier2d{}{}".format(env["suffix"], env["SHLIBSUFFIX"]),
		source=sources,
	)

Default(library)
