# Copyright 2021 Connor Horman

# Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

# The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

AC_DEFUN([_LCRUST_FIND_RUST_TARGET],[
    _RUSTC="$1"
    _RUSTFLAGS="$2"
    _target="$3"
    _target_alias="$4"

    case x$_RUSTC in
        x${host_alias}-* | x*[\\/]${host_alias}-* )
            rustc_target=${host_alias}
            $6
            ;;
        x${host}-* | x*[\\/]${host}-* )
            rust_target=${host}
            $6
            ;;
        x* )
            _SAVE_RUSTFLAGS="$_RUSTFLAGS"
            if test x$_target_alias != x
            then
                echo Trying target $_target_alias >> config.log
                echo "$_RUSTC $_RUSTFLAGS --target $_target_alias --print sysroot" >> config.log
                $_RUSTC $_RUSTFLAGS --target $_target_alias 2>> config.log > /dev/null
                if test $? -eq 0
                then
                    rust_target=$_target_alias
                    $5
                fi
            fi

            if test x$rust_target \= x
            then
                echo Trying target $_target >> config.log
                echo "$_RUSTC $_RUSTFLAGS --target $_target --print sysroot" >> config.log
                $_RUSTC $_RUSTFLAGS --target $_target 2>> config.log > /dev/null
                if test $? -eq 0
                then
                    rust_target=$_target
                    $5
                fi
            fi

            case x$_target in
                x*-*-mingw32 | x*-*-mingw32-* )
                    IFS="-" read _target_arch _rest <<< "$_target"
                    echo Trying target $_target_arch-pc-windows-gnu >> config.log
                    echo "$_RUSTC $_RUSTFLAGS --target $_target_arch-pc-windows-gnu --print sysroot" >> config.log
                    $_RUSTC $_RUSTFLAGS --target $_target_arch-pc-windows-gnu --print sysroot 2>> config.log > /dev/null
                    if test $? -eq 0
                    then
                        rust_target=$_target_arch-pc-windows-gnu
                        $5
                    fi
                    ;;
                xx86_64-*-* | xi?86-*-* )
                    IFS="-" read _target_arch _target_vendor _target_sys <<< "$_target"
                    echo Trying target $_target_arch-pc-$_target_sys >> config.log
                    echo "$_RUSTC $_RUSTFLAGS --target $_target_arch-pc-$_target_sys --print sysroot" >> config.log
                    $_RUSTC $_RUSTFLAGS --target $_target_arch-pc-$_target_sys --print sysroot 2>> config.log > /dev/null
                    if test $? -eq 0
                    then
                        rust_target=$_target_arch-pc-$_target_sys
                        $5
                    fi
                    ;;
            esac
            if test x$rust_target \= x
            then
                IFS="-" read _target_arch _target_vendor _target_sys <<< "$_target"
                echo Trying target $_target_arch-unknown-$_target_sys >> config.log
                echo "$_RUSTC $_RUSTFLAGS --target $_target_arch-unknown-$_target_sys --print sysroot" >> config.log
                $_RUSTC $_RUSTFLAGS --target $_target_arch-unknown-$_target_sys --print sysroot 2>> config.log >> /dev/null
                if test $? -eq 0
                then
                    rust_target=$_target_arch-unknown-$_target_sys
                    $5
                else
                    echo Trying target $_target_arch-$_target_sys >> config.log
                    "$_RUSTC $_RUSTFLAGS --target $_target_arch-$_target_sys --print sysroot" >> config.log
                    $_RUSTC $_RUSTFLAGS --target $_target_arch-$_target_sys --print sysroot 2>> config.log >> /dev/null
                    if test $? -eq 0
                    then
                        rust_target=$_target_arch-$_target_sys
                        $5
                    fi
                fi
            fi
            ;;
    esac

])

