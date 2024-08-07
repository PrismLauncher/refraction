{ lib, dockerTools }:
refraction:

dockerTools.buildLayeredImage {
  name = "refraction";
  tag = "latest-${refraction.passthru.dockerArchitecture}";
  config.Cmd = [ (lib.getExe refraction) ];
  architecture = refraction.passthru.dockerArchitecture;
}
