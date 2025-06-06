CARGO      := cargo
PROG       := target/release/toml2yaml
INPUT      := oxocarbon.toml
OUTDIR     := themes
OUTFILE    := $(OUTDIR)/oxocarbon.yaml
OLEDFILE   := $(OUTDIR)/oxocarbon-oled.yaml

.PHONY: all build clean

all: build $(OUTFILE) $(OLEDFILE)

build:
	$(CARGO) build --release

$(OUTFILE): build $(INPUT)
	mkdir -p $(OUTDIR)
	$(PROG) $(INPUT) > $(OUTFILE)

$(OLEDFILE): build $(INPUT)
	mkdir -p $(OUTDIR)
	$(PROG) --oled $(INPUT) > $(OLEDFILE)

clean:
	$(CARGO) clean
	rm -f $(OUTFILE) $(OLEDFILE)