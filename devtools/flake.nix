{
  description = "Allows sharing dependencies between dev tools and broc";

  inputs = {
    # change this path to the path of your broc folder
    broc.url = "path:/home/username/gitrepos/broc1/broc";
    # to easily make configs for multiple architectures
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, broc, flake-utils }:
    let supportedSystems = [ "x86_64-linux" "x86_64-darwin" "aarch64-darwin" ];
    in flake-utils.lib.eachSystem supportedSystems (system:
      let
        pkgs = import broc.inputs.nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };

        brocShell = broc.devShell.${system};
      in {
        devShell = pkgs.mkShell {
          packages = let
            devInputs = (with pkgs; [ less gdb bashInteractive]);
            vscodeWithExtensions = pkgs.vscode-with-extensions.override {
              vscodeExtensions = with pkgs.vscode-extensions; [
                matklad.rust-analyzer
                eamodio.gitlens
                bbenoist.nix
                vadimcn.vscode-lldb
                tamasfe.even-better-toml
              ]
                  ++ pkgs.vscode-utils.extensionsFromVscodeMarketplace [
                     {
                        name = "roc-lang-support";
                        publisher = "benjamin-thomas";
                        version = "0.0.3";
                        # keep this sha for the first run, nix will tell you the correct one to change it to
                        sha256 = "sha256-mabNegZ+XPQ6EIHFk6jz2mAPLHAU6Pm3w0SiFB7IE+s=";
                      }
                    ]
                  ;

            };
          in [ vscodeWithExtensions devInputs ];

          inputsFrom = [ brocShell ];

          # env vars
          LLVM_SYS_130_PREFIX = brocShell.LLVM_SYS_130_PREFIX;
          NIX_GLIBC_PATH = brocShell.NIX_GLIBC_PATH;
          LD_LIBRARY_PATH = brocShell.LD_LIBRARY_PATH;
          NIXPKGS_ALLOW_UNFREE = brocShell.NIXPKGS_ALLOW_UNFREE;
        };
      });
}
