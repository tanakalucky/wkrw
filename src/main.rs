use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use std::{io::stdout, io::Result, io::Write, thread, time::Duration};

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

fn main() -> Result<()> {
    let mut stdout = stdout();

    let screen_width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80); // 取得できない場合は80を使用

    execute!(stdout, EnterAlternateScreen, Hide)?;

    let text_width = WKRW[0].len();

    // 文字列が完全に表示されてから消えるまでを表現
    for x in (-(text_width as i32)..screen_width as i32).rev() {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

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
        Write::flush(&mut stdout)?;

        thread::sleep(Duration::from_micros(FRAME_DELAY));
    }

    execute!(stdout, Hide, LeaveAlternateScreen, Show)?;

    Write::flush(&mut stdout)?;

    Ok(())
}
