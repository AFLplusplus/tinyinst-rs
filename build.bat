git clone git@github.com:elbiazo/TinyInst.git
pushd TinyInst
git submodule update --init --recursive
popd

cxxbridge ./src/tinyinst.rs -o ./TinyInst/bridge.cc
cxxbridge ./src/tinyinst.rs --header -o ./TinyInst/bridge.h
cmake --build build