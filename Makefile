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


flash: build
	dfu-programmer $(ARCH) erase
	dfu-programmer $(ARCH) flash $(TARGET_HEX)
	dfu-programmer $(ARCH) launch

flashalt: build
	ravedude leonardo $(TARGET_ELF)

debug:
	cargo build
	SIMAVR_UART_XTERM=1 simavr -g -m atmega32u4 $(DEBUG_ELF)

debugat:
	avr-gdb $(DEBUG_ELF)

clean:
	rm -rf $(TOPDIR)/target
