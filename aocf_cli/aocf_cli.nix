# this will no longer build, since it requires a non-path `aocf` dependency to build

{ stdenv
, lib
, rustPlatform
, pkg-config
, openssl
}:

rustPlatform.buildRustPackage rec {
  pname = "aocf_cli";
  version = "0.1.8";
  src = ./.;
  cargoSha256 = "1d2m4jyf4b5mh2carms77m78cgakw5h3hwhmzrhj974y87599hy1";

  nativeBuildInputs = [ pkg-config ];

  buildInputs = [
    openssl
  ];

  meta = with lib; {
    description = "Advent of Code Swiss army knife";
    homepage = "https://github.com/nuxeh/aocf";
    license = licenses.isc;
    maintainers = with maintainers; [ edcragg ];
  };
}
