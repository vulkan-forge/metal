{ pkgs ? import <nixpkgs> {} }: pkgs.mkShell {
	buildInputs = with pkgs; [
		vulkan-loader
		cmake
		python3
		shaderc
	];
	LD_LIBRARY_PATH = "${pkgs.vulkan-loader}/lib";
}