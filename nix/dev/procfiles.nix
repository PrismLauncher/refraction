{
  perSystem = {
    lib,
    pkgs,
    ...
  }: {
    procfiles.daemons.processes = {
      redis = lib.getExe' pkgs.redis "redis-server";
    };
  };
}
