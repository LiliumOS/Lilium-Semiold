AC_INIT([Cargo.toml])

AC_CONFIG_MACRO_DIRS([../m4/])

AC_CANONICAL_HOST

AC_PROG_CC

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