AC_DEFUN([_LCRUST_FILENAMES],[
    _RUSTC=$1
    _RUSTFLAGS=$2

    touch comptest.rs
    echo "$_RUSTC $_RUSTFLAGS --crate-type bin,rlib,dylib,staticlib,cdylib --print file-names" >> config.log
    _binname=$($RUSTC $RUSTFLAGS --crate-type bin --crate-name comptest --print file-names comptest.rs 2>> config.log)
    _rlibname=$($RUSTC $RUSTFLAGS --crate-type rlib --crate-name comptest --print file-names comptest.rs 2>> config.log)
    _dylibname=$($RUSTC $RUSTFLAGS --crate-type dylib --crate-name comptest --print file-names comptest.rs 2>> config.log)
    _staticlibname=$($RUSTC $RUSTFLAGS --crate-type staticlib --crate-name comptest --print file-names comptest.rs 2>> config.log)
    _cdylibname=$($RUSTC $RUSTFLAGS --crate-type cdylib --crate-name comptest --print file-names comptest.rs 2>> config.log)

    rust_bin_prefix=$(echo $_binname | sed 's/\(.*\)comptest\(.*\)/\1/')
    rust_bin_suffix=$(echo $_binname | sed 's/\(.*\)comptest\(.*\)/\2/')
    rust_rlib_prefix=$(echo $_rlibname | sed 's/\(.*\)comptest\(.*\)/\1/')
    rust_rlib_suffix=$(echo $_rlibname | sed 's/\(.*\)comptest\(.*\)/\2/')
    rust_dylib_prefix=$(echo $_dylibname | sed 's/\(.*\)comptest\(.*\)/\1/')
    rust_dylib_suffix=$(echo $_dylibname | sed 's/\(.*\)comptest\(.*\)/\2/')
    rust_staticlib_prefix=$(echo $_staticlibname | sed 's/\(.*\)comptest\(.*\)/\1/')
    rust_staticlib_suffix=$(echo $_staticlibname | sed 's/\(.*\)comptest\(.*\)/\2/')
    rust_cdylib_prefix=$(echo $_cdylibname | sed 's/\(.*\)comptest\(.*\)/\1/')
    rust_cdylib_suffix=$(echo $_cdylibname | sed 's/\(.*\)comptest\(.*\)/\2/')
    rm -f comptest.rs
])

AC_DEFUN([LCRUST_PROG_RUSTC],[
    AC_REQUIRE([AC_PROG_CC])
    AC_REQUIRE([AC_CANONICAL_HOST])
    AC_ARG_VAR(RUSTC,[Rust compiler to use])
    AC_ARG_VAR(RUSTFLAGS,[Flags to pass to the rust compiler])
    

    if test "$RUSTFLAGS" \= "" 
    then
        RUSTFLAGS="-C opt-level=2 -g"
    fi

    if test x$host_alias != x 
    then
        AC_PATH_PROGS(RUSTC,[rustc $host_alias-lcrustc  lcrustc $host_alias-gccrs])
    else 
        AC_PATH_PROGS(RUSTC,[rustc $host-lcrustc  lcrustc $host-gccrs gccrs])
    fi

    

    if test "$RUSTC" \= ""
    then
        AC_MSG_ERROR([Failed to find a rust compiler. Install rustc in PATH, or set RUSTC to a suitable compiler])
    fi

    AC_MSG_CHECKING([What style of arguments $RUSTC Accepts])

    echo "$RUSTC -C opt-level=0" >> config.log
    $RUSTC -C opt-level=0 2>> config.log >/dev/null

    if test $? -eq 0
    then
        AC_MSG_RESULT([rustc])
    else
        AC_MSG_RESULT([gcc])
        if test \! -f ${abs_srcdir}/rustc-wrap
        then
            AC_MSG_ERROR([rustc-wrap is not found in the source directory])
        fi
        RUSTC="${abs_srcdir}/rustc-wrap $RUSTC"
    fi

    AC_MSG_CHECKING([how to compile for $host with $RUSTC])
    _LCRUST_FIND_RUST_TARGET([$RUSTC],[$RUSTFLAGS],[$host],[$host_alias],[
        rustc_host_target=$rust_target
        RUSTFLAGS="$RUSTFLAGS --target $rustc_host_target"
        AC_MSG_RESULT([--target $rustc_host_target])
    ],[
        rustc_host_target=$rust_target
        AC_MSG_RESULT([none needed])
    ])

    
    if test x$rustc_host_target \= x
    then
        AC_MSG_RESULT([not found])
        AC_MSG_ERROR([Cannot cross compile to $host with $RUSTC])
    fi

    _LCRUST_FILENAMES([$RUSTC],[$RUSTFLAGS])

    echo 'fn main(){}' > comptest.rs
    AC_MSG_CHECKING([whether $RUSTC works])
    echo "$RUSTC $RUSTFLAGS --crate-type bin --crate-name comptest --emit link=${rust_bin_prefix}comptest${rust_bin_suffix} comptest.rs" >> config.log

    $RUSTC $RUSTFLAGS --crate-type bin --crate-name comptest --emit link=${rust_bin_prefix}comptest${rust_bin_suffix} comptest.rs 2>>config.log >/dev/null

    if test $? -ne 0
    then
        echo "#![no_std]" > comptest.rs
        echo "$RUSTC $RUSTFLAGS --crate-type rlib --crate-name comptest --emit link=${rust_rlib_prefix}comptest${rust_rlib_suffix} comptest.rs" >> config.log
        $RUSTC $RUSTFLAGS --crate-type rlib --crate-name comptest --emit link=${rust_rlib_prefix}comptest${rust_rlib_suffix} comptest.rs 2>>config.log >/dev/null

        if $? -ne 0
        then
            AC_MSG_RESULT([no])
            AC_MSG_ERROR([Cannot compile simple test program with $RUSTC])
        else
            AC_MSG_RESULT([no_std only])
            rustc_has_std=no
        fi
    else
        AC_MSG_RESULT([yes])
        rustc_has_std=yes
    fi

    if x$cross_compiling \= xno
    then
        ./${rust_bin_prefix}comptest${rust_bin_suffix}
        if test $? -ne 0
        then
            AC_MSG_ERROR([Cannot run binaries produced by $RUSTC])
        fi
    fi
    
    AC_SUBST(rust_bin_prefix)
    AC_SUBST(rust_bin_suffix)
    AC_SUBST(rust_rlib_prefix)
    AC_SUBST(rust_rlib_suffix)
    AC_SUBST(rust_staticlib_prefix)
    AC_SUBST(rust_staticlib_suffix)
    AC_SUBST(rust_dylib_prefix)
    AC_SUBST(rust_dylib_suffix)
    AC_SUBST(rust_cdylib_prefix)
    AC_SUBST(rust_cdylib_suffix)
    AC_SUBST(rustc_has_std)
    AC_SUBST(RUSTC)
    AC_SUBST(RUSTFLAGS)
])

