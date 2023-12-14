{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        deployment.targetHost = "soz.fish";
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
          buildInputs = [ cargo
            rustc 
            rustfmt
            pre-commit
            rustPackages.clippy
            pkg-config
            openssl
            bacon
            sqlite
            postgresql
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
