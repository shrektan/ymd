#!/bin/sh -e

cargo vendor

# c.f. https://reproducible-builds.org/docs/archives/
# note, we must use a GNU tar. mac tar's option is different.
# gtar is the GNU tar. installed by `brew install gnu-tar`
gtar \
  --sort=name \
  --mtime='1970-01-01 00:00:00Z' \
  --owner=0 \
  --group=0 \
  --numeric-owner \
  --xz \
  --create \
  --file=vendor.tar.xz \
  vendor

echo
echo
echo "#############################################"
echo "#                                           #"
echo "#  UPDATE src/cargo_vendor_config.toml !!!  #"
echo "#                                           #"
echo "#############################################"
