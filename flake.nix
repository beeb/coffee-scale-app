{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    devenv.url = "github:cachix/devenv";
  };

  outputs = { self, nixpkgs, utils, devenv, ... } @ inputs:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.outputs.legacyPackages.${system};
      in
      {
        devShell = devenv.lib.mkShell {
          inherit inputs pkgs;
          modules = [
            ({ pkgs, ... }: {
              packages = with pkgs; [
                nodePackages.pnpm
              ];
              languages.javascript = {
                enable = true;
                corepack.enable = true;
              };
              languages.typescript.enable = true;
            })
          ];
        };
      });
}
