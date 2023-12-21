{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    nativeBuildInputs = with pkgs.buildPackages; [
      nodejs_21
      nodePackages.prettier
      nodePackages.typescript-language-server
    ];
  }
