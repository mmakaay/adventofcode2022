#!/bin/bash

DISK_SPACE=70000000
REQ_SPACE=30000000

if [ "$1" = "" ]; then
    echo "Usage: $0 <input file>"
    exit 1
fi

TMP=/tmp/hacky.$$
mkdir $TMP

cat $1 \
    | sed -e 's!^$ cd \/$!mkdir TREE \&\& cd TREE!' \
    | sed -e 's!^$ ls!!' \
    | sed -e 's!^dir .*!!' \
    | sed -e 's!^$ cd \.\.!cd ..!' \
    | sed -e 's!^$ cd \(.*\)!mkdir \1 \&\& cd \1!' \
    | sed -e 's!^\([0-9][0-9]*\) \(.*\)!dd if=/dev/zero of="\2" count=1 bs="\1" 2>/dev/null!' \
    > $TMP/setup.sh

cd $TMP
bash setup.sh

SIZES=()
for P in `find $TMP/TREE -type d`; do
    SIZE=$(find $P -type f -exec ls -la {} \; | awk '{print $5}' | paste -sd+ - | bc)
    SIZES+=($SIZE)
done

SMALL_ONES=()
SIZE_ALL=0
for SIZE in ${SIZES[@]}; do
    if [[ $SIZE -lt 100000 ]]; then
        SMALL_ONES+=($SIZE)
    fi
    if [[ $SIZE -gt $SIZE_ALL ]]; then
        SIZE_ALL=$SIZE
    fi
done

ANSWER1=$(IFS=+; echo "$((${SMALL_ONES[*]}))")

FREE_SPACE=$(($DISK_SPACE-$SIZE_ALL))
MUST_FREE=$(($REQ_SPACE-$FREE_SPACE))

ANSWER2=$SIZE_ALL
for SIZE in ${SIZES[@]}; do
    if [[ $SIZE -ge $MUST_FREE ]]; then
        if [[ $SIZE -lt $ANSWER2 ]]; then
            ANSWER2=$SIZE
        fi
    fi
done

echo ""
echo "------------------------------------------------"
echo "    *** Elfinator 2000 disk usage report ***    "
echo "------------------------------------------------"
echo "Total disk space         : $DISK_SPACE"
echo "Size of all files        : $SIZE_ALL"
echo "Free disk space          : $FREE_SPACE"
echo "Required disk space      : $REQ_SPACE"
echo "Must free at least       : $MUST_FREE"
echo ""
echo "Sum of dir sizes < 10000 : $ANSWER1 (answer 1)"
echo "Size of dir to delete    : $ANSWER2 (answer 2)"
echo "------------------------------------------------"
echo ""

\rm -fR $TMP
