{
  lib,
  stdenv,
  naersk,
  CoreFoundation,
  Security,
  SystemConfiguration,
  version,
  optimizeSize ? false,
}: let
  filter = path: type: let
    path' = toString path;
    base = baseNameOf path';

    dirBlocklist = ["nix"];

    matches = lib.any (suffix: lib.hasSuffix suffix base) [".rs"];
    isCargo = base == "Cargo.lock" || base == "Cargo.toml";
    isAllowedDir = !(builtins.elem base dirBlocklist);
  in
    (type == "directory" && isAllowedDir) || matches || isCargo;

  filterSource = src:
    lib.cleanSourceWith {
      src = lib.cleanSource src;
      inherit filter;
    };
in
  naersk.buildPackage {
    pname = "refraction";
    inherit version;

    src = filterSource ../.;

    buildInputs = lib.optionals stdenv.hostPlatform.isDarwin [
      CoreFoundation
      Security
      SystemConfiguration
    ];

    RUSTFLAGS = lib.optionalString optimizeSize "-C codegen-units=1 -C strip=symbols -C opt-level=z";

    meta = with lib; {
      mainProgram = "refraction";
      description = "Discord bot for Prism Launcher";
      homepage = "https://github.com/PrismLauncher/refraction";
      license = licenses.gpl3Plus;
      platforms = with platforms; linux ++ darwin;
      maintainers = with maintainers; [getchoo Scrumplex];
    };
  }
