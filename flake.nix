{
  description = "skindle";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShell = pkgs.mkShell {
        name = "skindle";
        buildInputs = with pkgs; [
          cargo
          rustc
          rust-analyzer
          rustfmt
          calibre
        ];
      };

      defaultPackage = pkgs.rustPlatform.buildRustPackage {
        pname = "skindle";
        version = "1.0.0";
        src = ./.;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };
        nativeBuildInputs = with pkgs; [ pkg-config openssl ];
        buildInputs = with pkgs; [ pkg-config openssl ];
      };
    });
}
