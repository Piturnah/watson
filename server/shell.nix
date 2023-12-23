{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    nativeBuildInputs = with pkgs.buildPackages; [
      postgresql
      diesel-cli
      openssl
      pkg-config
    ];
  }
