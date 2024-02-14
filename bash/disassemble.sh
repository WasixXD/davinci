#!/bin/bash

ffmpeg -i $1 -vf fps=1 dis/frames_%d.bmp
