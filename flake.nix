{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      overlays = [ (import rust-overlay) ];
      forAllSystems =
        function:
        nixpkgs.lib.genAttrs systems (
          system:
          let
            pkgs = import nixpkgs {
              inherit system;
              inherit overlays;
            };
          in
          function pkgs
        );
    in
    {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          buildInputs = [
            (pkgs.rust-bin.selectLatestNightlyWith (
              toolchain:
              toolchain.default.override {
                extensions = [
                  "rustc-codegen-cranelift-preview"
                  "rust-src"
                  "rustfmt"
                ];
              }
            ))
          ]
          ++ builtins.attrValues {
            inherit (pkgs)
              rust-analyzer-unwrapped
              nixd
              nixfmt-rfc-style
              gcc
              clang
              ;
          };
        };
      });

      packages = forAllSystems (pkgs: {
        moxctl = pkgs.callPackage ./nix/package.nix {
          rustPlatform =
            let
              rust-bin = pkgs.rust-bin.stable.latest.default;
            in
            pkgs.makeRustPlatform {
              cargo = rust-bin;
              rustc = rust-bin;
            };
        };
        default = self.packages.${pkgs.system}.moxctl;
      });

      homeManagerModules = {
        default = import ./nix/home-manager.nix;
      };
    };
}
