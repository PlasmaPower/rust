-include ../tools.mk

ifdef IS_MSVC
# FIXME(#27979)
all:
else
all:
	$(RUSTC) foo.rs -C panic=abort
	$(RUSTC) bar.rs -C panic=abort
	$(RUSTC) main.rs
	$(call RUN,main)
endif
