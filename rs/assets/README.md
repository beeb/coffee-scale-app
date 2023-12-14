Create image with ImageMagick:

```bash
nix shell nixpkgs#imagemagick
convert file.bmp -depth 1 gray:file.raw
```
