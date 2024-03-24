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
}
