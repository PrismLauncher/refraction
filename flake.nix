{
  description = "Discord bot for Prism Launcher";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs =
    {
      self,
      nixpkgs,
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
      nixpkgsFor = nixpkgs.legacyPackages;
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
        in
        {
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
              nil
              statix
            ];

            inputsFrom = [ self.packages.${pkgs.system}.refraction ];
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          };
        }
      );

      formatter = forAllSystems (system: nixpkgsFor.${system}.nixfmt-rfc-style);

      nixosModules.default = import ./nix/module.nix self;

      # For CI
      legacyPackages = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
        in
        {
          clippy-report = pkgs.callPackage ./nix/clippy.nix { inherit (self.packages.${system}) refraction; };

          refraction-debug = (self.packages.${system}.refraction.override { lto = false; }).overrideAttrs (
            finalAttrs: _: {
              cargoBuildType = "debug";
              cargoCheckType = finalAttrs.cargoBuildType;
            }
          );
        }
      );

      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
          packages' = self.packages.${system};

          refractionPackages = lib.makeScope pkgs.newScope (lib.flip self.overlays.default pkgs);

          mkStatic = pkgs.callPackage ./nix/static.nix { };
          containerize = pkgs.callPackage ./nix/containerize.nix { };
        in
        {
          inherit (refractionPackages) refraction;

          static-x86_64 = mkStatic { arch = "x86_64"; };
          static-aarch64 = mkStatic { arch = "aarch64"; };
          container-amd64 = containerize packages'.static-x86_64;
          container-arm64 = containerize packages'.static-aarch64;

          default = packages'.refraction;
        }
      );

      overlays.default = final: _: {
        refraction = final.callPackage ./nix/package.nix { };
      };
    };
}
