#!/bin/bash

set -e

if [ $# != 1 ]; then
	echo "Usage: . $0 <rust-target>"
	exit 1
fi

arch=""
sysroot=""
need_target_linker=""
ubuntu_cross_pkg_list=""
target_linker=""
target_cc=""

case "$1" in
	aarch64*)
		arch="arm64"
		sysroot="/usr/lib/aarch64-linux-gnu/"
		ubuntu_cross_pkg_list="gcc-aarch64-linux-gnu"
		# YES they are the same but otherwise it fails
		target_linker="aarch64-linux-gnu-gcc"
		target_cc="aarch64-linux-gnu-gcc"

	;;
	riscv64*)
		arch="riscv64"
		sysroot="/usr/lib/riscv64-linux-gnu/"
		ubuntu_cross_pkg_list="gcc-riscv64-linux-gnu"
		# YES they are the same but otherwise it fails
		target_linker="riscv64-linux-gnu-gcc"
		target_cc="riscv64-linux-gnu-gcc"

	;;
	x86_64*)
		target_linker="gcc"
		target_cc="gcc"
	;;
esac

echo MULTILIB_ARCH=$arch
echo PKG_CONFIG_SYSROOT_DIR=$sysroot
echo TARGET_CC=$target_cc
echo CARGO_TARGET_$(echo "$1" | tr 'a-z' 'A-Z' | tr '-' '_' )_LINKER=$target_linker
echo UBUNTU_CROSS_PKG_LIST=$ubuntu_cross_pkg_list
