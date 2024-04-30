{inputs, ...}: {
  perSystem = {
    lib,
    system,
    ...
  }: {
    _module.args = {
      refraction' = lib.mapAttrs (lib.const (v: v.${system} or v)) (inputs.get-flake ../../.);
    };
  };
}
