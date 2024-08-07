{
  description = "Discord bot for Prism Launcher";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems = fn: nixpkgs.lib.genAttrs systems (system: fn nixpkgs.legacyPackages.${system});
    in
    {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            redis

            # linters & formatters
            actionlint
            nodePackages.prettier

            # rust tools
            clippy
            rustfmt
            rust-analyzer

            # nix tools
            self.formatter.${system}
            deadnix
            nil
            statix
          ];

          inputsFrom = [ self.packages.${pkgs.system}.refraction ];
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };
      });

      formatter = forAllSystems (pkgs: pkgs.nixfmt-rfc-style);

      nixosModules.default = import ./nix/module.nix self;

      packages = forAllSystems (
        {
          lib,
          pkgs,
          system,
          ...
        }:
        let
          packages' = self.packages.${system};

          mkStatic = pkgs.callPackage ./nix/static.nix {
            inherit (self.packages.${pkgs.system}) refraction;
            rust-overlay = rust-overlay.packages.${system};
          };

          mkContainerFor =
            refraction:
            pkgs.dockerTools.buildLayeredImage {
              name = "refraction";
              tag = "latest-${refraction.stdenv.hostPlatform.qemuArch}";
              config.Cmd = [ (lib.getExe refraction) ];
              inherit (refraction) architecture;
            };
        in
        {
          refraction = pkgs.callPackage ./nix/derivation.nix { inherit self; };

          static-x86_64 = mkStatic { arch = "x86_64"; };
          static-aarch64 = mkStatic { arch = "aarch64"; };
          container-x86_64 = mkContainerFor packages'.static-x86_64;
          container-aarch64 = mkContainerFor packages'.static-aarch64;

          default = packages'.refraction;
        }
      );

      overlays.default = _: prev: {
        refraction = prev.callPackage ./nix/derivation.nix { inherit self; };
      };
    };
}
