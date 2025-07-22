QEMU_ARGS := -machine virt -nographic -kernel target/riscv64imac-unknown-none-elf/release/hyper-learn

ifeq ($(QEMU_LOG), y)
	QEMU_ARGS += -D qemu.log -d in_asm,int,pcall,cpu_reset,guest_errors
endif

all:

build:
	cargo build --target riscv64imac-unknown-none-elf --release

run: build
	qemu-system-riscv64 $(QEMU_ARGS)
