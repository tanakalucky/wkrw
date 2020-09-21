#include <stdio.h>
#include <ncurses.h>
#include <string.h>
#include <unistd.h>
#incluce <"wkrw.h">

int main() 
{
  int x, y, w, h;
  char *str = "wkrw";
  int key;

  /* start control prompt*/
  initscr();
  /* hide keyboard inputs */
  noecho();
  /* accept keyboard input without enter */
  cbreak();
  /* no wait keyboard input */
  timeout(0);
  /* hide cursor */
  curs_set(0);

  getmaxyx(stdscr, h, w);
  y = h/2;
  x = w - strlen(str);
  
  while (1) {
    erase();
    move(y, x);
    addstr(ROW1);
    refresh();

    key = getch();
    if (key == 'q') break;

    x--; if (x <= 0) x = w - strlen(str);

    usleep(100000);
  }

  endwin();

  return 0;
}
