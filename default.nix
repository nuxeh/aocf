with import <nixpkgs> {};

stdenv.mkDerivation {
    name = "aocf";

    buildInputs = [
      rustc
      cargo
      pkg-config
      openssl
      sqlite
    ];
}
