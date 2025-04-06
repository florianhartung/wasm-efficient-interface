{
  description = "";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
    utils.url = "git+https://github.com/numtide/flake-utils.git";
    devshell.url = "github:numtide/devshell";
    fenix = {
      url = "git+https://github.com/nix-community/fenix.git?ref=main";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, utils, devshell, fenix }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ devshell.overlays.default ];
        };

        # rust target name of the `system`
        rust-target = pkgs.pkgsStatic.targetPlatform.rust.rustcTarget;

        rust-toolchain = with fenix.packages.${system}; combine [
          latest.rustc
          latest.cargo
          latest.clippy
          latest.rustfmt
          targets.${rust-target}.latest.rust-std
          targets.wasm32-unknown-unknown.latest.rust-std
          targets.wasm32-wasip1.latest.rust-std
        ];
      in
      {
        # a devshell with all the necessary bells and whistles
        devShells.default = (pkgs.devshell.mkShell {
          name = "unnamed";
          packages = with pkgs; [
            stdenv.cc
            coreutils
            rust-toolchain
            rust-analyzer
            cargo-audit
            cargo-expand
            cargo-watch
            cargo-component
          ];
        });
      });
}

