#!/bin/sh

# Create a new offline container from the `ubuntu:22.04` image, return the id.
ctr=$(buildah from "ubuntu:22.04")

# Do some things or whatever
buildah config --author "Frank Waldheim" -- "$ctr"

# Run a script inside the container
buildah run "$ctr" -- /bin/sh <<EOF
  apt-get update
  apt-get install curl libgtk-4-dev build-essential -y
  echo "Ubuntu libs installed"
EOF

# Another one, same layer though
buildah run "$ctr" -- /bin/bash <<EOF
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain 1.69.0 -y
  echo "Rust installed"
EOF


# Copy sources
mnt=$(buildah mount "$ctr")
cp -Rv src "$mnt/" && cp -Rv Cargo.toml "$mnt/"

buildah run "$ctr" -- /bin/bash <<EOF
  source "/root/.cargo/env"
  cargo build
  echo "SaeueP build successfully - pick it up enjoy :)"
EOF

# Commit this container as "saeuep" - this will be used later during sub-sequent builds
buildah commit -- "$ctr" "saeuep"