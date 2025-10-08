#コンテストの問題ごとのフォルダ、ソースファイルを一括作成するためのスクリプト

mkdir $1
cd $1

for c in {A..Z}
do
    mkdir ${c}
    cd ${c}
    touch main.rs
    cd ..
    if [ ${c} = ${2} ] ; then
        break
    fi
done
