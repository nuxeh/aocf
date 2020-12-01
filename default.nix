with import <nixpkgs> {};

stdenv.mkDerivation {
    name = "aocf";

    buildInputs = [
      rustc
      cargo
      pkgconfig
      openssl
      sqlite
    ];
}
