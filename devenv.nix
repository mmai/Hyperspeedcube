{ pkgs, ... }:

{
  # https://devenv.sh/packages/
  packages = [
    # pour erreurs pendant compilation
    pkgs.python311
    pkgs.cmake
    pkgs.fontconfig
    pkgs.harfbuzz
    pkgs.freetype
    pkgs.expat
    pkgs.glib
    pkgs.atk 
    pkgs.gtk3 
    pkgs.pango
    pkgs.cairo
    pkgs.gdk-pixbuf 
    pkgs.zlib
    pkgs.xorg.libxcb

    # pour erreur à l'exécution, selon https://github.com/emilk/egui/discussions/1587
    pkgs.libxkbcommon
    pkgs.libGL

    # WINIT_UNIX_BACKEND=wayland
    pkgs.wayland

    # WINIT_UNIX_BACKEND=x11
    pkgs.xorg.libXcursor
    pkgs.xorg.libXrandr
    pkgs.xorg.libXi
    pkgs.xorg.libX11

    pkgs.vulkan-headers pkgs.vulkan-loader
  ];

  enterShell = ''
  cargo run --release
  '';

  # https://devenv.sh/languages/
  languages.rust.enable = true;

}
