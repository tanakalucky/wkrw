# include <stdio.h>
# include <ncurses.h>
# include <string.h>
# include <unistd.h>

int main() 
{
  int x, y, w, h;
  char *str = "wkrw";
  int key;

  /* ターミナル初期化 */
  initscr();
  /* キーボード入力を非表示 */
  noecho();
  /* enter不要の入力モード */
  cbreak();
  /* キー入力をまたない */
  timeout(0);
  /* カーソル非表示 */
  curs_set(0);

  getmaxyx(stdscr, h, w);
  y = h/2;
  x = w - strlen(str);
  
  while (1) {
    erase();
    move(y, x);
    addstr(str);
    refresh();

    key = getch();
    if (key == 'q') break;

    x--; if (x <= 0) x = w - strlen(str);

    usleep(100000);
  }

  endwin();

  return 0;
}
