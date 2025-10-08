#Rustでビルドしてojでテストするためのシェルスクリプト
#前みたいにバイナリ化しちゃってどうのこうのは保守が面倒なのでやめた
#引数$1=問題のURL

#コピーフェーズ
#ソースコードをALGO直下にコピー
#なんか、ALGO直下のmain.rsをcargo buildは参照しているっぽいです
cp main.rs /home/ouoz/workspace/comp/algo/main.rs

#ビルドフェーズ
#ビルドする、失敗したら終了
echo build project...
cargo build
if [ $? -eq 0 ]; then
    echo -e "\e[32;1mBuild succeeded"
else
    echo -e "\e[31;1mBuild failed"
    exit 1  
fi

#テストフェーズ
#バイナリファイルを持ってきてa.outとし
cp /home/ouoz/workspace/comp/algo/target/debug/algo a.out

#以前のtestフォルダを削除
echo delete previous cases...
rm -rf test

#oj dでtestフォルダを作成し
echo download cases...
oj d $1
if [ $? -ne 0 ]; then
    echo -e "\e[31;1mFailed to download test cases"
    exit 1  
fi

#oj tでテストを実行
echo test cases...
oj t

