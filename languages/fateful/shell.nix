{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
	pkg-config
  ];

  buildInputs = with pkgs; [
    openssl.dev
    openssl
    systemd
	wayland
	libGL
	pkg-config
	xorg.libX11
	xorg.libXrandr
	xorg.libXi
	xorg.libXcursor
	libxkbcommon
  ];
}