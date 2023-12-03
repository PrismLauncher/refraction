{
  self,
  inputs,
  ...
}: {
  perSystem = {
    pkgs,
    system,
    config,
    ...
  }: {
    packages = {
      refraction = pkgs.callPackage ./derivation.nix {
        version = builtins.substring 0 8 self.lastModifiedDate or "dirty";

        inherit
          (pkgs.darwin.apple_sdk.frameworks)
          CoreFoundation
          Security
          SystemConfiguration
          ;

        naersk = inputs.naersk.lib.${system};
      };

      default = config.packages.refraction;
    };
  };
}
