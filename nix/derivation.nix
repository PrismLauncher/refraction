{
  lib,
  stdenv,
  naersk,
  CoreFoundation,
  Security,
  SystemConfiguration,
  version,
  optimizeSize ? false,
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

  buildInputs = lib.optionals stdenv.hostPlatform.isDarwin [
    CoreFoundation
    Security
    SystemConfiguration
  ];

  cargoBuildFlags = lib.optionals optimizeSize ["-C" "codegen-units=1" "-C" "strip=symbols" "-C" "opt-level=z"];

  meta = with lib; {
    mainProgram = "refraction";
    description = "Discord bot for Prism Launcher";
    homepage = "https://github.com/PrismLauncher/refraction";
    license = licenses.gpl3Plus;
    maintainers = with maintainers; [getchoo Scrumplex];
  };
}
