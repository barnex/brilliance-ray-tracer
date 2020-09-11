#! /bin/bash
yes | ffmpeg -pattern_type glob -i 'out/*.jpg' -c:v vp9 -r 60 -crf 30 -b:v 2000k output.webm 