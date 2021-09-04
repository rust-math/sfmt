
SFMT_VERSION := 1.5.1
SFMT_DIR     := SFMT-src-$(SFMT_VERSION)
SFMT_ARCHIVE := $(SFMT_DIR).tar.gz

CXX := g++
CXX_FLAGS := -O2 -msse2 -DHAVE_SSE2

MEXPS := 607 1279 2281 4253 11213 19937 44497 86243 132049 216091
U64_REFERENCES := $(foreach MEXP,$(MEXPS),u64_$(MEXP).txt)

all: $(U64_REFERENCES)

$(SFMT_DIR)/SFMT.c:
	wget http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/SFMT/$(SFMT_ARCHIVE)
	tar xf $(SFMT_ARCHIVE)
	rm -f $(SFMT_ARCHIVE)

clean:
	rm -rf $(SFMT_DIR)
	rm -f $(U64_REFERENCES)

define generate_u64
	$(CXX) $(CXX_FLAGS) -DSFMT_MEXP=$(1) $^
	./a.out > u64_$(1).txt
	rm a.out

endef

$(U64_REFERENCES): sample.cpp $(SFMT_DIR)/SFMT.c
	$(foreach MEXP,$(MEXPS),$(call generate_u64,$(MEXP)))
