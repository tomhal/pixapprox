rem ffmpeg.exe -f image2 -framerate 15 -i %05d.png -s:v 512x256 -c:v libx264 -profile:v high -crf 12 -pix_fmt yuv420p movie.mp4

mencoder mf://*.png -mf w=800:h=600:fps=25:type=png -ovc lavc -lavcopts vcodec=mpeg4:mbd=2:trell -oac copy -o output.avi
