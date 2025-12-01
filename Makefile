A ?= rust/helloworld

APP := $(A)
ifeq ($(filter /%,$(A)),)
  ifeq ($(filter ~%,$(A)),)
    APP := $(PWD)/$(A)
  endif
endif

all: build

defconfig oldconfig build run justrun debug disasm disk_img clean clean_c:
	@make -C arceos A=$(APP) $@

test:
ifneq ($(filter command line,$(origin A)),)
	@./scripts/app_test.sh $(A)
else
	@./scripts/app_test.sh
endif

.PHONY: all chaxroot defconfig oldconfig build run justrun debug disasm disk_img clean clean_c test
