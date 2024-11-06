{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      imports = [
        inputs.flake-parts.flakeModules.modules
      ];
      perSystem = {
        config,
        self',
        pkgs,
        lib,
        system,
        ...
      }: let
        runtimeDeps = with pkgs; [systemd systemdLibs];
        buildDeps = with pkgs; [pkg-config rustPlatform.bindgenHook];
        devDeps = with pkgs; [gdb];

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        msrv = cargoToml.package.rust-version;

        rustPackage = features:
          (pkgs.makeRustPlatform {
            cargo = pkgs.rust-bin.stable.latest.minimal;
            rustc = pkgs.rust-bin.stable.latest.minimal;
          })
          .buildRustPackage {
            inherit (cargoToml.package) name version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            buildFeatures = features;
            buildInputs = runtimeDeps;
            nativeBuildInputs = buildDeps;
          };

        mkDevShell = rustc:
          pkgs.mkShell {
            shellHook = ''
              export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
            '';
            buildInputs = runtimeDeps;
            nativeBuildInputs = buildDeps ++ devDeps ++ [rustc];
          };
      in {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [(import inputs.rust-overlay)];
        };

        packages.default = self'.packages.server-tui;
        devShells.default = self'.devShells.stable;

        packages.server-tui = rustPackage "";

        devShells.nightly = mkDevShell (pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default));
        devShells.stable = mkDevShell (pkgs.rust-bin.stable.latest.default);
        devShells.msrv = mkDevShell (pkgs.rust-bin.stable.${msrv}.default);
      };
    };
}