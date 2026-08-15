cmake_minimum_required(VERSION 3.24)

project(
    fluidvoice-linux
    VERSION 0.1.0
    DESCRIPTION "Native local-first dictation for KDE Plasma on Wayland"
    LANGUAGES CXX
)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(Qt6 6.6 REQUIRED COMPONENTS Core DBus Widgets)

qt_standard_project_setup()

qt_add_executable(fluidvoice-linux
    src/main.cpp
)

target_link_libraries(fluidvoice-linux PRIVATE
    Qt6::Core
    Qt6::DBus
    Qt6::Widgets
)

target_compile_options(fluidvoice-linux PRIVATE
    $<$<CXX_COMPILER_ID:GNU,Clang>:-Wall -Wextra -Wpedantic -Wconversion>
)

install(TARGETS fluidvoice-linux
    RUNTIME DESTINATION bin
)

