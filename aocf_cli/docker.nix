{ pkgs ? import <nixpkgs> {} }:

let
  aocf_cli = pkgs.callPackage ./aocf_cli.nix {};
in
pkgs.dockerTools.buildImage {
  name = "aocf";
  config = {
    Cmd = [ "${aocf_cli}/bin/aocf" ];
  };
}
