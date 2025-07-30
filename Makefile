QEMU_ARGS := -machine virt \
	-nographic \
	-kernel target/riscv64imach-unknown-none/release/hyper-learn \
	-device virtio-net-device,netdev=net0 \
	-netdev user,id=net0 \
	-m 1G

ifeq ($(QEMU_LOG), y)
	QEMU_ARGS += -D qemu.log -d in_asm,int,pcall,cpu_reset,guest_errors
endif

all:

build:
	cargo build --target=rust-target/riscv64imach-unknown-none.json --release -Zbuild-std=core,alloc

run: build
	qemu-system-riscv64 $(QEMU_ARGS)
