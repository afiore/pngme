{
  description = "Pngme statically linked against musl";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs, }:
    let
      # Should work with other targets, but not tested.
      supportedSystems = [ "x86_64-linux" ];


      # Helper function to generate an attrset '{ x86_64-linux = f "x86_64-linux"; ... }'.
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;

      # Nixpkgs instantiated for supported system types.
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = nixpkgsFor.${system}.pkgsStatic;
        in
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "pngme";
            version = "0.1.0";

            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
        });

      devShells = forAllSystems (system:
        let pkgs = nixpkgsFor.${system};
        in {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo
              rustc
              rust.packages.stable.rustPlatform.rustLibSrc
            ];
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          };
        });
    };
}
