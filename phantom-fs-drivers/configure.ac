AC_INIT([Cargo.toml])


AC_CONFIG_MACRO_DIRS([../m4/])

AC_CANONICAL_BUILD
AC_CANONICAL_HOST

case "$host" in
    *-*-phantom-kernel )
        ;;
    * )
        AC_MSG_ERROR([Invalid target for configuring PhantomOS $host])
        ;;
esac


if test "$host_alias" \= ""
then
    host_noncanonical="$host"
else
    host_noncanonical="$host_alias"
fi

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

AC_PROG_MKDIR_P

AC_ARG_VAR([ASFLAGS])


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

AC_ARG_WITH([search-paths],[collon separated list of directories within the base directory to search for the kernel in],[
    search_paths="$search_paths"
],[
    search_paths="/:/system"
])

AC_ARG_WITH([kernel-lib-search],[the directories the loader searches for libraries, within the base directory],[
    kernel_lib_search="$withval"
],[
    kernel_lib_search="/system/lib:/system/$host_noncanonical/lib"
])

AC_ARG_WITH([kernel-libdir],[the directory to install kernel libraries and modules into],[
    kernel_libdir="$withval"
],[
    kernel_libdir='$bootdir/system/lib'
])

AC_ARG_WITH([kernel-name],[the filename of the kernel to run],[
    kernel_name="$kernel_name"
],[
    kernel_name="phantom-kernel.so"
])

AC_SUBST([kernel_libdir])
AC_SUBST([kernel_lib_search])
AC_SUBST([base_dirs])
AC_SUBST([search_paths])
AC_SUBST([kernel_name])
AC_SUBST([RUST_TARGET_PATH])

AC_CONFIG_FILES([Makefile])


AC_OUTPUT