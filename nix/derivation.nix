{
  lib,
  stdenv,
  rustPlatform,
  darwin,
  self,
  lto ? true,
  optimizeSize ? false,
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

  env = let
    toRustFlags = lib.mapAttrs' (
      name:
        lib.nameValuePair
        "CARGO_PROFILE_RELEASE_${lib.toUpper (builtins.replaceStrings ["-"] ["_"] name)}"
    );
  in
    lib.optionalAttrs lto (toRustFlags {
      lto = "thin";
    })
    // lib.optionalAttrs optimizeSize (toRustFlags {
      codegen-units = "1";
      opt-level = "s";
      panic = "abort";
      strip = "symbols";
    });

  meta = with lib; {
    mainProgram = "refraction";
    description = "Discord bot for Prism Launcher";
    homepage = "https://github.com/PrismLauncher/refraction";
    license = licenses.gpl3Plus;
    maintainers = with maintainers; [getchoo Scrumplex];
  };
}
