{
  lib,
  stdenv,
  go,
  rustPlatform,
  lto ? !optimizeSize,
  optimizeSize ? false,
}:

let
  fs = lib.fileset;
  toRustFlags = flags: toString (lib.mapAttrsToList (name: value: "-C ${name}=${value}") flags);
in
assert lib.assertMsg (lto -> !optimizeSize) "`lto` and `optimizeSize` are mutually exclusive";
rustPlatform.buildRustPackage rec {
  pname = "refraction";
  inherit (passthru.cargoToml.package) version;

  src = fs.toSource {
    root = ../.;
    fileset = fs.intersection (fs.gitTracked ../.) (
      fs.unions [
        ../src
        ../build.rs
        ../Cargo.lock
        ../Cargo.toml
        ../tags
      ]
    );
  };

  cargoLock.lockFile = ../Cargo.lock;

  # `panic=abort` breaks tests womp womp
  doCheck = stdenv.buildPlatform.canExecute stdenv.hostPlatform && !optimizeSize;

  env = {
    RUSTFLAGS = toRustFlags (
      lib.optionalAttrs lto {
        lto = "thin";
        embed-bitcode = "yes";
      }
      // lib.optionalAttrs optimizeSize {
        codegen-units = "1";
        opt-level = "s";
        panic = "abort";
        strip = "symbols";
      }
    );
  };

  passthru = {
    cargoToml = lib.importTOML ../Cargo.toml;
    # For container images
    dockerArchitecture = go.GOARCH;
  };

  meta = {
    description = "Discord bot for Prism Launcher";
    homepage = "https://github.com/PrismLauncher/refraction";
    license = lib.licenses.gpl3Plus;
    maintainers = with lib.maintainers; [
      getchoo
      Scrumplex
    ];
    mainProgram = "refraction";
  };
}
