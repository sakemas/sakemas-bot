{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.services.sakemas-bot;
in
{
  options.services.sakemas-bot = {
    enable = mkEnableOption "SAKEM@S Discord bot";

    package = mkOption {
      type = types.package;
      default = pkgs.sakemas-bot or (throw "No sakemas-bot package available");
      description = "The SAKEM@S bot package to run.";
    };

    secretsFile = mkOption {
      type = types.path;
      default = "/etc/sakemas-bot/secrets.env";
      description = ''
        systemd EnvironmentFile path containing secrets such as DISCORD_TOKEN
        and DATABASE_URL.

        The file must be in KEY=VALUE format (systemd EnvironmentFile format).
        Example:

          DISCORD_TOKEN=...
          DATABASE_URL=postgres://sakemas_bot:password@localhost/sakemas_bot
          VC_ANNOUNCEMENT_CHANNEL=...
          WELCOME_CHANNEL=...
          CAUTION_CHANNEL=...
          INTRODUCTION_CHANNEL=...
          X_POSTER_CHANNEL=...
          TWITTER_CLIENT_ID=...
          TWITTER_CLIENT_SECRET=...

        Do not commit this file to the repository.
      '';
    };

    databaseUrlFile = mkOption {
      type = types.nullOr types.path;
      default = null;
      description = ''
        Optional separate EnvironmentFile path for DATABASE_URL. When set, it
        is loaded in addition to secretsFile. This lets you keep DB
        credentials outside the main secrets file.
      '';
    };
  };

  config = mkIf cfg.enable {
    users.users.sakemas-bot = {
      isSystemUser = true;
      group = "sakemas-bot";
      home = "/var/lib/sakemas-bot";
      createHome = true;
      description = "SAKEM@S bot daemon user";
    };

    users.groups.sakemas-bot = { };

    services.postgresql = {
      enable = true;
      ensureDatabases = [ "sakemas_bot" ];
      ensureUsers = [
        {
          name = "sakemas_bot";
          ensureDBOwnership = true;
        }
      ];
    };

    systemd.services.sakemas-bot = {
      description = "SAKEM@S Discord bot";
      wantedBy = [ "multi-user.target" ];
      after = [ "network-online.target" "postgresql.service" ];
      requires = [ "postgresql.service" ];
      wants = [ "network-online.target" ];

      serviceConfig = {
        Type = "simple";
        User = "sakemas-bot";
        Group = "sakemas-bot";
        WorkingDirectory = "/var/lib/sakemas-bot";
        EnvironmentFile =
          if cfg.databaseUrlFile != null then
            [ cfg.secretsFile cfg.databaseUrlFile ]
          else
            cfg.secretsFile;
        ExecStart = "${cfg.package}/bin/sakemas-bot";
        Restart = "on-failure";
        RestartSec = "10s";

        # Hardening
        NoNewPrivileges = true;
        PrivateTmp = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        ReadWritePaths = "/var/lib/sakemas-bot";
      };
    };

    systemd.tmpfiles.rules = [
      "d /etc/sakemas-bot 0750 root root -"
      "d /var/lib/sakemas-bot 0750 sakemas-bot sakemas-bot -"
    ];
  };
}
