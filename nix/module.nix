{withSystem, ...}: {
  config,
  lib,
  pkgs,
  ...
}: let
  cfg = config.services.refraction;
  defaultUser = "refraction";

  inherit
    (lib)
    getExe
    literalExpression
    mdDoc
    mkEnableOption
    mkIf
    mkOption
    mkPackageOption
    optionals
    types
    ;
in {
  options.services.refraction = {
    enable = mkEnableOption "refraction";
    package = mkPackageOption (
      withSystem pkgs.stdenv.hostPlatform.system ({pkgs, ...}: pkgs)
    ) "refraction" {};

    user = mkOption {
      description = mdDoc ''
        User under which the service should run. If this is the default value,
        the user will be created, with the specified group as the primary
        group.
      '';
      type = types.str;
      default = defaultUser;
      example = literalExpression ''
        "bob"
      '';
    };

    group = mkOption {
      description = mdDoc ''
        Group under which the service should run. If this is the default value,
        the group will be created.
      '';
      type = types.str;
      default = defaultUser;
      example = literalExpression ''
        "discordbots"
      '';
    };

    redisUrl = mkOption {
      description = mdDoc ''
        Connection to a redis server. If this needs to include credentials
        that shouldn't be world-readable in the Nix store, set environmentFile
        and override the `REDIS_URL` entry.
        Pass the string `local` to setup a local Redis database.
      '';
      type = types.str;
      default = "local";
      example = literalExpression ''
        "redis://localhost/"
      '';
    };

    environmentFile = mkOption {
      description = mdDoc ''
        Environment file as defined in {manpage}`systemd.exec(5)`
      '';
      type = types.nullOr types.path;
      default = null;
      example = literalExpression ''
        "/run/agenix.d/1/refraction"
      '';
    };
  };

  config = mkIf cfg.enable {
    services.redis.servers.refraction = mkIf (cfg.redisUrl == "local") {
      enable = true;
      inherit (cfg) user;
      port = 0; # disable tcp listener
    };

    systemd.services."refraction" = {
      enable = true;
      wantedBy = ["multi-user.target"];
      after =
        ["network.target"]
        ++ optionals (cfg.redisUrl == "local") ["redis-refraction.service"];

      script = ''
        ${getExe cfg.package}
      '';

      environment = {
        REDIS_URL =
          if cfg.redisUrl == "local"
          then "unix:${config.services.redis.servers.refraction.unixSocket}"
          else cfg.redisUrl;
      };

      serviceConfig = {
        Type = "simple";
        Restart = "on-failure";

        EnvironmentFile = mkIf (cfg.environmentFile != null) cfg.environmentFile;

        User = cfg.user;
        Group = cfg.group;

        # hardening
        NoNewPrivileges = true;
        PrivateDevices = true;
        PrivateTmp = true;
        PrivateUsers = true;
        ProtectClock = true;
        ProtectControlGroups = true;
        ProtectHome = true;
        ProtectHostname = true;
        ProtectKernelLogs = true;
        ProtectKernelModules = true;
        ProtectKernelTunables = true;
        ProtectSystem = "strict";
        RestrictNamespaces = true;
        RestrictSUIDSGID = true;
        SystemCallArchitectures = "native";
        SystemCallFilter = [
          "@system-service"
          "~@resources"
          "~@privileged"
        ];
      };
    };

    users = {
      users = mkIf (cfg.user == defaultUser) {
        ${defaultUser} = {
          isSystemUser = true;
          inherit (cfg) group;
        };
      };

      groups = mkIf (cfg.group == defaultUser) {
        ${defaultUser} = {};
      };
    };
  };
}
