{
  perSystem = {
    config,
    lib,
    ...
  }: {
    pre-commit.settings = {
      rootSrc = lib.mkForce ../../.;
      hooks = {
        actionlint.enable = true;
        nil.enable = true;
        statix.enable = true;
        treefmt = {
          enable = true;
          package = config.treefmt.build.wrapper;
        };
      };
    };
  };
}
