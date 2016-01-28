rustc -O -Z no-landing-pads --target thumbv7m-none-eabi -g $1/src/libcore/lib.rs
rustc -O -Z no-landing-pads --target thumbv7m-none-eabi -g $1/src/liballoc/lib.rs -L . --cfg feature=\"external_funcs\"
rustc -O -Z no-landing-pads --target thumbv7m-none-eabi -g $1/src/librustc_unicode/lib.rs -L .
rustc -O -Z no-landing-pads --target thumbv7m-none-eabi -g $1/src/libcollections/lib.rs -L .
