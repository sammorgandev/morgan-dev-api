{ pkgs ? import <nixpkgs> {} }:
pkgs.stdenv.mkDerivation {
  name = "my-rust-project";
  buildInputs = with pkgs; [
    rustc
    cargo
    clang
    lld
  ];
  # Other configuration options...
}