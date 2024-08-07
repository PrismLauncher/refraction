{
  lib,
  refraction,
  rust-overlay,
  pkgsCross,
}:
let
  crossPlatformFor = with pkgsCross; {
    x86_64 = musl64.pkgsStatic;
    aarch64 = aarch64-multiplatform.pkgsStatic;
  };

  toolchain = rust-overlay.rust.minimal.override {
    extensions = [ "rust-std" ];
    targets = lib.mapAttrsToList (_: pkgs: pkgs.stdenv.hostPlatform.rust.rustcTarget) crossPlatformFor;
  };

  rustPlatformFor = lib.mapAttrs (
    _: pkgs:
    pkgs.makeRustPlatform (
      lib.genAttrs [
        "cargo"
        "rustc"
      ] (_: toolchain)
    )
  ) crossPlatformFor;
in
{ arch }:
(refraction.override {
  rustPlatform = rustPlatformFor.${arch};
  optimizeSize = true;
}).overrideAttrs
  (old: {
    passthru = old.passthru or { } // {
      inherit toolchain;
    };
  })
