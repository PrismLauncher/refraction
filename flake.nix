{
  description = "Discord bot for Prism Launcher";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      treefmt-nix,
    }:
    let
      inherit (nixpkgs) lib;
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems = lib.genAttrs systems;
      nixpkgsFor = forAllSystems (system: nixpkgs.legacyPackages.${system});
      treefmtFor = forAllSystems (system: treefmt-nix.lib.evalModule nixpkgsFor.${system} ./treefmt.nix);
    in
    {
      checks = forAllSystems (system: {
        treefmt = treefmtFor.${system}.config.build.check self;
      });

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              redis
              self.formatter.${system}

              # linters & formatters
              actionlint
              nodePackages.prettier

              # rust tools
              clippy
              rustfmt
              rust-analyzer

              # nix tools
              nixfmt-rfc-style
              nil
              statix
            ];

            inputsFrom = [ self.packages.${pkgs.system}.refraction ];
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          };
        }
      );

      formatter = forAllSystems (system: treefmtFor.${system}.config.build.wrapper);

      nixosModules.default = import ./nix/module.nix self;

      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
          packages' = self.packages.${system};

          mkStatic = pkgs.callPackage ./nix/static.nix {
            inherit (packages') refraction;
            rust-overlay = rust-overlay.packages.${system};
          };

          containerize =
            refraction:
            pkgs.dockerTools.buildLayeredImage {
              name = "refraction";
              tag = "latest-${refraction.passthru.architecture}";
              config.Cmd = [ (lib.getExe refraction) ];
              inherit (refraction.passthru) architecture;
            };
        in
        {
          refraction = pkgs.callPackage ./nix/derivation.nix { inherit self; };

          static-x86_64 = mkStatic { arch = "x86_64"; };
          static-aarch64 = mkStatic { arch = "aarch64"; };
          container-x86_64 = containerize packages'.static-x86_64;
          container-aarch64 = containerize packages'.static-aarch64;

          default = packages'.refraction;
        }
      );

      overlays.default = _: prev: {
        refraction = prev.callPackage ./nix/derivation.nix { inherit self; };
      };
    };
}
