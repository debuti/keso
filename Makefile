project = keso
src = src
srcaux = srcaux
bin = bin
target = target/thumbv6m-none-eabi/debug/$(project)

ELF2UF2=tools/elf2uf2
OPENOCD=openocd -f interface/picoprobe.cfg -f target/rp2040.cfg -s tcl
OPENOCD0=openocd -f interface/picoprobe.cfg -f target/rp2040-core0.cfg -s tcl
OPENOCD1=openocd -f interface/picoprobe.cfg -f target/rp2040-core1.cfg -s tcl

.PHONY: all
all: bin objdump-disassembly objdump-hexdump

.PHONY: startuplib
startuplib:
	mkdir -p $(bin)
	arm-none-eabi-as -march=armv6s-m $(srcaux)/asm.s -o $(bin)/libstartup.o
	ar crs $(bin)/thumbv6m-none-eabi.a $(bin)/libstartup.o

.PHONY: cargo
cargo: startuplib
	cargo build

.PHONY: uf2
uf2: cargo
	$(ELF2UF2) $(target) $(target).uf2

.PHONY: bin
bin: cargo
	arm-none-eabi-objcopy $(target) -O binary $(target).bin

.PHONY: clean
clean:
	-rm -rf $(target) $(target).bin $(target).uf2

.PHONY: debug
debug:
	$(OPENOCD) >/dev/null 2>&1 & echo "$$!" > .openocd.pid
	sleep 3
	-arm-none-eabi-gdb --command=res/debug.gdb $(target)
	kill -9 `cat .openocd.pid` && rm .openocd.pid

.PHONY: debugrunning0
debugrunning0:
	$(OPENOCD0) >/dev/null 2>&1 & echo "$$!" > .openocd.pid
	sleep 3
	-arm-none-eabi-gdb -command=res/debugrunning.gdb $(target)
	kill -9 `cat .openocd.pid` && rm .openocd.pid

.PHONY: debugrunning1
debugrunning1:
	$(OPENOCD1) >/dev/null 2>&1 & echo "$$!" > .openocd.pid
	sleep 3
	-arm-none-eabi-gdb -command=res/debugrunning.gdb $(target)
	kill -9 `cat .openocd.pid` && rm .openocd.pid

.PHONY: flash
flash: cargo
	$(OPENOCD) -c "init; targets rp2040.core0; program $(target); reset; exit"

.PHONY: reset
reset:
	$(OPENOCD) -c "init; reset; exit"

.PHONY: uart
uart:
	screen /dev/ttyACM0 115200

.PHONY: objdump-disassembly
objdump-disassembly:
	-arm-none-eabi-objdump -d $(target) > $(target).dis

.PHONY: objdump-hexdump
objdump-hexdump:
	-arm-none-eabi-objdump -s $(target) > $(target).dump

