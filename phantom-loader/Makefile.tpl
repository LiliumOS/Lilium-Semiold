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

rust_bin_prefix=@rust_bin_prefix@
rust_bin_suffix=@rust_bin_suffix@
rust_rlib_prefix=@rust_rlib_prefix@
rust_rlib_suffix=@rust_rlib_suffix@
rust_staticlib_prefix=@rust_staticlib_prefix@
rust_staticlib_suffix=@rust_staticlib_suffix@
rust_dylib_prefix=@rust_dylib_prefix@
rust_dylib_suffix=@rust_dylib_suffix@
rust_cdylib_prefix=@rust_cdylib_prefix@
rust_cdylib_suffix=@rust_cdylib_suffix@


[+for deps+]
[+dep_name+]_DIR = [+dir+]
[+dep_name+]_FILE = $([+dep+]_DIR)/$(rust_[+crate_type+]_prefix)[+dep_name+]$(rust_[+crate_type+]_suffix)
[+endfor+]


all: stamp

Makefile: config.status
    ./config.status Makefile 
    
config.status: @srcdir@/configure
    ./config.status --recheck

[+for outputs+]
$(rust_[+crate_type+]_prefix)[+crate_name+]$(rust_[+crate_type+]_suffix).d: $(foreach dep,[+deps+],$($(dep)_DIR)/stamp)
    $(RUSTC) $(RUSTFLAGS) --crate-name [+crate_name+] --crate-type [+crate_type+] [+for deps+]--extern "[+dep_name+]=$([+dep_name+]_FILE)" [+endfor+] --emit dep-info=$@ -o $(rust_[+crate_type+]_prefix)[+crate_name+]$(rust_[+crate_type+]_suffix)

include $(rust_[+crate_type+]_prefix)[+crate_name+]$(rust_[+crate_type+]_suffix).d

$(rust_[+crate_type+]_prefix)[+crate_name+]$(rust_[+crate_type+]_suffix): $(foreach dep,[+deps+],$($(dep)_DIR)/stamp) Makefile 
    $(RUSTC) $(RUSTFLAGS) --crate-name [+crate_name+] --crate-type [+crate_type+] [+for deps+]--extern "[+dep_name+]=$([+dep_name+]_FILE)" [+endfor+] --emit link=$@ -o $@
stamp: $(rust_[+crate_type+]_prefix)[+crate_name+]$(rust_[+crate_type+]_suffix)
[+endfor+]

[+for nextrules+]

[+makefile+]: config.status
    ./config.status [+makefile+]


include [+makefile+]
[+endfor+]

stamp: 
    touch stamp