AC_DEFUN([LCRUST_RUSTC_VERSION],[
    AC_REQUIRE([LCRUST_PROG_RUSTC])

    version_output="`${RUSTC} --version`"

    AC_MSG_CHECKING(the rust version supported by ${RUSTC})
    
    read rustc_name rust_version <<< ${version_output}

    AC_MSG_RESULT(${rust_version})

    case $rust_version in
        *.*.*-beta.* )
            rust_channel=beta
            IFS="." read rust_major rust_minor _lcrust_rest <<< ${rust_version}
            IFS="-" read rust_patch <<< ${_lcrust_rest}
            ;;
        *.*.*-* )
            IFS="." read rust_major rust_minor _lcrust_rest <<< ${rust_version}
            IFS="-" read rust_patch rust_channel <<< ${_lcrust_rest}
            ;;
        *.*.* )
            rust_channel=stable
            IFS="." read rust_major rust_minor rust_patch <<< ${rust_version}
            ;;
    esac
    AC_MSG_CHECKING(whether $RUSTC is lccc)
    case $rustc_name in
        lcrust* | lccc* ) dnl lccc doesn't distinguish between stable and unstable compiler, 
            rustc_is_lccc=yes
            ;;
        * )
            rustc_is_lccc=no
            ;;
    esac
    AC_MSG_RESULT([$rustc_is_lccc])
    
    AC_SUBST(rustc_name)
    AC_SUBST(rust_version)
    AC_SUBST(rust_channel)
    AC_SUBST(rust_major)
    AC_SUBST(rust_minor)
    AC_SUBST(rust_patch)
])

