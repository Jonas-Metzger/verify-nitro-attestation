# wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${WASI_VERSION}/wasi-sdk-12.0-linux.tar.gz
# tar xvf wasi-sdk-12.0-linux.tar.gz
# export WASI_SDK_PATH=`pwd`/wasi-sdk-12.0
export WASI_SDK_PATH=/home/metzgerj/github/wasi-sdk/wasi-sdk-12.0
export CFLAGS="--sysroot=${WASI_SDK_PATH}/share/wasi-sysroot -D_WASI_EMULATED_SIGNAL -OPENSSL_NO_SOCK -DOPENSSL_STATIC"
export CFLAGS="${CFLAGS} -DNO_SYSLOG -DOPENSSL_NO_UI -DOPENSSL_NO_UI_CONSOLE -DOPENSSL_NO_DGRAM -D_WASI_EMULATED_MMAN"
env "CFLAGS_wasm32-wasi=${CFLAGS}" cargo build --target=wasm32-wasi
