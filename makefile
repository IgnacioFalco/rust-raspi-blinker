default:
	rm -rf target
	cargo rustc -- -C link-arg=--script=./linker.ld 
	rm kernel7.img
	/usr/arm-none-eabi/bin/objcopy -O binary target/armv7a-none-eabi/debug/rust-blink ./kernel7.img     