AC_DEFUN([LCRUST_PROG_RUSTC_FOR_BUILD],[
    AC_REQUIRE([AX_PROG_CC_FOR_BUILD])
    AC_REQUIRE([AC_CANONICAL_BUILD])
    AC_ARG_VAR(RUSTC_FOR_BUILD,[Rust compiler to use on the build system])
    AC_ARG_VAR(RUSTFLAGS_FOR_BUILD,[Flags to pass to the rust compiler for the build system])

    AC_MSG_NOTICE([checking for the compiler to use for $build...])

    AC_PATH_PROGS(RUSTC_FOR_BUILD,[rustc lcrustc $build-gccrs gccrs])

    if test "$RUSTC_FOR_BUILD" \= ""
    then
        AC_MSG_NOTICE([checking for the compiler to use for $build... not found])
        AC_MSG_ERROR([Failed to find a rust compiler. Install rustc in PATH, or set RUSTC_FOR_BUILD to a suitable compiler])
    fi

    AC_MSG_NOTICE([checking for the compiler to use for $build... $RUSTC_FOR_BUILD])

   AC_MSG_CHECKING([how to compile for $build with $RUSTC_FOR_BUILD])
   _LCRUST_FIND_RUST_TARGET([$RUSTC_FOR_BUILD],[$RUSTFLAGS_FOR_BUILD],[$host],[$host_alias],[
        rustc_build_target=$rust_target
        RUSTFLAGS_FOR_BUILD="$RUSTFLAGS --target $rustc_build_target"
        AC_MSG_RESULT([--target $rustc_build_target])
    ],[
        rustc_build_target=$rust_target
        AC_MSG_RESULT([none needed])
    ])

    if test x$rustc_host_target \= x
    then
        AC_MSG_RESULT([not found])
        AC_MSG_ERROR([Cannot cross compile to $build with $RUSTC_FOR_BUILD])
    fi

    pushdef([rust_bin_prefix],rust_build_bin_prefix)
    pushdef([rust_bin_suffix],rust_build_bin_suffix)
    pushdef([rust_rlib_prefix],rust_build_rlib_prefix)
    pushdef([rust_rlib_suffix],rust_build_rlib_suffix)
    pushdef([rust_dylib_prefix],rust_build_dylib_prefix)
    pushdef([rust_dylib_suffix],rust_build_dylib_suffix)
    pushdef([rust_staticlib_prefix],rust_build_staticlib_prefix)
    pushdef([rust_staticlib_suffix],rust_build_staticlib_suffix)
    pushdef([rust_cdylib_prefix],rust_build_cdylib_prefix)
    pushdef([rust_cdylib_suffix],rust_build_cdylib_suffix)
    _LCRUST_FILENAMES([$RUSTC_FOR_BUILD],[$RUSTFLAGS_FOR_BUILD])
    AC_SUBST(rust_bin_prefix)
    AC_SUBST(rust_bin_suffix)
    AC_SUBST(rust_rlib_prefix)
    AC_SUBST(rust_rlib_suffix)
    AC_SUBST(rust_staticlib_prefix)
    AC_SUBST(rust_staticlib_suffix)
    AC_SUBST(rust_dylib_prefix)
    AC_SUBST(rust_dylib_suffix)
    AC_SUBST(rust_cdylib_prefix)
    AC_SUBST(rust_cdylib_suffix)
    popdef([rust_bin_prefix])
    popdef([rust_bin_suffix])
    popdef([rust_rlib_prefix])
    popdef([rust_rlib_suffix])
    popdef([rust_dylib_prefix])
    popdef([rust_dylib_suffix])
    popdef([rust_staticlib_prefix])
    popdef([rust_staticlib_suffix])
    popdef([rust_cdylib_prefix])
    popdef([rust_cdylib_suffix])
    _proc_macroname=$($RUSTC $RUSTFLAGS --crate-type proc-macro --print file-names 2>> config.log)

    rust_build_proc_macro_prefix=$(echo $_proc_macroname | sed 's/\(.*\)comptest\(.*\)/\1/')
    rust_build_proc_macro_suffix=$(echo $_proc_macroname | sed 's/\(.*\)comptest\(.*\)/\2/')


    AC_MSG_CHECKING([whether $RUSTC_FOR_BUILD works])
    echo 'fn main(){}' > test.rs 
    $RUSTC_FOR_BUILD $RUSTFLAGS_FOR_BUILD --crate-type bin --crate-name test test.rs 2>> config.log > /dev/null
    if test $? -ne 0
    then
        AC_MSG_RESULT([no])
        AC_MSG_ERROR([Cannot compile a simple program with $RUSTC_FOR_BUILD])
    fi
    
    ./test${EXEEXT_FOR_BUILD}
    if test $? -ne 0
    then
        AC_MSG_RESULT([no])
        AC_MSG_ERROR([Cannot run executables compiled by $RUSTC_FOR_BUILD])
    fi

    rm -rf test.rs test${EXEEXT_FOR_BUILD}

    AC_MSG_RESULT([yes])

    AC_SUBST(RUSTC_FOR_BUILD)
    AC_SUBST(RUSTFLAGS_FOR_BUILD)
])

