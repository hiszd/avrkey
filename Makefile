TARGET := avrkey
SOURCE := src/main.rs

ARCH := atmega32u4
BUILDTARGET := avr-atmega32u4
PROGRAMMER := avr109

TOPDIR := $(shell pwd)
BUILDDIR := $(TOPDIR)/target/$(BUILDTARGET)/release/
DEBUGDIR := $(TOPDIR)/target/$(BUILDTARGET)/debug/

DEBUG_ELF := $(DEBUGDIR)/$(TARGET).elf

TARGET_ELF := $(BUILDDIR)/$(TARGET).elf
TARGET_HEX := $(BUILDDIR)/$(TARGET).hex

all: build flash

build:
	cargo build --release
	avr-objcopy -O ihex $(TARGET_ELF) $(TARGET_HEX)


flashserdfu: build
	dfu-programmer $(ARCH) erase
	dfu-programmer $(ARCH) flash $(TARGET_HEX)
	dfu-programmer $(ARCH) launch
	sudo minicom -D /dev/ttyACM0 -w

flashdfu: build
	dfu-programmer $(ARCH) erase
	dfu-programmer $(ARCH) flash $(TARGET_HEX)
	dfu-programmer $(ARCH) launch

flashseravr: build
	ravedude leonardo $(TARGET_ELF)
	sudo minicom -D /dev/ttyACM0 -w

flashavr: build
	ravedude leonardo $(TARGET_ELF)

debug:
	cargo build --release
	SIMAVR_UART_XTERM=1 simavr -g -f 16000000 -m atmega32u4 $(TARGET_ELF)

debugat:
	avr-gdb $(TARGET_ELF) -x .gdb/debug.gdb

clean:
	rm -rf $(TOPDIR)/target
