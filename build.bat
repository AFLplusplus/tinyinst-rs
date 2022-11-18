git clone git@github.com:elbiazo/TinyInst.git
pushd TinyInst
git submodule update --init --recursive
popd

 copy src\* .\TinyInst\

:: this will give you new litecov.exe to test on with Vector Coverage instead of list coverage 
copy test\tinyinst-coverage.cpp TinyInst\tinyinst-coverage.cpp

cxxbridge ./src/tinyinst.rs -o ./TinyInst/bridge.cc
cxxbridge ./src/tinyinst.rs --header -o ./TinyInst/bridge.h
cmake --build build