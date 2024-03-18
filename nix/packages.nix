{
  self,
  inputs,
  ...
}: {
  perSystem = {
    pkgs,
    system,
    self',
    ...
  }: {
    packages = {
      refraction = pkgs.callPackage ./derivation.nix {
        version = builtins.substring 0 7 self.rev or "dirty";
        naersk = inputs.naersk.lib.${system};
      };

      default = self'.packages.refraction;
    };
  };
}