AC_DEFUN([LCRUST_RUSTC_VERSION_FOR_BUILD],[
    AC_REQUIRE([LCRUST_PROG_RUSTC_FOR_BUILD])

    version_output="`${RUSTC_FOR_BUILD} --version`"

    AC_MSG_CHECKING(the rust version supported by ${RUSTC_FOR_BUILD})
    
    read build_rustc_name build_rust_version <<< ${version_output}

    AC_MSG_RESULT(${build_rust_version})

    case $build_rust_version in
        *.*.*-beta.* )
            rust_channel=beta
            IFS="." read build_rust_major build_rust_minor _lcrust_rest <<< ${build_rust_version}
            IFS="-" read build_rust_patch <<< ${_lcrust_rest}
            ;;
        *.*.*-* )
            IFS="." read build_rust_major build_rust_minor _lcrust_rest <<< ${build_rust_version}
            IFS="-" read build_rust_patch build_rust_channel <<< ${_lcrust_rest}
            ;;
        *.*.* )
            rust_channel=stable
            IFS="." read build_rust_major build_rust_minor build_rust_patch <<< ${build_rust_version}
            ;;
    esac
    AC_MSG_CHECKING(whether $RUSTC_FOR_BUILD is lccc)
    case $build_rustc_name in
        lcrust* | lccc* ) dnl lccc doesn't distinguish between stable and unstable compiler, 
            build_rustc_is_lccc=yes
            ;;
        * )
            build_rustc_is_lccc=no
            ;;
    esac
    AC_MSG_RESULT([$build_rustc_is_lccc])
    
    AC_SUBST(build_rustc_name)
    AC_SUBST(build_rust_version)
    AC_SUBST(build_rust_channel)
    AC_SUBST(build_rust_major)
    AC_SUBST(build_rust_minor)
    AC_SUBST(build_rust_patch)
])


AC_DEFUN([LCRUST_TRY_COMPILE],[
    echo ["$1"] >> test.rs
    ${RUSTC} ${RUSTFLAGS} --crate-type rlib --crate-name test --emit link=libtest.rlib test.rs

    if test $? -eq 0 
    then
        rm -f test.rs libtest.rlib
        $2
    else
        rm -f test.rs libtest.rlib
        $3
    fi
])

AC_DEFUN([LCRUST_TRY_COMPILE_FOR_BUILD],[
    echo ["$1"] >> test.rs
    ${RUSTC_FOR_BUILD} ${RUSTFLAGS_FOR_BUILD} --crate-type rlib --crate-name test --emit link=libtest.rlib test.rs

    if test $? -eq 0 
    then
        rm -f test.rs libtest.rlib
        try_compile_result=yes
        $2
    else
        rm -f test.rs libtest.rlib
        try_compile_result=no
        $3
    fi
])

AC_DEFUN([LCRUST_PROG_CARGO],[
    AC_REQUIRE([LCRUST_PROG_RUSTC])
    AC_ARG_VAR([CARGO])
    AC_ARG_VAR([CARGOFLAGS])

    AC_PATH_PROG([CARGO],[cargo])


    CARGOFLAGS="$CARGOFLAGS --target $rustc_host_target"

    AC_MSG_CHECKING([whether $CARGO works])
    mkdir -m700 tmp
    cat > tmp/Cargo.toml << "EOF"
[package]
name = "cargotest"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

EOF
    mkdir tmp/src
    echo '#![no_std]' > tmp/src/lib.rs
    CARGO_RUSTFLAGS="`sed -e 's/--target [[[:graph:]]]*//'<<<"$RUSTFLAGS"`"
    AC_MSG_CHECKING([whether $CARGO works])
    echo "RUSTC=\"$RUSTC\" RUSTFLAGS=\"$CARGO_RUSTFLAGS\" $CARGO build $CARGOFLAGS --lib --manifest-path tmp/Cargo.toml --target-dir tmp/target/" >> config.log
    RUSTC="$RUSTC" RUSTFLAGS="$CARGO_RUSTFLAGS" $CARGO build $CARGOFLAGS --lib --manifest-path tmp/Cargo.toml --target-dir tmp/target/ 2>> config.log > /dev/null
    if test $? -ne 0
    then
        echo "RUSTC=\"$RUSTC\" RUSTFLAGS=\"$CARGO_RUSTFLAGS\" $CARGO gccrs $CARGOFLAGS --lib --manifest-path tmp/Cargo.toml --target-dir tmp/target/" >> config.log
        RUSTC="$RUSTC" RUSTFLAGS="$CARGO_RUSTFLAGS" $CARGO gccrs $CARGOFLAGS --lib --manifest-path tmp/Cargo.toml --target-dir tmp/target/ 2>> config.log > /dev/null
        if test $? -ne 0
        then
            AC_MSG_RESULT([no])
            rm -rf tmp/
            AC_MSG_ERROR([Cannot build a simple workspace with $CARGO])
        fi
        cargo_build_command=gccrs
    else
        cargo_build_command=build
    fi
    rm -rf tmp/
    AC_MSG_RESULT([yes])

    AC_SUBST([CARGO])
    AC_SUBST([CARGOFLAGS])
    AC_SUBST([CARGO_RUSTFLAGS])
    AC_SUBST([cargo_build_command])
])


