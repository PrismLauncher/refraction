{
  lib,
  stdenv,
  rustPlatform,
  darwin,
  self,
  lto ? false,
}:
rustPlatform.buildRustPackage {
  pname = "refraction";
  version =
    (lib.importTOML ../Cargo.toml).package.version
    + "-${self.shortRev or self.dirtyShortRev or "unknown-dirty"}";

  __structuredAttrs = true;

  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [
      ../src
      ../build.rs
      ../Cargo.lock
      ../Cargo.toml
      ../tags
    ];
  };

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  buildInputs = lib.optionals stdenv.hostPlatform.isDarwin (with darwin.apple_sdk.frameworks; [
    CoreFoundation
    Security
    SystemConfiguration
  ]);

  env = {
    CARGO_BUILD_RUSTFLAGS = lib.concatStringsSep " " (
      lib.optionals lto ["-C" "lto=thin" "-C" "embed-bitcode=yes" "-Zdylib-lto"]
    );
  };

  meta = with lib; {
    mainProgram = "refraction";
    description = "Discord bot for Prism Launcher";
    homepage = "https://github.com/PrismLauncher/refraction";
    license = licenses.gpl3Plus;
    maintainers = with maintainers; [getchoo Scrumplex];
  };
}
