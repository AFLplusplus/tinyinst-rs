cmake_minimum_required(VERSION 3.10)
project("test")

# Disable implicit function declaration warning on macOS so it can compile vulnerable code 
if(APPLE)
    set(CMAKE_C_FLAGS "-Wno-error=implicit-function-declaration")
endif()

add_executable(test test.c)