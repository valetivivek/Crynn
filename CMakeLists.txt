cmake_minimum_required(VERSION 3.16)
project(crynn_browser VERSION 0.1.0 LANGUAGES CXX Rust)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find Rust
find_program(CARGO cargo REQUIRED)

# Find Gecko (if available)
find_path(GECKO_INCLUDE_DIR
    NAMES nsThreadUtils.h
    PATHS
        /usr/include/firefox
        /usr/local/include/firefox
        /opt/firefox/include
        ${CMAKE_SOURCE_DIR}/third_party/gecko/include
    NO_DEFAULT_PATH
)

if(GECKO_INCLUDE_DIR)
    message(STATUS "Found Gecko headers at: ${GECKO_INCLUDE_DIR}")
    include_directories(${GECKO_INCLUDE_DIR})
    add_definitions(-DHAVE_GECKO)
else()
    message(WARNING "Gecko headers not found. Building without Gecko integration.")
    add_definitions(-DNO_GECKO)
endif()

# Rust build configuration
set(RUST_PROFILE "release")
set(CARGO_FLAGS "--release --package crynn-shell")

# Add custom target for Rust build
add_custom_target(crynn_shell
    COMMAND ${CARGO} build ${CARGO_FLAGS}
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    COMMENT "Building Crynn shell with Cargo"
    VERBATIM
)

# Memory optimization flags for Rust
set(RUSTFLAGS "-C target-cpu=native -C opt-level=z -C panic=abort -C strip=symbols -C lto=fat -C codegen-units=1")
set(ENV{RUSTFLAGS} ${RUSTFLAGS})

# Create install target
install(TARGETS crynn_shell
    RUNTIME DESTINATION bin
)

# Add memory benchmark target
add_custom_target(memory_benchmark
    COMMAND ${CMAKE_SOURCE_DIR}/build/scripts/memory_bench.sh
    DEPENDS crynn_shell
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    COMMENT "Running memory benchmark"
    VERBATIM
)

# Add optimized build target
add_custom_target(build_optimized
    COMMAND ${CMAKE_SOURCE_DIR}/build/scripts/build_optimized.sh
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    COMMENT "Building with all optimizations"
    VERBATIM
)

# Print configuration summary
message(STATUS "")
message(STATUS "Crynn Browser Configuration Summary:")
message(STATUS "====================================")
message(STATUS "Build type: ${CMAKE_BUILD_TYPE}")
message(STATUS "Rust profile: ${RUST_PROFILE}")
message(STATUS "Gecko integration: ${GECKO_INCLUDE_DIR}")
message(STATUS "Rust flags: ${RUSTFLAGS}")
message(STATUS "")
message(STATUS "Available targets:")
message(STATUS "  crynn_shell          - Build the main browser")
message(STATUS "  memory_benchmark     - Run memory benchmark")
message(STATUS "  build_optimized      - Build with all optimizations")
message(STATUS "")
