{pkgs ? import <nixpkgs> {}}:
with pkgs; pkgs.mkShell rec {
  nativeBuildInputs = [
    rustup
    pkg-config
  ];
  buildInputs = with pkgs; [
    udev alsaLib vulkan-loader
    xlibsWrapper xorg.libXcursor xorg.libXrandr xorg.libXi
    libxkbcommon
  ];
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
