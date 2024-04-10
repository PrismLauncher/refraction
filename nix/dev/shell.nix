{
  perSystem = {
    pkgs,
    config,
    self',
    refraction',
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

      inputsFrom = [refraction'.packages.refraction];
      RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
    };
  };
}
