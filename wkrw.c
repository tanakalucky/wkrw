#include <stdio.h>
#include <ncurses.h>
#include <unistd.h>
#include "wkrw.h"

int main() 
{
  int x, y, w, h;
  int key;

  /* start control screen*/
  initscr();
  /* hide keyboard input */
  noecho();
  /* accept keyboard input without enter */
  cbreak();
  /* no wait keyboard input */
  timeout(0);
  /* hide cursor */
  curs_set(0);

  /* get screen height and width */
  getmaxyx(stdscr, h, w);
  y = (h - WKRWHEIGHT) /2;
  x = w;
  while (1) {
    erase();
    static char *wkrw[WKRWHEIGHT]
        = {ROW1, ROW2, ROW3, ROW4, ROW5, ROW6, ROW7, ROW8, ROW9, ROW10, ROW11, ROW12, ROW13, ROW14, ROW15};

    for (int i = 0; i < WKRWHEIGHT; ++i) {
	for (int j = 0; j < WKRWLENGTH; ++j) {
	    if (x + j < w) {
		    mvaddch(y + i, x + j, wkrw[i][j]);
	    }
	}	
    }

    refresh();

    key = getch();
    if (key == 'q') break;

    x--;
    if (x <= - WKRWLENGTH) x = w;

    usleep(100000);
  }

  endwin();

  return 0;
}
