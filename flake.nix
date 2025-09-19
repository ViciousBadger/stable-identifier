{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
      naersk-lib = pkgs.callPackage naersk {};
    in {
      defaultPackage = naersk-lib.buildPackage ./.;
      devShell = with pkgs; let
        deps = [
          cargo
          rustc
          rustfmt
          pre-commit
          rustPackages.clippy # Base rust setup
          cargo-watch # Cargo-watch command (live recompiling)
          cargo-edit # Allows upgrading all dependencies
          cargo-modules # Show module tree of a crate
          rust-analyzer # Language server protocol
          pkg-config

          clang
          mold # Faster linker

          # Bevy runtime deps
          udev
          alsa-lib
          vulkan-loader
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr # To use the x11 feature
          libxkbcommon
          wayland # To use the wayland feature
        ];
      in
        mkShell {
          buildInputs = deps;
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          LD_LIBRARY_PATH = lib.makeLibraryPath deps;
        };
    });
}
