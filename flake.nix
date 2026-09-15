{
  description = "a geometry dash recreation in the terminal";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      naersk,
      ...
    }@inputs:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      eachSystem = f: nixpkgs.lib.genAttrs systems (system: f system (nixpkgs.legacyPackages.${system}));
    in
    {
      packages = eachSystem (
        system: pkgs:
        let
          naerskLib = pkgs.callPackage naersk { };
        in
        {
          default = naerskLib.buildPackage {
            name = "termdash";
            version = "1.0.0";
            src = ./.;
          };
        }
      );
    };
}
