{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    crane = {
      url = "github:ipetkov/crane";
    };
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    pre-commit-hooks,
  }: let
    forEachSystem = nixpkgs.lib.genAttrs [
      "aarch64-darwin"
      "aarch64-linux"
      "x86_64-darwin"
      "x86_64-linux"
    ];
  in {
    checks = forEachSystem (system: let
      craneDerivations = nixpkgs.legacyPackages.${system}.callPackage ./default.nix {inherit crane;};
      pre-commit-check = pre-commit-hooks.lib.${system}.run {
        src = ../.;
        hooks = {
          actionlint.enable = true;
          alejandra.enable = true;
          prettier.enable = true;
          rustfmt.enable = true;

          cargo-deny-check = {
            enable = true;
            name = "cargo-deny check";
            entry = "cargo deny check";
            files = "^Cargo\\.(toml|lock)$";
            pass_filenames = false;
            language = "system";
          };
        };
      };
    in {
      inherit
        (craneDerivations)
        myCrate
        myCrateClippy
        myCrateCoverage
        ;
      inherit pre-commit-check;
    });

    devShells = forEachSystem (system: {
      default = nixpkgs.legacyPackages.${system}.mkShell {
        nativeBuildInputs = with nixpkgs.legacyPackages.${system}; [
          cargo
          cargo-deny
          clippy
          rust-analyzer
          rustc
          rustfmt
        ];

        inherit (self.checks.${system}.pre-commit-check) shellHook;
      };
    });
  };
}
