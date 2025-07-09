{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.programs.moxctl;
in
{
  options.programs.moxctl = {
    enable = lib.mkEnableOption "moxctl";
    package = lib.mkPackageOption pkgs "moxctl" { };
  };

  config = lib.mkIf cfg.enable { home.packages = [ cfg.package ]; };
}
