project = keso
src = src
srcaux = srcaux
bin = bin
target = target/thumbv6m-none-eabi/debug/$(project)

ELF2UF2=tools/elf2uf2
OPENOCD=openocd -f interface/picoprobe.cfg -f target/rp2040.cfg -s tcl

.PHONY: all
all: bin

.PHONY: startuplib
startuplib:
	mkdir -p $(bin)
	arm-none-eabi-as -march=armv6-m $(srcaux)/asm.s -o $(bin)/libstartup.o
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

.PHONY: debugrunning
debugrunning:
	$(OPENOCD) >/dev/null 2>&1 & echo "$$!" > .openocd.pid
	sleep 3
	-arm-none-eabi-gdb -command=res/debugrunning.gdb $(target)
	kill -9 `cat .openocd.pid` && rm .openocd.pid

flash: cargo
	$(OPENOCD) -c "init; targets rp2040.core0; program $(target); reset; exit"

reset:
	$(OPENOCD) -c "init; reset; exit"

uart:
	screen /dev/ttyACM0 115200