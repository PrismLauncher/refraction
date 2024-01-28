{
  perSystem = {
    lib,
    pkgs,
    config,
    self',
    ...
  }: {
    pre-commit.settings.hooks = {
      actionlint.enable = true;
      alejandra.enable = true;
      deadnix.enable = true;
      rustfmt.enable = true;
      statix.enable = true;
      nil.enable = true;
      prettier = {
        enable = true;
        excludes = ["flake.lock"];
      };
    };

    procfiles.daemons.processes = {
      redis = lib.getExe' pkgs.redis "redis-server";
    };

    devShells.default = pkgs.mkShell {
      shellHook = ''
        ${config.pre-commit.installationScript}
      '';

      packages = with pkgs; [
        # general
        actionlint
        config.procfiles.daemons.package

        # rust
        cargo
        rustc
        clippy
        rustfmt
        rust-analyzer

        # nix
        self'.formatter
        deadnix
        nil
        statix
      ];

      RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };

    formatter = pkgs.alejandra;
  };
}
