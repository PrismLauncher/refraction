{ lib, dockerTools }:
refraction:

dockerTools.buildLayeredImage {
  name = "refraction";
  tag = "latest-${refraction.passthru.architecture}";
  config.Cmd = [ (lib.getExe refraction) ];
  inherit (refraction.passthru) architecture;
}
