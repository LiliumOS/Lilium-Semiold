AC_INIT([install-sh])



AC_CONFIG_MACRO_DIRS([m4/])

AC_CANONICAL_BUILD
AC_CANONICAL_HOST

case "$host" in
    *-*-phantom-kernel )
        ;;
    * )
        AC_MSG_ERROR([Invalid target for configuring PhantomOS $host])
        ;;
esac

AC_ARG_VAR([RUST_TARGET_PATH])

export RUST_TARGET_PATH

if test "$RUST_TARGET_PATH" \= ""
then
    RUST_TARGET_PATH="`realpath $srcdir`/targets"
fi

AC_MSG_CHECKING([where to find the target file for $host])
AC_MSG_RESULT([$RUST_TARGET_PATH])

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
    
    RUSTFLAGS=$(RUSTFLAGS) RUSTC=$(RUSTC) CC=$(CC) $(libsrc)/configure --host=$(host) --bindir=`pwd`/build-std-build/build --with-rustlibdir=`pwd`/build-std-build/ --enable-libraries=core,alloc 
    $(MAKE) -C build-std-build/ all 
    $(MAKE) -C build-std-build/ install
    RUSTFLAGS="-L build-std-build/"
else
    if test -d build-std-build/ -a -f build-std-build/rustc-version
    then
        if test "`$RUSTC -V`" \= "`cat build-std-build/rustc-version`"
        then
            _skip_std_build=yes
        fi
    fi
    if test x$_skip_std_build != xyes
    then
    AC_ARG_VAR([CARGO])
    AC_PATH_PROG([CARGO],[cargo])
    CARGO_RUSTFLAGS="`sed -e 's/--target [[[:graph:]]]*//'<<<"$RUSTFLAGS"`"
    mkdir -p build-std-src
    cat > build-std-src/Cargo.toml << "EOF"
[[package]]
name = "dummy"
version = "0.1.0"
edition = "2018"
[[workspace]]
EOF
    mkdir -p build-std-src/build
    mkdir -p build-std-src/src
    echo '#![no_std]' > build-std-src/src/lib.rs
    echo "$CARGO build --target $rustc_host_target --target-dir build-std-src/build --manifest-path build-std-src/ --release -Z build-std=core,alloc" >> config.log
    RUST_TARGET_PATH="$RUST_TARGET_PATH"  RUSTFLAGS="$CARGO_RUSTFLAGS" RUSTC="$RUSTC" $CARGO build --target $rustc_host_target --target-dir build-std-src/build --manifest-path build-std-src/Cargo.toml --release -Z build-std=core,alloc > /dev/null 2>> config.log
    if  test $? -ne 0
    then
        AC_MSG_ERROR([Failed to build the standard library for $RUSTC])
    fi
    mkdir -p build-std-build 2>&1 > /dev/null

    for lib in core alloc compiler_builtins rustc_std_workspace_core
    do
        cp build-std-src/build/$rustc_host_target/release/deps/lib${lib}-*.rlib build-std-build/lib${lib}.rlib
    done
    rm -rf build-std-src
    echo -n "`$RUSTC -V`" > build-std-build/rustc-version
    fi
    RUSTFLAGS="$RUSTFLAGS --extern \"core=$PWD/build-std-build/libcore.rlib\" -L \"$PWD/build-std-build\""
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

AC_SUBST([base_dirs])
AC_SUBST([search_paths])
AC_SUBST([kernel_name])
AC_SUBST([RUST_TARGET_PATH])

AC_CONFIG_FILES([Makefile])


AC_OUTPUT