{
  inputs,
  self,
  ...
}: {
  flake.nixosModules.default = import ./module.nix self;

  perSystem = {
    lib,
    pkgs,
    system,
    self',
    inputs',
    ...
  }: let
    crossPkgsFor =
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

    exeFor = arch: let
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
        naersk = naersk';
        optimizeSize = true;
      };

      newAttrs = lib.fix (finalAttrs: {
        CARGO_BUILD_TARGET = target;
        "CC_${target'}" = "${cc}/bin/${cc.targetPrefix}cc";
        "CARGO_TARGET_${targetUpper}_RUSTFLAGS" = "-C target-feature=+crt-static";
        "CARGO_TARGET_${targetUpper}_LINKER" = finalAttrs."CC_${target'}";
      });

      inherit (crossPkgsFor.${arch}.stdenv) cc;
    in
      lib.getExe (
        refraction.overrideAttrs (_: newAttrs)
      );

    containerFor = arch:
      pkgs.dockerTools.buildLayeredImage {
        name = "refraction";
        tag = "latest-${arch}";
        contents = [pkgs.dockerTools.caCertificates];
        config.Cmd = [(exeFor arch)];

        architecture = crossPkgsFor.${arch}.go.GOARCH;
      };
  in {
    legacyPackages = {
      container-x86_64 = containerFor "x86_64";
      container-aarch64 = containerFor "aarch64";
    };
  };
}
