{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-parts,
      rust-overlay,
      ...
    }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "armv5tel-linux"
        "armv6l-linux"
        "armv7a-linux"
        "armv7l-linux"
        "i686-linux"
        "powerpc64le-linux"
        "riscv64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      perSystem =
        { pkgs, system, ... }:
        {
          imports = [ "${nixpkgs}/nixos/modules/misc/nixpkgs.nix" ];
          nixpkgs = {
            hostPlatform = system;
            overlays = [
              rust-overlay.overlays.default
              (final: prev: {
                rust-nightly = final.rust-bin.selectLatestNightlyWith (
                  toolchain:
                  toolchain.default.override {
                    extensions = [
                      "clippy"
                      "rust-analyzer"
                      "rust-src"
                    ];
                    targets = [ "wasm32-unknown-unknown" ];
                  }
                );
              })
            ];
            config.allowUnfree = true;
          };

          devShells.default =
            with pkgs;
            let
              linker = "${clang}/bin/clang";
              rustFlags = [
                "-C"
                "link-arg=-fuse-ld=${mold}/bin/mold"
                "-Z"
                "share-generics=y"
                "-Z"
                "threads=8"
              ];
            in
            mkShell rec {
              name = "rust-nightly";

              # https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/11
              RUST_SRC_PATH = "${rust-nightly}/lib/rustlib/src/rust/library";

              CARGO_BUILD_TARGET = "x86_64-unknown-linux-gnu";
              CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = "${clang}/bin/clang";
              CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS = [
                "-C"
                "link-arg=-fuse-ld=${mold}/bin/mold"
                "-Z"
                "share-generics=y"
                "-Z"
                "threads=8"
              ];

              TRUNK_BUILD_OFFLINE = "true";
              TRUNK_SERVE_OFFLINE = "true";
              TRUNK_WATCH_OFFLINE = "true";

              buildInputs = [
                alsa-lib
                libxkbcommon
                udev
                vulkan-loader
                wayland
              ];

              nativeBuildInputs = [
                (
                  let
                    pname = "butler";
                    version = "15.21.0";
                  in
                  symlinkJoin {
                    name = "${pname}-${version}";
                    paths = [
                      (fetchzip {
                        url = "https://broth.itch.zone/${pname}/linux-amd64/${version}/${pname}.zip";
                        stripRoot = false;
                        hash = "sha256-jHni/5qf7xST6RRonP2EW8fJ6647jobzrnHe8VMx4IA=";
                      })
                    ];
                    buildInputs = [ steam-run ];
                    nativeBuildInputs = [ makeWrapper ];
                    postBuild = ''
                      makeWrapper ${steam-run}/bin/steam-run $out/bin/butler --add-flags $out/butler
                    '';
                  }
                )
                binaryen
                cargo-audit
                cargo-deny
                cargo-edit
                cargo-license
                cargo-nextest
                cargo-outdated
                nodePackages.prettier
                pkg-config
                rust-nightly
                sass
                tailwindcss
                taplo
                trunk
                wasm-bindgen-cli
              ];

              shellHook = ''
                export LD_LIBRARY_PATH="${lib.makeLibraryPath buildInputs}''${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
                export BEVY_ASSET_ROOT="$PWD"
              '';
            };
        };
    };
}
