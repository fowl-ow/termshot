{
  description = "termshot";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    bevy_cli.url = "github:TheBevyFlock/bevy_cli";
  };

  outputs =
    inputs@{
      flake-parts,
      nixpkgs,
      rust-overlay,
      bevy_cli,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "aarch64-darwin"
        "x86_64-linux"
      ];

      perSystem =
        { system, ... }:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };

          rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (
            toolchain:
            toolchain.default.override {
              extensions = [
                "rust-src"
                "rust-analyzer"
              ];
            }
          );

          bevyCliPackage = bevy_cli.packages.${system}.default.overrideAttrs (old: {
            nativeBuildInputs = (old.nativeBuildInputs or [ ]) ++ [ pkgs.pkg-config ];
            buildInputs = (old.buildInputs or [ ]) ++ [ pkgs.zlib ];
          });
        in
        {
          devShells.default = pkgs.mkShell {
            packages = [
              rustToolchain
              bevyCliPackage
              pkgs.pkg-config
              pkgs.dioxus-cli
            ];

            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          };
        };
    };
}
