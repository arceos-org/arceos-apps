A ?= rust/helloworld
AX_ROOT ?= $(PWD)/.arceos

APP := $(A)
ifeq ($(filter /%,$(A)),)
  ifeq ($(filter ~%,$(A)),)
    APP := $(PWD)/$(A)
  endif
endif

all: build

ax_root:
	@./scripts/set_ax_root.sh $(AX_ROOT)

build run justrun debug fmt disasm disk_img clean clean_c: ax_root
	@make -C $(AX_ROOT) A=$(APP) $@

test:
ifneq ($(filter command line,$(origin A)),)
	@./scripts/app_test.sh $(A)
else
	@./scripts/app_test.sh
endif

.PHONY: all ax_root build run justrun debug fmt disasm disk_img clean clean_c test
