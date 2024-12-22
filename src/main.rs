use std::{thread, time::Duration};

const WKRW: [&str; 15] = [
    r#"    #                 #                                #            "#,
    r#"    #                 #              #########         #            "#,
    r#"    #                 #      #             ##          #            "#,
    r#"    #   #####         #      ##           ##           #   #####    "#,
    r#" ####  ##   ##    ########    ##         ##         ####  ##   ##   "#,
    r#"    # ##     ##       #   #    #        ##             # ##     ##  "#,
    r#"    ###       #       #   #    ##      #######         ###       #  "#,
    r#"    ##        #      ##   #     #     ##     ##        ##        #  "#,
    r#"    #         #      #    #     #    ##       ##       #         #  "#,
    r#"   ##         #      #    #         ##         #      ##         #  "#,
    r#"  ###         #     ##    #            ###     #     ###         #  "#,
    r#" ## #        ##     #     #           #  ##    #    ## #        ##  "#,
    r#"    #       ##     ##    ##           #   #   ##       #       ##   "#,
    r#"    #     ###     ##     #            ##  #  ##        #     ###    "#,
    r#"    #                  ###             #######         #            "#,
];

const FRAME_DELAY: u64 = 40000;

fn main() {
    // ターミナルのサイズを取得
    let screen_width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80); // 取得できない場合は80を使用

    // カーソル位置を保存
    print!("\x1B[?47h"); // 代替画面バッファに切り替え
    print!("\x1B[?25l"); // カーソルを非表示

    let text_width = WKRW[0].len();

    // 文字列が完全に表示されてから消えるまでを表現
    for x in (-(text_width as i32)..screen_width as i32).rev() {
        // 画面をクリアしてカーソルを先頭に移動
        print!("\x1B[2J\x1B[1;1H");

        // 各行を処理
        for line in WKRW.iter() {
            if x >= 0 {
                print!("{:width$}", "", width = x as usize);

                // 画面内に表示される部分のみを切り出して表示
                let visible_width = if x > screen_width as i32 {
                    0
                } else {
                    screen_width - x as usize
                };

                println!("{}", &line[0..visible_width.min(line.len())]);
            } else {
                // 文字列の一部が画面外に出た場合
                let start_pos = (-x) as usize;
                if start_pos < line.len() {
                    println!("{}", &line[start_pos..]);
                }
            }
        }

        // 出力をフラッシュして確実に表示
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        thread::sleep(Duration::from_micros(FRAME_DELAY));
    }

    // 画面をクリアし、カーソルを元の位置に戻す
    print!("\x1B[2J"); // 画面クリア
    print!("\x1B[?47l"); // メイン画面バッファに戻る
    print!("\x1B[?25h"); // カーソルを再表示
    println!(); // 新しい行に移動
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}