AC_DEFUN([LCRUST_PROG_RUSTDOC],[
    AC_REQUIRE([LCRUST_PROG_RUSTC])
    AC_ARG_VAR([RUSTDOC])
    AC_ARG_VAR([RUSTDOCFLAGS])

    AC_PATH_PROG([RUSTDOC],[rustdoc])

    RUSTDOCFLAGS="$RUSTDOCFLAGS --target $rustc_host_target"

    AC_MSG_CHECKING([whether $RUSTDOC works])
    
    cat > comptest.rs << EOF
#![no_std]
#![doc = r"#
Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
Vivamus quis porttitor tortor, gravida pharetra mi. 
Cras eu est nec massa faucibus efficitur. 
Cras congue ultrices efficitur. 
Cras non auctor augue. 
Mauris faucibus purus ac dui dictum fermentum. 
Suspendisse dapibus elementum justo non consequat. 
Ut sit amet massa vel justo auctor euismod non rutrum justo. 
Fusce sed porttitor lectus. Sed semper enim eu nunc cursus elementum.
#"]
EOF

    echo "$RUSTDOC $RUSTDOCFLAGS --crate-type rlib --crate-name comptest --output tmp/ comptest.rs" >> config.log
    $RUSTDOC $RUSTDOCFLAGS --crate-type rlib --crate-name comptest --output tmp/ comptest.rs 2> config.log > /dev/null

    if test $? -ne 0
    then 
        rm -rf tmp/
        AC_MSG_RESULT([no])
        AC_MSG_ERROR([$RUSTDOC cannot build documentation for a simple program])
    fi

    if test ! -f tmp/comptest/index.html
    then
        rm -rf tmp/
        AC_MSG_RESULT([no])
        AC_MSG_ERROR([$RUSTDOC did not produce output in the expected format])
    fi

    if test "`grep 'Lorem ipsum dolor sit amet' tmp/comptest/index.html`" \= ""
    then
        rm -rf tmp/
        AC_MSG_RESULT([no])
        AC_MSG_ERROR([$RUSTDOC did not produce the expected output])
    fi
    rm -rf tmp/
    AC_MSG_RESULT([yes])
])

# Separate macro because `--test-builder` is unstable
AC_DEFUN([LCRUST_RUSTDOC_USE_RUSTC],[
    AC_REQUIRE([LCRUST_PROG_RUSTDOC])
    AC_REQUIRE([LCRUST_PROG_RUSTC])

    AC_MSG_CHECKING([how to pass --test-builder to $RUSTDOC])
    echo "$RUSTDOC --test-builder \"$RUSTC\" $RUSTDOCFLAGS --crate-type rlib --crate-name comptest --output tmp/ comptest.rs" >> config.log
    $RUSTDOC --test-builder "$RUSTC" $RUSTDOCFLAGS --crate-type rlib --crate-name comptest --output tmp/ comptest.rs  2> config.log > /dev/null

    if test $? -eq 0
    then
        rustdoc_use_rustc=yes
        RUSTDOCFLAGS="--test-builder \"$RUSTC\" $RUSTDOCFLAGS"
        AC_MSG_RESULT([--test-builder \"$RUSTC\"])
    else 
        echo "$RUSTDOC -Z unstable-options --test-builder \"$RUSTC\" $RUSTDOCFLAGS --crate-type rlib --crate-name comptest --output tmp/ comptest.rs" >> config.log
        $RUSTDOC -Z unstable-options --test-builder "$RUSTC" $RUSTDOCFLAGS --crate-type rlib --crate-name comptest --output tmp/ comptest.rs 2> config.log > /dev/null
        if test $? -eq 0
            rustdoc_use_rustc=unstable
            RUSTDOCFLAGS="-Z unstable-options --test-builder \"$RUSTC\" $RUSTDOCFLAGS"
            AC_MSG_RESULT([-Z unstable-options --test-builder \"$RUSTC\"])
        else
            rustdoc_use_rustc=no
            AC_MSG_RESULT([not found])
        fi
    fi
])
