PHONY: .flash

target/avr-atmega328p/release/rustyspa2.elf: src/main.rs
	cargo build --release

flash: target/avr-atmega328p/release/rustyspa2.elf
	avrdude -patmega328p -carduino -P/dev/cu.usbserial-2210 -b115200 -D -Uflash:w:target/avr-atmega328p/release/rustyspa2.elf:e
