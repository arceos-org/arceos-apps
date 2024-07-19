A ?= rust/helloworld
AX_ROOT ?= $(PWD)/.arceos

APP := $(A)
ifeq ($(filter /%,$(A)),)
  ifeq ($(filter ~%,$(A)),)
    APP := $(PWD)/$(A)
  endif
endif

all:
	@make -C $(AX_ROOT) A=$(APP)

$(MAKECMDGOALS):
	@make -C $(AX_ROOT) A=$(APP) $(MAKECMDGOALS)

test:
ifneq ($(filter command line,$(origin A)),)
	./scripts/app_test.sh $(A)
else
	./scripts/app_test.sh
endif
