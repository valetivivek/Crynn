cmake_minimum_required(VERSION 3.16)
project(crynn_browser VERSION 0.1.0 LANGUAGES CXX Rust)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find Rust
find_program(CARGO cargo REQUIRED)

# Find GeckoView using pkg-config
find_package(PkgConfig REQUIRED)

# Try to find GeckoView
pkg_check_modules(GECKOVIEW geckoview>=91.0)

if(GECKOVIEW_FOUND)
    message(STATUS "Found GeckoView ${GECKOVIEW_VERSION}")
    message(STATUS "GeckoView include directories: ${GECKOVIEW_INCLUDE_DIRS}")
    message(STATUS "GeckoView libraries: ${GECKOVIEW_LIBRARIES}")
    
    include_directories(${GECKOVIEW_INCLUDE_DIRS})
    link_directories(${GECKOVIEW_LIBRARY_DIRS})
    add_definitions(${GECKOVIEW_CFLAGS_OTHER})
    add_definitions(-DHAVE_GECKOVIEW)
    
    # Link against GeckoView libraries
    set(GECKOVIEW_LINK_LIBS ${GECKOVIEW_LIBRARIES})
    
else()
    message(WARNING "GeckoView not found. Trying fallback Gecko detection...")
    
    # Fallback: try to find Gecko manually
    find_path(GECKO_INCLUDE_DIR
        NAMES nsThreadUtils.h
        PATHS
            /usr/include/firefox
            /usr/local/include/firefox
            /opt/firefox/include
            /Applications/Firefox.app/Contents/MacOS
            "C:/Program Files/Mozilla Firefox"
            "C:/Program Files (x86)/Mozilla Firefox"
            ${CMAKE_SOURCE_DIR}/third_party/gecko/include
        NO_DEFAULT_PATH
    )
    
    find_library(GECKO_LIBRARY
        NAMES xul gecko
        PATHS
            /usr/lib/firefox
            /usr/local/lib/firefox
            /opt/firefox
            /Applications/Firefox.app/Contents/MacOS
            "C:/Program Files/Mozilla Firefox"
            "C:/Program Files (x86)/Mozilla Firefox"
        NO_DEFAULT_PATH
    )
    
    if(GECKO_INCLUDE_DIR AND GECKO_LIBRARY)
        message(STATUS "Found Gecko headers at: ${GECKO_INCLUDE_DIR}")
        message(STATUS "Found Gecko library at: ${GECKO_LIBRARY}")
        include_directories(${GECKO_INCLUDE_DIR})
        add_definitions(-DHAVE_GECKO)
        set(GECKOVIEW_LINK_LIBS ${GECKO_LIBRARY})
    else()
        message(WARNING "Gecko headers/library not found. Building without Gecko integration.")
        message(STATUS "To enable GeckoView support, install GeckoView development packages:")
        message(STATUS "  Ubuntu/Debian: sudo apt-get install libgeckoview-dev")
        message(STATUS "  Fedora/RHEL: sudo yum install geckoview-devel")
        message(STATUS "  macOS: brew install geckoview")
        add_definitions(-DNO_GECKO)
    endif()
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
