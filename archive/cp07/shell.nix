{ pkgs ? import <nixpkgs> { } }:

let
  libPath = with pkgs; lib.makeLibraryPath [
    libxkbcommon
    vulkan-loader
    wayland
  ];
in
pkgs.mkShell {
  packages = with pkgs; [
    trunk
  ];

  LD_LIBRARY_PATH = "${libPath}";
}
