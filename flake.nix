{
  description = "Discord bot for Prism Launcher";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    procfile-nix = {
      url = "github:getchoo/procfile-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {parts, ...} @ inputs:
    parts.lib.mkFlake {inherit inputs;} {
      imports = [
        ./nix/dev.nix
        ./nix/packages.nix
        ./nix/deployment.nix

        inputs.pre-commit-hooks.flakeModule
        inputs.procfile-nix.flakeModule
      ];

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
    };
}
