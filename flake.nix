{

  description = "Development environment for StudyCycle.";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs =
    { self, nixpkgs }:
    let
      lib = nixpkgs.lib;
      systems = [ "x86_64-linux" ];
    in
    {
      devShells = lib.genAttrs lib.systems.flakeExposed (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default =
            if lib.elem system systems then
              pkgs.mkShell rec {
                name = "studycycle";
                packages = with pkgs; [
                  cargo
                  rustc
                  gcc
                  pnpm
                  nodejs
                  postgresql
                  jq
                  curl
                  psmisc
                ];
                shellHook = ''
                  # Initial Greeting
                  printf "\033[1m──── ${name}-env [ \033[32mENABLED\033[0m\033[1m ] ────\033[0m

                  · Rust        \033[1m[ \033[35m$(rustc --version | cut -d ' ' -f 2)\033[0m\033[1m ]\033[0m
                  · Cargo       \033[1m[ \033[35m$(cargo --version | cut -d ' ' -f 2)\033[0m\033[1m ]\033[0m
                  · Node.js     \033[1m[ \033[35m$(node --version | cut -c 2-)\033[0m\033[1m ]\033[0m
                  · pnpm        \033[1m[ \033[35m$(pnpm --version)\033[0m\033[1m ]\033[0m
                  · PostgreSQL  \033[1m[ \033[35m$(psql --version | cut -d ' ' -f 3)\033[0m\033[1m ]\033[0m

                  \033[1;33mNOTE\033[0m\033[1m: PostgreSQL\033[0m requires manual configuration.
                  See: https://join-lemmy.org/docs
                  "
                '';
              }
            else
              throw "unsupported architecture (${system})";
        }
      );
    };

}
