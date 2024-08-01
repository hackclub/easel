{ lib, fetchFromGitHub, rustPlatform }:

let 
  pkgs = import <nixpkgs> {};
in
rustPlatform.buildRustPackage rec {
  pname = "fateful";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "commonkestrel";
    repo = pname;
    rev = "e8897cc";
    hash = "sha256-910xg3yq5Ne6XhJ71RVqZW2rAdPciYvIv+Zj5KG86Y0=";
  };

  buildInputs = with pkgs; [
    pkg-config
    openssl.dev
    openssl
    systemd
    wayland
    libGL
    xorg.libX11
    xorg.libXrandr
    xorg.libXi
    xorg.libXcursor
    libxkbcommon
  ];

  cargoLock = {
    lockFile = ./Cargo.lock;
    outputHashes = {
      "fateful_macros-0.1.0" = "sha256-heaUewDYEtEW/uhIWeinSGoXDwKSpR75YlKE1xGSMEk=";
    };
  };

  meta = {
    description = "A command line utility to create and test programs for the F8ful CPU";
    homepage = "https://github.com/commonkestrel/fateful";
    licence = lib.licences.mit;
    maintainers = ["commonkestrel"];
  };
}
