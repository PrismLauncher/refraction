{
  description = "Discord bot for Prism Launcher";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    # Inputs below this are optional
    # You may remove them with `inputs.<name>.follows = ""`

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
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
      checks = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
          inherit (self.packages.${system}) refraction;
        in
        {
          treefmt = treefmtFor.${system}.config.build.check self;
          clippy = pkgs.stdenv.mkDerivation {
            pname = "check-clippy";
            inherit (refraction)
              version
              src
              cargoDeps
              buildInputs
              ;

            nativeBuildInputs = [
              pkgs.cargo
              pkgs.clippy
              pkgs.clippy-sarif
              pkgs.sarif-fmt
            ];

            buildPhase = ''
              cargo clippy \
                --all-features \
                --all-targets \
                --tests \
                --message-format=json \
              | clippy-sarif | tee $out | sarif fmt
            '';
          };
        }
      );

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

          mkStatic = pkgs.callPackage ./nix/static.nix { };
          containerize = pkgs.callPackage ./nix/containerize.nix { };
        in
        {
          refraction = pkgs.callPackage ./nix/derivation.nix { };

          static-x86_64 = mkStatic { arch = "x86_64"; };
          static-aarch64 = mkStatic { arch = "aarch64"; };
          container-amd64 = containerize packages'.static-x86_64;
          container-arm64 = containerize packages'.static-aarch64;

          default = packages'.refraction;
        }
        // lib.mapAttrs' (name: lib.nameValuePair "check-${name}") self.checks.${system}
      );

      overlays.default = _: prev: {
        refraction = prev.callPackage ./nix/derivation.nix { inherit self; };
      };
    };
}
