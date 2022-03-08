# See docs/cross_compilation.md for details.
let
  rustVersion = "1.57.0";
  defaultPkgs = import (builtins.getFlake "nixpkgs") {
    overlays = [ (builtins.getFlake "github:oxalica/rust-overlay").overlay ];
  };
in
{ pkgs ? defaultPkgs }:
with pkgs;
mkShell {
  buildInputs = [
    (rust-bin.stable."${rustVersion}".default.override {
      extensions = [ "rust-src" "rustfmt" ];
    })
    rust-analyzer
    rustup
    nodejs-14_x
  ];
}
