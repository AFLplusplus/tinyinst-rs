git clone git@github.com:elbiazo/TinyInst.git
pushd TinyInst
git submodule update --init --recursive
popd

copy src/aflcov.cpp TinyInst/aflcov.cpp
copy src/aflcov.h TinyInst/aflcov.h
copy src/instrumentation.cpp TinyInst/instrumentation.cpp
copy src/instrumentation.h TinyInst/instrumentation.h
copy src/runresult.h TinyInst/runresult.h
copy src/shim.cc TinyInst/shim.cc
copy src/shim.h TinyInst/shim.h
copy src/tinyinstrumentation.cpp TinyInst/tinyinstrumentation.cpp
copy src/tinyinstrumentation.h TinyInst/tinyinstrumentation.h

:: this will give you new litecov.exe to test on with Vector Coverage instead of list coverage 
copy test/tinyinst-coverage.cpp TinyInst/tinyinst-coverage.cpp

cxxbridge ./src/tinyinst.rs -o ./TinyInst/bridge.cc
cxxbridge ./src/tinyinst.rs --header -o ./TinyInst/bridge.h
cmake --build build