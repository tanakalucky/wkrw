#include <stdio.h>
#include <ncurses.h>
#include <string.h>
#include <unistd.h>
#include "wkrw.h"

int main() 
{
  int x, y, w, h;
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

  /* get screem height and width */
  getmaxyx(stdscr, h, w);
  y = h/2;
  x = w - WKRWLENGTH;
  while (1) {
    erase();
    /*move(y, x);*/
    /*   addstr(ROW1);*/
    mvaddstr(y, x, ROW1);
    mvaddstr(y + 1, x, ROW2);
    mvaddstr(y + 2, x, ROW3);
    mvaddstr(y + 3, x, ROW4);
    mvaddstr(y + 4, x, ROW5);
    mvaddstr(y + 5, x, ROW6);
    mvaddstr(y + 6, x, ROW7);
    mvaddstr(y + 7, x, ROW8);
    mvaddstr(y + 8, x, ROW9);
    mvaddstr(y + 9, x, ROW10);
    mvaddstr(y + 10, x, ROW11);
    mvaddstr(y + 11, x, ROW12);
    mvaddstr(y + 12, x, ROW13);
    mvaddstr(y + 13, x, ROW14);
    mvaddstr(y + 14, x, ROW15);
    refresh();

    key = getch();
    if (key == 'q') break;

    x--; if (x <= 0) x = w - WKRWLENGTH;

    usleep(100000);
  }

  endwin();

  return 0;
}
