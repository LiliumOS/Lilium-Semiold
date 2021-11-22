AC_INIT([install-sh])



AC_CONFIG_MACRO_DIRS([m4/])

AC_CANONICAL_BUILD
AC_CANONICAL_HOST

case "$host" in
    *-*-phantom-kernel )
        ;;
    * )
        AC_MSG_ERROR([Invalid target for configuring PhantomOS $target])
        ;;
esac

AC_ARG_VAR([RUST_TARGET_PATH])

if test "$RUST_TARGET_PATH" \= ""
then
    RUST_TARGET_PATH="$ac_abssrcdir/targets"
fi

LCRUST_PROG_RUSTC

LCRUST_RUSTC_VERSION

AC_PROG_CC

AC_PROG_INSTALL 

AC_ARG_VAR([ASFLAGS])

AC_ARG_VAR([MAKE])
AC_PATH_PROGS([MAKE],[gmake lc-make make])

if test x$rustc_is_lccc \= xyes
then
    libsrc=`$RUSTC $RUSTFLAGS -Z extra-prints --print rust-libsrc`
    AS_MKDIR_P([build-std-build/build])
    
    $(RUSTFLAGS) $(RUSTC) $(CC) $(libsrc)/configure --bindir=`pwd`/build-std-build/build --with-rustlibdir=`pwd`/build-std-build/ --enable-libraries=core,alloc 
    $(MAKE) -C build-std-build/ all 
    $(MAKE) -C build-std-build/ install
else
    AC_ARG_VAR([CARGO])
    AC_PATH_PROG([CARGO],[cargo])
    mkdir build-std-src
    echo > build-std-src/Cargo.toml << "EOF"
[package]
name = "dummy"
version = "0.1.0"
edition = "2021"
EOF
    mkdir build-std-src/build
    mkdir build-std-src/src
    echo '#![no_std]' > build-std-src/src/lib.rs
    echo "$CARGO build --target $rustc_host_target --target-dir build-std-src/build --manifest-path build-std-src/ --release -Z build-std=core,alloc" > config.log
    RUSTFLAGS="$RUSTFLAGS" RUSTC="$RUSTC" $CARGO build --target $rustc_host_target --target-dir build-std-src/build --manifest-path build-std-src/ --release -Z build-std=core,alloc > /dev/null 2>> config.log
    if  test $? -ne 0
    then
        AC_MSG_ERROR([Failed to build the standard library for $RUSTC])
    fi
    mkdir build-std-build

    for lib in core alloc compiler_builtins
    do
        cp build-std-src/build/x86_64-pc-phantom-kernel/deps/lib${lib}-*.rlib build-std-build/lib${lib}.rlib
    done
    rm -rf build-std-src
    RUSTFLAGS="--extern \"core=$PWD/build-std-build/libcore.rlib --extern \"noprelude:alloc=$PWD/build-std-build/liballoc.rlib\" --extern \"noprelude:compiler_builtins=$PWD/build-std-build/libcompiler_builtins.rlib\""
fi


AC_ARG_WITH([bootdir],[Sets the directory to install boot artifacts into],[
    case "$enableval" in
        [/\\]* )
            bootdir="$enableval"
            base_bootdir="$enableval"
            ;;
        * )
            base_bootdir="/$enableval"
            case "$prefix" in
            *[/\\]usr  )
                bootdir="`basename $prefix`/$enableval"
                ;;
            *[/\\]usr[/\\]local )
                _prefix_stripped="`basename $prefix`"
                bootdir="`basname $_prefix_stripped`/$enableval"
                ;;
            * )
                bootdir="$prefix/$enableval"
                ;;
            esac
        ;;
    esac
],[
    base_bootdir="/boot"
    case "$prefix" in
    *[/\\]usr  )
        bootdir="`basename $prefix`/boot"
        ;;
    *[/\\]usr[/\\]local )
        _prefix_stripped="`basename $prefix`"
        bootdir="`basname $_prefix_stripped`/boot"
        ;;
    * )
        bootdir="$prefix/boot"
        ;;
    esac
])

AC_ARG_WITH([boot-base-dirs],[Directories to search for the kernel and libraries in],[
    base_dirs="$withval"
],[
    base_dirs="/boot:/"
])

AC_ARG_WITH([search-paths],[comma separated list of directories within the base directory to search for the kernel in],[
    search_paths="$search_paths"
],[
    search_paths="/:/system"
])

AC_ARG_WITH([kernel-name],[the filename of the kernel to run],[
    kernel_name="$kernel_name"
],[
    kernel_name="phantom-kernel.so"
])

AC_SUBST([base_dirs search_paths kernel_name])

AC_CONFIG_FILES([Makefile])


AC_OUTPUT