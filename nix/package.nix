{ lib, rustPlatform }:
let
  cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
in
rustPlatform.buildRustPackage {
  pname = "moxctl";
  version = "${cargoToml.package.version}";
  cargoLock.lockFile = ../Cargo.lock;
  src = lib.fileset.toSource {
    root = ./..;
    fileset = lib.fileset.intersection (lib.fileset.fromSource (lib.sources.cleanSource ./..)) (
      lib.fileset.unions [
        ../src
        ../Cargo.toml
        ../Cargo.lock
      ]
    );
  };

  meta = {
    description = "Idle daemon with conditional timeouts and built-in audio inhibitor";
    mainProgram = "moxctl";
    homepage = "https://github.com/unixpariah/moxidle";
    license = lib.licenses.gpl3;
    maintainers = builtins.attrValues { inherit (lib.maintainers) unixpariah; };
    platforms = lib.platforms.unix;
  };
}
