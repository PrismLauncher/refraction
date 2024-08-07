{ pkgsCross }:
let
  crossPlatformFor = with pkgsCross; {
    x86_64 = musl64.pkgsStatic;
    aarch64 = aarch64-multiplatform.pkgsStatic;
  };
in
{ arch }:
crossPlatformFor.${arch}.callPackage ./package.nix { optimizeSize = true; }
