{
  description = "Discord bot for Prism Launcher";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    flake-checks.url = "github:getchoo/flake-checks";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-checks,
    rust-overlay,
  }: let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    forAllSystems = fn: nixpkgs.lib.genAttrs systems (system: fn nixpkgs.legacyPackages.${system});
  in {
    checks = forAllSystems (pkgs: let
      flake-checks' = flake-checks.lib.mkChecks {
        inherit pkgs;
        root = ./.;
      };
    in {
      check-actionlint = flake-checks'.actionlint;
      check-alejandra = flake-checks'.alejandra;
      check-deadnix = flake-checks'.deadnix;
      check-rustfmt = flake-checks'.rustfmt;
      check-statix = flake-checks'.statix;
    });

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

        inputsFrom = [self.packages.${pkgs.system}.refraction];
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
      };
    });

    formatter = forAllSystems (pkgs: pkgs.alejandra);

    nixosModules.default = import ./nix/module.nix self;

    packages = forAllSystems ({
      lib,
      pkgs,
      system,
      ...
    }: let
      packages' = self.packages.${system};

      mkStatic = pkgs.callPackage ./nix/static.nix {
        inherit (self.packages.${pkgs.system}) refraction;
        rust-overlay = rust-overlay.packages.${system};
      };

      mkContainerFor = refraction:
        pkgs.dockerTools.buildLayeredImage {
          name = "refraction";
          tag = "latest-${refraction.stdenv.hostPlatform.qemuArch}";
          config.Cmd = [(lib.getExe refraction)];
          inherit (refraction) architecture;
        };
    in {
      refraction = pkgs.callPackage ./nix/derivation.nix {inherit self;};

      static-x86_64 = mkStatic {arch = "x86_64";};
      static-aarch64 = mkStatic {arch = "aarch64";};
      container-x86_64 = mkContainerFor packages'.static-x86_64;
      container-aarch64 = mkContainerFor packages'.static-aarch64;

      default = packages'.refraction;
    });

    overlays.default = _: prev: {
      refraction = prev.callPackage ./nix/derivation.nix {inherit self;};
    };
  };
}
