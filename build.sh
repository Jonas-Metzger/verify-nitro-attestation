#wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-12/wasi-sdk-12.0-linux.tar.gz
#tar xvf wasi-sdk-12.0-linux.tar.gz
export WASI_SDK_PATH=`pwd`/wasi-sdk-12.0
#export WASI_SDK_PATH=/home/metzgerj/github/wasi-sdk/wasi-sdk-12.0
export CFLAGS="--sysroot=${WASI_SDK_PATH}/share/wasi-sysroot -D_WASI_EMULATED_SIGNAL -DOPENSSL_NO_SOCK -DOPENSSL_STATIC -DOPENSSL_NO_ASM -DOPENSSL_NO_THREADS -DOPENSSL_NO_AFALGENG"
export CFLAGS="${CFLAGS} -DNO_SYSLOG -DOPENSSL_NO_UI_CONSOLE -DOPENSSL_NO_DGRAM -D_WASI_EMULATED_MMAN -DOPENSSL_SYS_NETWARE -DSIG_DFL=0 -DSIG_IGN=0 -DHAVE_FORK=0 -DOPENSSL_NO_AFALGENG=1 -DOPENSSL_NO_SPEED=1 -DNO_SYSLOG"
#export CFLAGS="${CFLAGS} -DNO_SYSLOG -DOPENSSL_NO_UI_CONSOLE -DOPENSSL_NO_DGRAM -D_WASI_EMULATED_MMAN"
env "CC_wasm32_wasi=${WASI_SDK_PATH}/bin/clang" env "TARGET_CC=${WASI_SDK_PATH}/bin/clang" env "CC=${WASI_SDK_PATH}/bin/clang" env "CFLAGS_wasm32-wasi=${CFLAGS}" cargo build --target=wasm32-wasi
