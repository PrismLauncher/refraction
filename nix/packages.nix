{self, ...}: {
  perSystem = {
    pkgs,
    self',
    ...
  }: {
    packages = {
      refraction = pkgs.callPackage ./derivation.nix {inherit self;};
      default = self'.packages.refraction;
    };
  };

  flake.overlays.default = _: prev: {
    refraction = prev.callPackage ./derivation.nix {inherit self;};
  };
}
