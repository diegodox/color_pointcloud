# color pointcloud

入力plyファイルに指定のrgbを書き込みます（入力と同じフォーマットで書き出します）

入力plyの全エレメントについて以下の操作を行います．

もし，型がUcharで"r", "g", "b" のプロパティが存在していれば，そこに指定されたrgbを書き込みます．
なければプロパティを追加して，そこに指定されたrgbを書き込みます

## Usage

### Build and run

To show help

```sh
$ cargo run --release -- --help
```

Example usage

```sh
$ cargo run --release -- --src example.ply --dst colored_example.ply --r 100 --g 10 --b 1
```

### Build

```sh
$ cargo build --release
```

### Run

To show help

```sh
$ ./target/debug/color_pointcloud --help
```

Example usage

```sh
$ ./target/debug/color_pointcloud --src example.ply --dst colored_example.ply --r 100 --g 10 --b 1
```

## Limitation

- PLYファイルの仕様は[こちら](http://paulbourke.net/dataformats/ply/)に沿っています

    特にfloat32やint8のような書き方は対応していません．(エラーになります)
- エラーハンドリングがほとんどされていません

    内部（特にライブラリ内部）でエラーが発生しても意味のあるメッセージは返せません
