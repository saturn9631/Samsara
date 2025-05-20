{
	description = "A very basic flake";

	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-25.05-darwin";
	};

	outputs = { self, nixpkgs }: 
	let
			system = "x86_64-linux";
			pkgs = nixpkgs.legacyPackages.${system};
	in 
	{
			devShells.${system}.default = 
				pkgs.mkShell {
					buildInputs = with pkgs; [
						rustup
						#libllvm
						#llvm-manpages
						#lld_20
						#clang_20

						#qemu
						#qemu_kvm
						qemu_full
						#OVMF
						OVMFFull
					];
				};
	};
}
