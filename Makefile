A ?= rust/helloworld
AX_ROOT ?= $(shell cat .axroot 2>/dev/null)

APP := $(A)
ifeq ($(filter /%,$(A)),)
  ifeq ($(filter ~%,$(A)),)
    APP := $(PWD)/$(A)
  endif
endif

$(if $(V), $(info AX_ROOT: "$(AX_ROOT)"))

all: build

chaxroot:
	@./scripts/set_ax_root.sh $(AX_ROOT)

defconfig oldconfig build run justrun debug disasm disk_img clean clean_c:
	@make -C $(AX_ROOT) A=$(APP) $@

test:
ifneq ($(filter command line,$(origin A)),)
	@./scripts/app_test.sh $(A)
else
	@./scripts/app_test.sh
endif

.PHONY: all chaxroot defconfig oldconfig build run justrun debug disasm disk_img clean clean_c test
