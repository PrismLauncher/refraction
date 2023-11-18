{
  description = "Discord bot for Prism Launcher";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-root.url = "github:srid/flake-root";
    proc-flake.url = "github:srid/proc-flake";
  };

  outputs = {
    flake-parts,
    pre-commit-hooks,
    flake-root,
    proc-flake,
    ...
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        pre-commit-hooks.flakeModule
        flake-root.flakeModule
        proc-flake.flakeModule
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
        proc.groups.daemons.processes = {
          redis.command = "${lib.getExe' pkgs.redis "redis-server"}";
        };
        devShells.default = pkgs.mkShell {
          shellHook = ''
            ${config.pre-commit.installationScript}
          '';
          nativeBuildInputs = [config.proc.groups.daemons.package];
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
