# Use latest stable Rust
#
# To use, clone the Mozilla overlay, and provide the path at nix-shell
# invocation, e.g.:
#
#     git clone https://github.com/mozilla/nixpkgs-mozilla.git
#     nix-shell stable.nix -I rustoverlay=/path/to/overlay

with import <nixpkgs> {};
with import <rustoverlay/rust-overlay.nix> pkgs pkgs;

stdenv.mkDerivation {
  name = "aocf";

  buildInputs = [
    latest.rustChannels.stable.rust
    pkgconfig
    openssl
    sqlite
  ];
}
