{
  perSystem = {
    lib,
    pkgs,
    config,
    self',
    ...
  }: {
    devShells.default = pkgs.mkShell {
      shellHook = ''
        ${config.pre-commit.installationScript}
      '';

      packages = with pkgs; [
        # general
        actionlint
        nodePackages.prettier
        config.procfiles.daemons.package

        # rust
        clippy
        rustfmt
        rust-analyzer

        # nix
        self'.formatter
        deadnix
        nil
        statix
      ];

      inputsFrom = [self'.packages.refraction];
      RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
    };

    treefmt = {
      projectRootFile = "flake.nix";

      programs = {
        alejandra.enable = true;
        deadnix.enable = true;
        prettier.enable = true;
        rustfmt.enable = true;
      };

      settings.global = {
        excludes = [
          "./target"
          "./flake.lock"
          "./Cargo.lock"
        ];
      };
    };

    pre-commit.settings.hooks = {
      actionlint.enable = true;
      nil.enable = true;
      statix.enable = true;
      treefmt = {
        enable = true;
        package = config.treefmt.build.wrapper;
      };
    };

    procfiles.daemons.processes = {
      redis = lib.getExe' pkgs.redis "redis-server";
    };
  };
}
