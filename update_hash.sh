#!/bin/bash
DNA_HASH=$(hc hash)
LEN_OUT=${#DNA_HASH}
echo ${DNA_HASH}
HASH=${DNA_HASH:$(expr $LEN_OUT - 46):$LEN_OUT}
sed -i "s/dna_hash = \".*/dna_hash = \"${HASH}\"/g" $1
