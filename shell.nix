{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
	buildInputs = [
		pkgs.vulkan-loader
		pkgs.cmake
		pkgs.python3
		pkgs.shaderc
	];
	LD_LIBRARY_PATH = "${pkgs.vulkan-loader}/lib";
}
