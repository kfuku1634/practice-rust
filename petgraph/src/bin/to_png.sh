#!/bin/bash

img_type="png"

rm ${img_type}/*.${img_type}
rm dot/*.dot

for rs_file in `find . -name "*.rs"`
do
    base=$(basename ${rs_file} ".rs")
    echo ${base}
    dot_file=dot/${base}.dot
    img_file=${img_type}/${base}.${img_type}

    cargo run --bin ${base} > $dot_file
    dot -Tpng -o $img_file $dot_file
done
