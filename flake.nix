{
  nixConfig.extra-substituters = ["https://ericcrosson-git-dl.cachix.org"];
  nixConfig.extra-trusted-public-keys = ["ericcrosson-git-dl.cachix.org-1:qhFI0OIKhtlyEQeKRnyfXryIiDkk/p8R77xfjiOfntM="];

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    crane = {
      url = "github:ipetkov/crane";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    fenix,
  }: let
    forEachSystem = nixpkgs.lib.genAttrs [
      "aarch64-darwin"
      "aarch64-linux"
      "x86_64-darwin"
      "x86_64-linux"
    ];
  in {
    packages = forEachSystem (system: let
      craneDerivations = nixpkgs.legacyPackages.${system}.callPackage ./nix/default.nix {
        inherit crane fenix;
      };
    in {
      default = craneDerivations.myCrate;
    });
  };
}
