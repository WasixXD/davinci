#!/bin/bash

ffmpeg -framerate 1 -i frames/frame%d.png -c:v libx264 -r 30 -pix_fmt yuv420p $1
