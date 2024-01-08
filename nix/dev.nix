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
      rustfmt.enable = true;
      nil.enable = true;
      prettier = {
        enable = true;
        excludes = ["flake.lock"];
      };
    };

    proc.groups.daemons.processes = {
      redis.command = "${lib.getExe' pkgs.redis "redis-server"}";
    };

    devShells.default = pkgs.mkShell {
      shellHook = ''
        ${config.pre-commit.installationScript}
      '';

      packages = with pkgs; [
        # general
        actionlint
        config.proc.groups.daemons.package

        # rust
        cargo
        rustc
        clippy
        rustfmt
        rust-analyzer

        # nix
        self'.formatter
        nil
      ];

      RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };

    formatter = pkgs.alejandra;
  };
}
