{
  description = "Discord bot for Prism Launcher";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    forAllSystems = fn: nixpkgs.lib.genAttrs systems (system: fn nixpkgs.legacyPackages.${system});
  in {
    nixosModules.default = import ./nix/module.nix self;

    packages = forAllSystems (pkgs: rec {
      refraction = pkgs.callPackage ./nix/derivation.nix {inherit self;};
      default = refraction;
    });

    overlays.default = _: prev: {
      refraction = prev.callPackage ./nix/derivation.nix {inherit self;};
    };
  };
}
