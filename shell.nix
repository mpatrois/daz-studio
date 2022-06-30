with import <nixpkgs> {};
mkShell {
  packages = [
    pkg-config
    portaudio alsa-lib
    SDL2 SDL2_ttf
    cargo
  ];
}
