#!/bin/sh

set -eu

VERSION="0.9"
GIT=$(which git)
SED=$(which gsed || which sed)

$GIT submodule update --init --recursive
OLD_VERSION="$VERSION.$($GIT -C ./ubpf rev-list --count HEAD)+$($GIT -C ./ubpf rev-parse --short=7 HEAD)"
$GIT submodule deinit -f ./ubpf
rm -rf ./.git/modules/ubpf
$GIT rm -f ./ubpf
$GIT submodule add https://github.com/iovisor/ubpf.git ./ubpf
$GIT submodule update --init --recursive
$GIT reset ./ubpf
NEW_VERSION="$VERSION.$($GIT -C ./ubpf rev-list --count HEAD)+$($GIT -C ./ubpf rev-parse --short=7 HEAD)"

if [ "$OLD_VERSION" != "$NEW_VERSION" ]; then

  $SED --in-place -e 's/^version = "[^\"]\+"$/version = "'"$NEW_VERSION"'"/' "$(ls Cargo.toml)"
  printf "\nOld version: %s\nNew version: %s\n\n" "$OLD_VERSION" "$NEW_VERSION"

else

  printf "\nVersion unchanged: %s\n\n" "$NEW_VERSION"

fi
