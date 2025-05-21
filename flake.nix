{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";
    nixpkgs-stable.url = "github:nixos/nixpkgs?ref=nixos-24.11";
    utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, utils, ... } @ inputs: utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      pkgs-stable = inputs.nixpkgs-stable.legacyPackages.${system};
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = (with pkgs; [
          cargo
          rustc
          rust-analyzer
          rustfmt
          pnpm
          nodejs
          vtsls
          entr
          findutils
          ripgrep
          posting
          sea-orm-cli
          tailwindcss-language-server
        ]);
      };
    }
  );
}
