#wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-12/wasi-sdk-12.0-linux.tar.gz
#tar xvf wasi-sdk-12.0-linux.tar.gz
#rm wasi-sdk-12.0-linux.tar.gz
export WASI_SDK_PATH=$PWD/wasi-sdk-12.0
#export CFLAGS="--sysroot=${WASI_SDK_PATH}/share/wasi-sysroot"
export CFLAGS="${CFLAGS} -D_WASI_EMULATED_SIGNAL -DOPENSSL_NO_SOCK -DOPENSSL_STATIC -DOPENSSL_NO_AFALGENG  -DOPENSSL_SYS_NETWARE"
export CFLAGS="${CFLAGS} -DNO_SYSLOG -DOPENSSL_NO_UI_CONSOLE -DOPENSSL_NO_DGRAM -D_WASI_EMULATED_MMAN -DOPENSSL_NO_SECURE_MEMORY"
CC=wasicc CFLAGS=$CFLAGS cargo build --target=wasm32-wasi

# export CFLAGS="${CFLAGS} -DNO_SYSLOG -DOPENSSL_NO_UI_CONSOLE -DOPENSSL_NO_DGRAM -D_WASI_EMULATED_MMAN -DOPENSSL_SYS_NETWARE -DSIG_DFL=0 -DSIG_IGN=0 -DHAVE_FORK=0 -DOPENSSL_NO_AFALGENG=1 -DOPENSSL_NO_SPEED=1 -DNO_SYSLOG"
#export CFLAGS="${CFLAGS}  -DNO_SYSLOG -DOPENSSL_NO_UI_CONSOLE -DOPENSSL_NO_DGRAM -D_WASI_EMULATED_MMAN -DOPENSSL_SYS_NETWARE -DOPENSSL_NO_SECURE_MEMORY"
# env "CC_wasm32_wasi=${WASI_SDK_PATH}/bin/clang" env "TARGET_CC=${WASI_SDK_PATH}/bin/clang" env "CC=${WASI_SDK_PATH}/bin/clang" env "CFLAGS_wasm32-wasi=${CFLAGS}" cargo build --target=wasm32-wasi
#CARGO_TARGET_WASM32_WASI_LINKER=$WASI_SDK_PATH cargo build --target=wasm32-wasi -C target-feature=-crt-static
#CC_wasm32_wasi=$WASI_SDK_PATH/bin/clang CARGO_TARGET_WASM32_WASI_LINKER=$WASI_SDK_PATH cargo build --target=wasm32-wasi -C target-feature=-crt-static
#CC_wasm32_wasi=$WASI_SDK_PATH/bin/clang CARGO_TARGET_WASM32_WASI_LINKER=$WASI_SDK_PATH/bin/clang CFLAGS=$CFLAGS cargo build --target=wasm32-wasi
#CC_wasm32_wasi=$WASI_SDK_PATH/bin/clang CFLAGS=$CFLAGS cargo build --target=wasm32-wasi
