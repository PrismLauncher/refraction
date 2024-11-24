{
  cargo,
  clippy,
  clippy-sarif,
  refraction,
  rustPlatform,
  sarif-fmt,
  stdenv,
}:

stdenv.mkDerivation {
  pname = "${refraction.pname}-sarif-report";
  inherit (refraction)
    version
    src
    cargoDeps
    buildInputs
    ;

  nativeBuildInputs = [
    cargo
    clippy
    clippy-sarif
    rustPlatform.cargoSetupHook
    sarif-fmt
  ];

  buildPhase = ''
    cargo clippy \
      --all-features \
      --all-targets \
      --tests \
      --message-format=json \
    | clippy-sarif | tee $out | sarif-fmt
  '';

  dontInstall = true;
  dontFixup = true;
}
