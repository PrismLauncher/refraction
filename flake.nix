{
  description = "Discord bot for Prism Launcher";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    flake-parts,
    pre-commit-hooks,
    ...
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        pre-commit-hooks.flakeModule
      ];

      perSystem = {
        config,
        lib,
        pkgs,
        ...
      }: {
        pre-commit.settings.hooks = {
          alejandra.enable = true;
          prettier = {
            enable = true;
            excludes = ["flake.lock" "pnpm-lock.yaml"];
          };
        };
        devShells.default = pkgs.mkShell {
          shellHook = ''
            ${config.pre-commit.installationScript}
          '';
          packages = with pkgs; [nodePackages.pnpm redis];
        };
        formatter = pkgs.alejandra;
      };

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
    };
}
