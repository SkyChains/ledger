#*******************************************************************************
#   Ledger App
#   (c) 2018-2020 Zondax GmbH
#   (c) 2017 Ledger
#
#  Licensed under the Apache License, Version 2.0 (the "License");
#  you may not use this file except in compliance with the License.
#  You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.
#*******************************************************************************

ifeq ($(BOLOS_SDK),)
$(error BOLOS_SDK is not set)
endif

MY_DIR := $(dir $(lastword $(MAKEFILE_LIST)))

all: bin/app.elf
	@echo "#!/usr/bin/env bash" > $(OUTPUT_INSTALLER)
	@echo "APPNAME=\"${APPNAME}\"" >> $(OUTPUT_INSTALLER)
	@echo "APPVERSION=\"${APPVERSION}\"" >> $(OUTPUT_INSTALLER)
	@echo "APPPATH=\""${APPPATH}"\"" >> $(OUTPUT_INSTALLER)
	@echo "LOAD_PARAMS=($$(echo "${APP_LOAD_PARAMS}" | sed -e "s|"${APPNAME}"|\""${APPNAME}"\"|"))" >> $(OUTPUT_INSTALLER)
	@echo "DELETE_PARAMS=($$(echo "${COMMON_DELETE_PARAMS}" | sed -e "s|"${APPNAME}"|\""${APPNAME}"\"|"))" >> $(OUTPUT_INSTALLER)
	@echo "APPHEX=\"" >> $(OUTPUT_INSTALLER)
	@cat $(CURDIR)/bin/app.hex >> $(OUTPUT_INSTALLER)
	@echo "\"" >> $(OUTPUT_INSTALLER)
	@cat $(CURDIR)/../deps/template.sh >> $(OUTPUT_INSTALLER)
	@chmod +x $(OUTPUT_INSTALLER)
	@cp $(CURDIR)/bin/* $(ARTIFACT_OUTPUT_PATH)/output
	@cp $(ARTIFACT_OUTPUT_PATH)/output/app.elf ${OUTPUT_ELF}
	@rm $(ARTIFACT_OUTPUT_PATH)/output/app.elf

include $(BOLOS_SDK)/Makefile.defines

DEFINES += APP_SECRET_MODE_ENABLED

$(info ************ TARGET_NAME  = [$(TARGET_NAME)])

ifeq ($(APP_TESTING),1)
DEFINES += APP_TESTING
DEFINES += ZEMU_LOGGING
$(info ************ LOGGING ENABLED ************)
endif

ifndef COIN
COIN=LUX
endif

include $(CURDIR)/Makefile.version

$(info COIN  = [$(COIN)])
ifeq ($(COIN),LUX)
# Main app configuration
DEFINES += APP_STANDARD
APPNAME = "Lux"
APPPATH = "44'/9000'" --path "44'/60'"
else
define error_message
COIN value not supported: [$(COIN)]
endef
$(error "$(error_message)")
endif

APP_LOAD_PARAMS = $(COMMON_LOAD_PARAMS) --path $(APPPATH)

ARTIFACT_OUTPUT_PATH := $(CURDIR)/../build

ifeq ($(TARGET_NAME),TARGET_NANOS)
APP_STACK_SIZE:=3072
ICONNAME:=$(CURDIR)/../nanos_icon.gif
OUTPUT_ELF ?= $(ARTIFACT_OUTPUT_PATH)/output/app_s.elf
OUTPUT_INSTALLER := $(ARTIFACT_OUTPUT_PATH)/pkg/installer_s.sh
RUST_TARGET:=thumbv6m-none-eabi
APP_LOAD_PARAMS += --appFlags 0x000
APP_FULL:=1
endif

ifeq ($(TARGET_NAME),TARGET_NANOX)
ICONNAME:=$(CURDIR)/../nanox_icon.gif
OUTPUT_ELF ?= $(ARTIFACT_OUTPUT_PATH)/output/app_x.elf
OUTPUT_INSTALLER:= $(ARTIFACT_OUTPUT_PATH)/pkg/installer_x.sh
RUST_TARGET:=thumbv6m-none-eabi
APP_LOAD_PARAMS += --appFlags 0x200
APP_FULL:=1
endif

ifeq ($(TARGET_NAME),TARGET_NANOS2)
ICONNAME:=$(CURDIR)/../nanox_icon.gif
OUTPUT_ELF ?= $(ARTIFACT_OUTPUT_PATH)/output/app_sp.elf
OUTPUT_INSTALLER:= $(ARTIFACT_OUTPUT_PATH)/pkg/installer_sp.sh
#RUST_TARGET:=thumbv8m.main-none-eabi
RUST_TARGET:=thumbv6m-none-eabi
APP_LOAD_PARAMS += --appFlags 0x000
APP_FULL:=1
endif

ifeq ($(APP_TESTING),1)
RUST_FEATURES+=--features "dev"
endif
ifeq ($(APP_FULL),1)
RUST_FEATURES+=--features "full"
else
RUST_FEATURES+=--features "lite"
endif

$(info TARGET_NAME  = [$(TARGET_NAME)])
$(info ICONNAME  = [$(ICONNAME)])
$(info OUTPUT_ELF = [$(OUTPUT_ELF)])
$(info OUTPUT_INSTALLR = [$(OUTPUT_INSTALLER)])

ifndef ICONNAME
$(error ICONNAME is not set)
endif

############
# Platform

DEFINES       += UNUSED\(x\)=\(void\)x
DEFINES       += PRINTF\(...\)=

APPVERSION=$(APPVERSION_M).$(APPVERSION_N).$(APPVERSION_P)
DEFINES       += APPVERSION=\"$(APPVERSION)\"

DEFINES       += OS_IO_SEPROXYHAL
DEFINES       += HAVE_BAGL HAVE_SPRINTF
DEFINES       += HAVE_IO_USB HAVE_L4_USBLIB IO_USB_MAX_ENDPOINTS=7 IO_HID_EP_LENGTH=64 HAVE_USB_APDU

DEFINES       += LEDGER_MAJOR_VERSION=$(APPVERSION_M) LEDGER_MINOR_VERSION=$(APPVERSION_N) LEDGER_PATCH_VERSION=$(APPVERSION_P)
DEFINES       += $(TARGET_NAME)

DEFINES       += USB_SEGMENT_SIZE=64
DEFINES       += HAVE_BOLOS_APP_STACK_CANARY

DEFINES       += HAVE_WEBUSB WEBUSB_URL_SIZE_B=0 WEBUSB_URL=""

DEFINES		  += HAVE_ECDSA

DEFINES		  += HAVE_HASH HAVE_SHA256 HAVE_SHA3

ifeq ($(TARGET_NAME),TARGET_NANOS)
DEFINES       += IO_SEPROXYHAL_BUFFER_SIZE_B=128

DEFINES		  += HAVE_BAGL BAGL_WIDTH=128 BAGL_HEIGHT=32
DEFINES		  += BAGL_WIDTH_MARGIN=0
else
# Assume Nano S+/X

DEFINES       += IO_SEPROXYHAL_BUFFER_SIZE_B=300

DEFINES       += HAVE_GLO096
DEFINES       += HAVE_BAGL BAGL_WIDTH=128 BAGL_HEIGHT=64
DEFINES       += HAVE_BAGL_ELLIPSIS # long label truncation feature
DEFINES       += HAVE_BAGL_FONT_OPEN_SANS_REGULAR_11PX
DEFINES       += HAVE_BAGL_FONT_OPEN_SANS_EXTRABOLD_11PX
DEFINES       += HAVE_BAGL_FONT_OPEN_SANS_LIGHT_16PX

DEFINES       += HAVE_UX_FLOW
endif

# X specific
ifeq ($(TARGET_NAME),TARGET_NANOX)
DEFINES       += HAVE_BLE
DEFINES       += HAVE_BLE_APDU BLE_COMMAND_TIMEOUT_MS=2000

SDK_SOURCE_PATH  += lib_blewbxx lib_blewbxx_impl
endif

#Feature temporarily disabled
DEFINES   += LEDGER_SPECIFIC

# Compiler, assembler, and linker

ifneq ($(BOLOS_ENV),)
$(info BOLOS_ENV is $(BOLOS_ENV))
CLANGPATH := /usr/bin/
GCCPATH := /usr/bin/
else
$(info BOLOS_ENV is not set: falling back to CLANGPATH and GCCPATH)
endif

ifeq ($(CLANGPATH),)
$(info CLANGPATH is not set: clang will be used from PATH)
endif

ifeq ($(GCCPATH),)
$(info GCCPATH is not set: arm-none-eabi-* will be used from PATH)
endif

#########################

CC := $(CLANGPATH)clang
CFLAGS += -O3 -Os -Wno-unknown-pragmas -Wno-unused-parameter

AS := $(GCCPATH)arm-none-eabi-gcc
AFLAGS +=

LD       := $(GCCPATH)arm-none-eabi-gcc
LDFLAGS  += -O3 -Os
LDLIBS   += -lrslib
LDLIBS   += -lm -lgcc -lc
LDLIBS   += -L../target/$(RUST_TARGET)/release

##########################
GLYPH_SRC_DIR = glyphs
INCLUDES_PATH += $(MY_DIR)/glyphs
include $(BOLOS_SDK)/Makefile.glyphs

APP_SOURCE_PATH += $(MY_DIR)/c
APP_SOURCE_PATH += $(MY_DIR)/../deps/zxlib

SDK_SOURCE_PATH += lib_stusb lib_stusb_impl
SDK_SOURCE_PATH += lib_ux

.PHONY: rust
rust:
	CARGO_HOME="$(CURDIR)/.cargo" TARGET_NAME=$(TARGET_NAME) ZEMU_LOGGING=$(ZEMU_LOGGING) \
	cargo build --release --target $(RUST_TARGET) \
	--no-default-features $(RUST_FEATURES)

.PHONY: rust_clean
rust_clean:
	CARGO_HOME="$(CURDIR)/.cargo" cargo clean

clean: rust_clean
	rm -r $(ARTIFACT_OUTPUT_PATH)

.PHONY: create_output_folders
create_output_folders:
	- mkdir -p $(ARTIFACT_OUTPUT_PATH)
	- mkdir -p $(ARTIFACT_OUTPUT_PATH)/pkg
	- mkdir -p $(ARTIFACT_OUTPUT_PATH)/output

# Before linking, we need to be sure rust lib is there
bin/app.elf: create_output_folders rust

# load, delete and listvariants are provided to comply with Ledger requirements
.PHONY: load
load:
	python -m ledgerblue.loadApp $(APP_LOAD_PARAMS)

.PHONY: delete
delete:
	python -m ledgerblue.deleteApp $(COMMON_DELETE_PARAMS)

# Import generic rules from the SDK
include $(BOLOS_SDK)/Makefile.rules

#add dependency on custom makefile filename
dep/%.d: %.c Makefile

.PHONY: listvariants
listvariants:
	@echo VARIANTS COIN LUX

.PHONY: version
version:
	@echo "v$(APPVERSION)" > app.version
