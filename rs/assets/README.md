Create image with ImageMagick:

```bash
nix shell nixpkgs#imagemagick
convert file.png -depth 1 gray:file.raw
```
