#!/usr/bin/env bash
set -e

MOLD_VERSION=${1:-undefined}

if [[ "${MOLD_VERSION}" == undefined ]]; then
    echo "No Mold version specified!"
    exit 1
fi

architecture=$(dpkg --print-architecture)
case "${architecture}" in
    arm64)
        ARCH=aarch64 ;;
    amd64)
        ARCH=x86_64 ;;
    *)
        echo "Unsupported architecture ${architecture}."
        exit 1
        ;;
esac

# Remove any character that's not a 0-9 or a '.'
MOLD_VERSION_STRING="${MOLD_VERSION//[^0-9\.]}"
MOLD_DOWNLOAD_LINK="https://github.com/rui314/mold/releases/download/v${MOLD_VERSION_STRING}/mold-${MOLD_VERSION_STRING}-${ARCH}-linux.tar.gz"

# Install Mold
mkdir -p /tmp/mold
wget -qO /tmp/mold/mold.tar.gz "${MOLD_DOWNLOAD_LINK}"
tar -xvzf /tmp/mold/mold.tar.gz -C /tmp/mold
mv /tmp/mold/mold-${MOLD_VERSION_STRING}-${ARCH}-linux/ /opt/mold
ln -s /opt/mold/bin/mold /usr/local/bin/mold
ln -s /opt/mold/bin/ld.mold /usr/local/bin/ld.mold

# Cleanup
rm -r /tmp/mold