{
  description = "skindle";

  inputs = { 
    nixpkgs.url = "github:nixos/nixpkgs/master";
  };

  outputs = { self, nixpkgs }: let
    pkgs = import nixpkgs {
      system = "x86_64-linux";
    };

  in {
    devShell.x86_64-linux = pkgs.mkShell {
      name = "skindle";
      buildInputs = with pkgs; [
        cargo
        rustc
        rust-analyzer
        rustfmt
      ];
      shellHook = ''
      '';
    };
  };
}
