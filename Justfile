build_configure:
    cmake -S ./test -B ./test/build -DCMAKE_BUILD_TYPE=Debug

build_test: build_configure
    cmake --build ./test/build --config Debug
