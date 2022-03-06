function setup_colors {
    if test -t 1; then
        ncolors=$(tput colors)
        if test -n "$ncolors" -a $ncolors -ge 8; then
            bold="$(tput bold)"
            normal="$(tput sgr0)"
            red="$(tput setaf 1)"
            cyan="$(tput setaf 6)"

            export error="${bold}${red}error:${normal}"
            export status="${bold}${cyan}status:${normal}"
        else
            export error="error:"
            export status="status:"
        fi
    else
        export error="error:"
        export status="status:"
    fi
}

function error {
    echo "$error $1" >&2
    exit 1
}

function build_project {
    echo "$status building PhantomOS initializer, kernel, modules, and userspace"

    cargo build || error "cargo build failed"
    export INIT_PATH=target/x86_64/debug/libphantomos_init.so

    echo "$status PhantomOS successfully built"
}

function build_limine {
    echo $status building Limine bootloader

    git submodule update --init

    test -d build-limine || mkdir build-limine
    pushd build-limine > /dev/null

    test -x ../limine/configure || ../limine/autogen.sh || error "limine autogen failed"
    ../limine/configure BUILD_ELTORITO_EFI=yes || error "limine configure failed"
    make || error "limine build failed"

    popd > /dev/null

    echo $status Limine bootloader successfully built
}

function build_iso {
    echo $status building ISO image

    rm -rf build-iso
    mkdir -p build-iso/boot
    cp -v $INIT_PATH build-iso/phantomos.elf
    cp -v limine-iso.cfg build-iso/limine.cfg
    cp -v build-limine/bin/{limine-eltorito-efi.bin,limine-cd.bin,limine.sys} build-iso/boot/

    xorriso -as mkisofs -b boot/limine-cd.bin \
        -no-emul-boot -boot-load-size 4 -boot-info-table \
        --efi-boot boot/limine-eltorito-efi.bin \
        -efi-boot-part --efi-boot-image --protective-msdos-label \
        build-iso -o phantomos.iso || error "ISO build failed"

    build-limine/bin/limine-install phantomos.iso || error "limine install failed"

    echo $status ISO image successfully built
}

pushd $(dirname $0) > /dev/null

setup_colors

build_project
build_limine
build_iso

popd > /dev/null
