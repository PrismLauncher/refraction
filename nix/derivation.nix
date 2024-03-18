{
  lib,
  stdenv,
  naersk,
  darwin,
  version,
  lto ? false,
}:
naersk.buildPackage {
  pname = "refraction";
  inherit version;

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

  buildInputs = lib.optionals stdenv.hostPlatform.isDarwin (with darwin.apple_sdk.frameworks; [
    CoreFoundation
    Security
    SystemConfiguration
  ]);

  cargoBuildFlags = lib.optionals lto ["-C" "lto=thin" "-C" "embed-bitcode=yes" "-Zdylib-lto"];

  meta = with lib; {
    mainProgram = "refraction";
    description = "Discord bot for Prism Launcher";
    homepage = "https://github.com/PrismLauncher/refraction";
    license = licenses.gpl3Plus;
    maintainers = with maintainers; [getchoo Scrumplex];
  };
}
