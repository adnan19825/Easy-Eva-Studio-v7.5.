cmake_minimum_required(VERSION 3.10)
project(hybrid-kem VERSION 0.1.0 LANGUAGES C)

set(CMAKE_C_STANDARD 99)
add_compile_options(-Wall -Wextra)

find_package(OpenSSL REQUIRED)
find_library(OQS_LIB oqs HINTS $ENV{PREFIX}/lib)
find_path(OQS_INCLUDE_DIR oqs/oqs.h HINTS $ENV{PREFIX}/include)

include_directories(include ${OPENSSL_INCLUDE_DIR} ${OQS_INCLUDE_DIR})

add_library(hybrid_kem STATIC
    src/hybrid/hybrid.c
    src/kem/kyber.c
    src/kem/dilithium.c
    src/utils/random.c
)

target_link_libraries(hybrid_kem ${OQS_LIB} OpenSSL::Crypto)

add_executable(test_hybrid tests/test_hybrid_full.c)
target_link_libraries(test_hybrid hybrid_kem)

add_executable(benchmark_hybrid tests/benchmark_hybrid.c)
target_link_libraries(benchmark_hybrid hybrid_kem)

