#!/bin/bash

set -xe

version="1.5.1"
dir="SFMT-src-${version}"
arc="${dir}.tar.gz"

wget http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/SFMT/${arc}
tar xvf $arc
rm $arc
clang-format -i ${dir}/*.h ${dir}/*.c
g++ -msse2 -DHAVE_SSE2 mm_recursion.c && ./a.out > mm_recursion.txt
g++ -msse2 -DHAVE_SSE2 init.c SFMT-src-1.5.1/SFMT.c && ./a.out > init1234.txt
