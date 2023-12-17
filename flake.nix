{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs, rust-overlay, }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk { };

        buildInputs = with pkgs; [
          vulkan-loader
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          alsa-lib
          udev
          pkg-config
          openssl
        ];

        nativeBuildInputs = with pkgs; [
          rustc
          clippy
          cargo
          mold
        ];

        all_deps = with pkgs; [
          cargo-flamegraph
          cargo-expand
          nixpkgs-fmt
        ] ++ buildInputs ++ nativeBuildInputs;

      in
      rec {
        # For `nix build` & `nix run`:
        defaultPackage = packages.beyv_template;
        packages = rec {
          beyv_template = naersk'.buildPackage
            {
              src = ./.;
              nativeBuildInputs = nativeBuildInputs;
              buildInputs = buildInputs;

              cargoBuildOptions = x: x ++ [ "--no-default-features" ];
            };
        };

        # For `nix develop`:
        devShell = pkgs.mkShell
          {
            nativeBuildInputs = all_deps;
            shellHook = ''
              export CARGO_MANIFEST_DIR=$(pwd)
              export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath all_deps}"
            '';
          };
      }
    );
}
