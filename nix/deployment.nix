{
  inputs,
  flake-parts-lib,
  withSystem,
  ...
}: {
  flake.nixosModules.default = flake-parts-lib.importApply ./module.nix {
    inherit withSystem;
  };

  perSystem = {
    lib,
    pkgs,
    system,
    self',
    inputs',
    ...
  }: let
    crossPkgs =
      rec {
        x86_64-linux = {
          x86_64 = pkgs.pkgsStatic;
          aarch64 = pkgs.pkgsCross.aarch64-multiplatform.pkgsStatic;
        };

        aarch64-linux = {
          x86_64 = pkgs.pkgsCross.musl64;
          aarch64 = pkgs.pkgsStatic;
        };

        x86_64-darwin = {
          x86_64 = pkgs.pkgsCross.musl64;
          aarch64 = pkgs.pkgsCross.aarch64-multiplatform.pkgsStatic;
        };

        aarch64-darwin = x86_64-darwin;
      }
      .${system};

    refractionFor = arch: let
      inherit (crossPkgs.${arch}.stdenv) cc;

      target = "${arch}-unknown-linux-musl";
      target' = builtins.replaceStrings ["-"] ["_"] target;
      targetUpper = lib.toUpper target';

      toolchain = with inputs'.fenix.packages;
        combine [
          minimal.cargo
          minimal.rustc
          targets.${target}.latest.rust-std
        ];

      naersk' = inputs.naersk.lib.${system}.override {
        cargo = toolchain;
        rustc = toolchain;
      };

      refraction = self'.packages.refraction.override {
        lto = true;
        naersk = naersk';
      };

      newAttrs = {
        CARGO_BUILD_TARGET = target;
        "CC_${target'}" = "${cc}/bin/${cc.targetPrefix}cc";
        "CARGO_TARGET_${targetUpper}_RUSTFLAGS" = "-C target-feature=+crt-static";
        "CARGO_TARGET_${targetUpper}_LINKER" = newAttrs."CC_${target'}";
      };
    in
      refraction.overrideAttrs newAttrs;

    containerFor = arch:
      pkgs.dockerTools.buildLayeredImage {
        name = "refraction";
        tag = "latest-${arch}";
        contents = [pkgs.dockerTools.caCertificates];
        config.Cmd = [
          (lib.getExe (refractionFor arch))
        ];

        architecture = crossPkgs.${arch}.go.GOARCH;
      };

    mkPackagesFor = arch: {
      "refraction-static-${arch}" = refractionFor arch;
      "container-${arch}" = containerFor arch;
    };
  in {
    legacyPackages = lib.attrsets.mergeAttrsList (map mkPackagesFor ["x86_64" "aarch64"]);
  };
}
