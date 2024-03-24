{
  flake-parts-lib,
  withSystem,
  ...
}: {
  imports = [./static.nix];

  flake.nixosModules.default = flake-parts-lib.importApply ./module.nix {
    inherit withSystem;
  };

  perSystem = {
    lib,
    pkgs,
    self',
    ...
  }: let
    containerFor = arch:
      pkgs.dockerTools.buildLayeredImage {
        name = "refraction";
        tag = "latest-${arch}";
        contents = [pkgs.dockerTools.caCertificates];
        config.Cmd = [
          (lib.getExe self'.packages."refraction-static-${arch}")
        ];

        architecture = withSystem "${arch}-linux" ({pkgs, ...}: pkgs.pkgsStatic.go.GOARCH);
      };
  in {
    packages = {
      container-x86_64 = containerFor "x86_64";
      container-aarch64 = containerFor "aarch64";
    };
  };
}
