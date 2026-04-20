redis-version := 7.0.12
redis-dir := $(APP)/redis-$(redis-version)
redis-objs := redis-$(redis-version)/src/redis-server.o

app-objs := $(redis-objs)

CFLAGS += -Wno-format

redis-build-args := \
  CC=$(CC) \
  CFLAGS="$(CFLAGS)" \
  USE_JEMALLOC=no \
  -j

# - ARM32 commonly needs atomic runtime symbols at link time
#   (e.g. __atomic_fetch_add_8), see:
#   https://github.com/redis/redis/issues/6275
#   https://github.com/redis/redis/pull/6975
ifeq ($(ARCH), arm)
libatomic := $(shell $(CC) -print-file-name=libatomic.a)
ifneq ($(libatomic),libatomic.a)
	libgcc += $(libatomic)
endif

libgcc-path := $(shell $(CC) -print-libgcc-file-name)
ifneq ($(libgcc-path),)
	libgcc += $(libgcc-path)
endif
endif

ifneq ($(V),)
  redis-build-args += V=$(V)
endif

$(redis-dir):
	@echo "Download redis source code"
	wget https://github.com/redis/redis/archive/$(redis-version).tar.gz -P $(APP)
	tar -zxvf $(APP)/$(redis-version).tar.gz -C $(APP) && rm -f $(APP)/$(redis-version).tar.gz
	cd $(redis-dir) && git init && git add .
	patch -p1 -N -d $(redis-dir) --no-backup-if-mismatch -r - < $(APP)/redis.patch

$(APP)/$(redis-objs): build_redis

build_redis: $(redis-dir)
	cd $(redis-dir) && $(MAKE) $(redis-build-args)

clean_c::
	$(MAKE) -C $(redis-dir) distclean

.PHONY: build_redis clean_c
