{
  config,
  lib,
  pkgs,
  self,
  ...
}:
let
  cfg = config.programs.moxctl;
  inherit (lib) types;
in
{
  options.programs.moxctl = {
    enable = lib.mkEnableOption "moxctl";
    package = lib.mkOption {
      type = types.package;
      default = self.packages.${pkgs.hostPlatform.system}.moxctl;
    };
  };

  config = lib.mkIf cfg.enable { home.packages = [ cfg.package ]; };
}
