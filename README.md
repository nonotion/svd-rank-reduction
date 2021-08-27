# svd-rank-reduction
Animated rank reduction using the Singular Value Decomposition in Rust.
Produces a sequence of PNGs which may be unified via
`ffmpeg -r 30 -f image2 -s dim.0xdim.1 -i "filename %d.png" -vcodec libx264 -crf 17 -pix_fmt yuv420p filename.mp4`
![Fractal Brownian Motion Noise Example](example_fbm.gif)