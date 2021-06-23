#!/bin/bash
set -exo pipefail

echo "starting build for TARGET $TARGET"

export CRATE_NAME=kiss-ftpd

# cross doesn't actually support stdin/stdout pipes for some reason, skip it for now
DISABLE_TESTS=1

SUFFIX=""

echo "$TARGET" | grep -E '^x86_64-pc-windows-gnu$' >/dev/null && SUFFIX=".exe"

echo "$TARGET" | grep -E '^(i586-unknown-linux-musl|i586-unknown-linux-gnu)$' >/dev/null && echo "$TARGET not supported in raw-cpuid" && exit 0
echo "$TARGET" | grep -E '^(armv5te-unknown-linux-gnueabi|armv5te-unknown-linux-musleabi|arm-linux-androideabi)$' >/dev/null && echo "$TARGET not supported in atomic-shim" && exit 0
echo "$TARGET" | grep -E '^(s390x|powerpc|mips|riscv64gc|.*solaris$)' >/dev/null && echo "$TARGET not supported in ring" && exit 0
echo "$TARGET" | grep -E '-netbsd$' >/dev/null && echo "$TARGET fails at linking" && exit 0

cross rustc --bin kiss-ftpd --target $TARGET --release

# to check how they are built
file "target/$TARGET/release/kiss-ftpd$SUFFIX"

# if this commit has a tag, upload artifact to release
strip "target/$TARGET/release/kiss-ftpd$SUFFIX" || true # if strip fails, it's fine
mkdir -p release
mv "target/$TARGET/release/kiss-ftpd$SUFFIX" "release/kiss-ftpd-$TARGET$SUFFIX"

echo 'build success!'
exit 0
