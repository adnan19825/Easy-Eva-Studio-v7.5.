cmake_minimum_required(VERSION 3.16)
project(hybrid-kem C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

# Finde OpenSSL
find_package(OpenSSL REQUIRED)

# Quellcodedateien
set(SOURCES
    src/hybrid_kem.c
    src/kem/kyber.c
    src/kem/dilithium.c
    src/utils/random.c
    src/utils/errors.c
)

# Include-Verzeichnisse
include_directories(
    include
    ${OPENSSL_INCLUDE_DIR}
)

# Hauptbibliothek
add_library(hybrid-kem SHARED ${SOURCES})

# OpenSSL linken
target_link_libraries(hybrid-kem OpenSSL::SSL OpenSSL::Crypto)

# Beispiele
add_executable(example examples/basic_example.c)
target_link_libraries(example hybrid-kem)
