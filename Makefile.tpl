[+ AutoGen5 template -*- Mode: Makefile -*-
in
+]

srcdir = @srcdir@
builddir = @builddir@

RUSTC = @RUSTC@
RUSTFLAGS = @RUSTFLAGS@
CC = @CC@
CFLAGS = @CFLAGS@

INSTALL = @INSTALL@

RUST_TARGET_PATH = @RUST_TARGET_PATH@

prefix = @prefix@
includedir = @libdir@

bootdir = @bootdir@
kernel_libdir = @kernel_libdir@
search_paths = @search_paths@
base_dirs = @base_dirs@
kernel_name = @kernel_name@

MKDIR_P = @MKDIR_P@

# all

[+define vars+] CC="$(CC)" CFLAGS="$(CFLAGS)" RUSTC="$(RUSTC)" RUSTFLAGS="$(RUSTFLAGS)" prefix="$(prefix)" includedir="$(includedir)" kernel_libdir="$(kernel_libdir)" bootdir="$(bootdir)" RUST_TARGET_PATH="$(RUST_TARGET_PATH)" [+enddef+]

[+define make_rule+]+$(MAKE) [+vars+][+enddef+]

[+define configure_rule+]../$(srcdir)/[+target+]/configure [+vars+] -prefix=$(prefix) --includedir=$(includedir) --with-kernel-libdir=$(kernel_libdir) --with-bootdir=$(bootdir) --build=@build@ --host=@host@[+enddef+]

all: Makefile
	touch stamp
[+for targets+]	[+make_rule+] [+target+]/all 
[+endfor+]


Makefile: config.status
	./config.status Makefile

config.status: @srcdir@/configure
	./config.status --recheck

clean:
[+for targets+]	[+make_rule+] [+target+]/clean 
[+endfor+]

install:
[+for targets+]	[+make_rule+] [+target+]/install 
[+endfor+]

install-strip:
[+for targets+]	[+make_rule+] [+target+]/install-strip 
[+endfor+]

# configure and build subdirs

[+for targets+]

.PHONY: configure-[+target+] [+target+]/all [+target+]/install [+target+]/install-strip [+target+]/clean

configure-[+target+]:
	$(MKDIR_P) [+target+]
	cd [+target+] && [+configure_rule+]

[+target+]/: 
	[+make_rule+] configure-[+target+]

[+target+]/all: [+target+]/
	[+make_rule+] -C [+target+] all

[+target+]/install: [+target+]/
	[+make_rule+] -C [+target+] install

[+target+]/install-strip: [+target+]/
	[+make_rule+] -C [+target+] install-strip

[+target+]/clean: [+target+]/
	[+make_rule+] -C [+target+] clean
[+endfor+]