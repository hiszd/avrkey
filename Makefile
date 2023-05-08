TARGET := avrkey
SOURCE := src/main.rs

ARCH := atmega32u4
BUILDTARGET := avr-atmega32u4
PROGRAMMER := avr109

TOPDIR := $(shell pwd)
BUILDDIR := $(TOPDIR)/target/$(BUILDTARGET)/release/

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

clean:
	rm -rf $(TOPDIR)/target
