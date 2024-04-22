#!/bin/bash

set -euo pipefail

toolchain=${1:-stable}

echo "Building with ${toolchain}"

rustup target add thumbv6m-none-eabi --toolchain=${toolchain}
rustup target add thumbv7m-none-eabi --toolchain=${toolchain}
rustup target add thumbv7em-none-eabi --toolchain=${toolchain}
rustup target add thumbv7em-none-eabihf --toolchain=${toolchain}
rustup target add thumbv8m.base-none-eabi --toolchain=${toolchain}
rustup target add thumbv8m.main-none-eabi --toolchain=${toolchain}
rustup target add thumbv8m.main-none-eabihf --toolchain=${toolchain}

i=0
while read -r arg; do
    i=$((i + 1))
    if [[ $arg != \#* ]]; then
        target=`echo $arg | cut -f1 -d\|`
        args=`echo $arg | cut -f2 -d\|`
        echo "Building ${toolchain} ${target} ${args}..."
        RUSTFLAGS="$args" cargo +${toolchain} build --release --target $target || exit 1
        cp ./target/$target/release/armtesting $i.elf
        arm-none-eabi-objdump $i.elf --no-addresses -dCt  | sed "s/$i\.elf/filename.elf/" > $i.log 
    fi
done < targets.txt

shasum *.log | sort > compare.txt

for i in *.log; do
    line_no=${i/.log/}
    line=`cat targets.txt | head -n $line_no | tail -n 1`
    line=${line/\\n//}
    echo "swapping ${i} for $line_no ~$line~"
    mv compare.txt compare_old.txt
    sed "s/ ${i}$/ $line/g" compare_old.txt > compare.txt
    rm compare_old.txt
done

out_dir=./${toolchain}_out

rm -rf ${out_dir}
mkdir ${out_dir}
mv *.elf *.log ${out_dir}
mv compare.txt ${out_dir}/compare.txt

../analyse/target/debug/analyse ${out_dir}/compare.txt > ${out_dir}/analysed.txt
