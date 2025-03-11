{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    wgsl_analyzer = {
      url = "github:wgsl-analyzer/wgsl-analyzer";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { nixpkgs, rust-overlay, ... }:
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
              system = system;
              overlays = overlays;
            };
          in
          function pkgs
        );
    in
    {

      devShells = forAllSystems (pkgs: {
        default =
          with pkgs;
          mkShell {
            buildInputs = [
              (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
              (rust-bin.selectLatestNightlyWith (toolchain: toolchain.rust-analyzer))
              nixd
              nixfmt-rfc-style
            ];
          };
      });

      packages = forAllSystems (pkgs: {
        default = pkgs.callPackage ./nix/package.nix { };
      });
    };
}
