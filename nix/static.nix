{
  lib,
  refraction,
  rust-overlay,
  pkgsCross,
}: {arch}: let
  targets = with pkgsCross; {
    x86_64 = musl64.pkgsStatic;
    aarch64 = aarch64-multiplatform.pkgsStatic;
  };

  getRustcTarget = pkgs: pkgs.stdenv.hostPlatform.rust.rustcTarget;
  toolchain = rust-overlay.rust.minimal.override {
    extensions = ["rust-std"];
    targets = lib.mapAttrsToList (lib.const getRustcTarget) targets;
  };

  mkRustPlatformWith = pkgs:
    pkgs.makeRustPlatform (
      lib.genAttrs ["cargo" "rustc"] (lib.const toolchain)
    );
  rustPlatforms = lib.mapAttrs (lib.const mkRustPlatformWith) targets;
in
  refraction.override {
    rustPlatform = rustPlatforms.${arch};
    optimizeSize = true;
  }
