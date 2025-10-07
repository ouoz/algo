#Rustでビルドしてojでテストするためのシェルスクリプト
#前みたいにバイナリ化しちゃってどうのこうのは保守が面倒なのでやめた
#引数$1=問題のURL


#ビルドフェーズ
#ビルドする、失敗したら終了
cargo build
if [ $? -eq 0 ]; then
    echo "Build succeeded"
else
    echo "Build failed"
    exit 1  
fi

#テストフェーズ
#バイナリファイルを持ってきてa.outとし
cp target/debug/algo a.out

#以前のtestフォルダを削除
rm -rf test

#oj dでtestフォルダを作成し
oj d $1

#oj tでテストを実行
oj t

