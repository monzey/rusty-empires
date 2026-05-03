{ pkgs, lib, ... }:

let
  runtimeLibraries = with pkgs; [
    alsa-lib
    libxkbcommon
    udev
    vulkan-loader
    wayland
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
  ];
in
{
  languages.rust = {
    enable = true;
    channel = "stable";
    components = [
      "cargo"
      "clippy"
      "rust-src"
      "rustc"
      "rustfmt"
    ];
  };

  packages = with pkgs; [
    clang
    git
    pkg-config
    rust-analyzer
    vulkan-tools
  ] ++ runtimeLibraries;

  env.LD_LIBRARY_PATH = lib.makeLibraryPath runtimeLibraries;

  scripts.check.exec = "cargo check";
  scripts.run.exec = "cargo run";
  scripts.fmt.exec = "cargo fmt";
  scripts.lint.exec = "cargo clippy --all-targets --all-features -- -D warnings";

  enterShell = ''
    echo "Rusty Empires development shell"
    rustc --version
    cargo --version
  '';
}
