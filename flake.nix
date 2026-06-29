{
  description = "SAKEM@S bot - Rust Discord bot";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, crane }:
    let
      lib = nixpkgs.lib;
      supportedSystems = [ "aarch64-darwin" "aarch64-linux" "x86_64-linux" ];
      forAllSystems = lib.genAttrs supportedSystems;

      sakemasBotFor = pkgs:
        let
          craneLib = crane.mkLib pkgs;
          src = craneLib.cleanCargoSource ./.;
          commonArgs = {
            inherit src;
            strictDeps = true;
            buildInputs = with pkgs; [
              openssl
              libopus
              ffmpeg
            ];
            nativeBuildInputs = with pkgs; [
              pkg-config
              cmake
            ];
          };
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        in
        craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          pname = "sakemas-bot";
          version = "0.0.6";
        });
    in
    {
      # The OCI boot image and the bot package for aarch64-linux must be built
      # on an aarch64-linux host or remote builder. macOS can evaluate the flake
      # but cannot build the qcow2 image locally.
      packages = forAllSystems (system:
        let
          sakemas-bot = sakemasBotFor nixpkgs.legacyPackages.${system};
        in
        {
          default = sakemas-bot;
          sakemas-bot = sakemas-bot;
        } // lib.optionalAttrs (system == "aarch64-linux") {
          oci-image = self.nixosConfigurations.sakemas-oci.config.system.build.OCIImage;
        });

      nixosConfigurations.sakemas-oci = nixpkgs.lib.nixosSystem {
        system = "aarch64-linux";
        modules = [
          "${nixpkgs}/nixos/modules/virtualisation/oci-image.nix"
          ./nix/nixos-modules/sakemas-bot.nix
          ({ config, pkgs, lib, ... }: {
            nixpkgs.hostPlatform = "aarch64-linux";
            system.stateVersion = "25.05";

            # oci-image.nix (via oci-common.nix) sets EFI GRUB and the root
            # filesystem automatically for OCI. Avoid overriding those defaults
            # so the produced qcow2 image boots on Oracle Cloud ARM instances.

            # Minimal SSH access and firewall.
            services.openssh.enable = true;
            services.openssh.settings.PermitRootLogin = lib.mkForce "prohibit-password";
            networking.firewall.allowedTCPPorts = [ 22 ];

            # Keep the closure small on the free tier VM.
            environment.defaultPackages = [ ];

            # Enable the bot service and its PostgreSQL dependency.
            services.sakemas-bot = {
              enable = true;
              package = sakemasBotFor pkgs;
            };
          })
        ];
      };

      devShells = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
              rustc
              rustfmt
              clippy
              sqlx-cli
              pkg-config
              openssl
              libopus
              ffmpeg
              cmake
            ];
          };
        });
    };
}
