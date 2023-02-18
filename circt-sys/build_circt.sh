#!/bin/sh
CIRCT_SRC_DIR=$PWD/circt
BUILD_DIR=$CIRCT_SRC_DIR/build

cmake -B $BUILD_DIR -G Ninja $CIRCT_SRC_DIR/llvm/llvm \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_INSTALL_PREFIX=/usr/local \
    -DCMAKE_C_COMPILER=clang \
    -DCMAKE_CXX_COMPILER=clang++ \
    -DCMAKE_EXPORT_COMPILE_COMMANDS=ON \
    -DLLVM_TARGETS_TO_BUILD=host \
    -DLLVM_ENABLE_PROJECTS=mlir \
    -DLLVM_ENABLE_ASSERTIONS=ON \
    -DLLVM_EXTERNAL_PROJECTS=circt \
    -DLLVM_EXTERNAL_CIRCT_SOURCE_DIR=$CIRCT_SRC_DIR \
    -DLLVM_STATIC_LINK_CXX_STDLIB=ON \
    -DLLVM_OPTIMIZED_TABLEGEN=ON \
    -DLLVM_INSTALL_UTILS=ON \
    -DMLIR_ENABLE_BINDINGS_PYTHON=ON \
    -DMLIR_INSTALL_AGGREGATE_OBJECTS=OFF \
    -DCIRCT_ENABLE_FRONTENDS=ON \
    -DCIRCT_INCLUDE_DOCS=ON \
    -DCIRCT_BINDINGS_PYTHON_ENABLED=ON

cmake --build $BUILD_DIR --target install
