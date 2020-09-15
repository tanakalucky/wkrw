# include <stdio.h>
# include <ncurses.h>
# include <string.h>
# include <unistd.h>

int main() 
{
  int x, y, w, h;
  char *str = "wkrw";
  int key;

  initscr();
  noecho();
  cbreak();
  timeout(0);

  getmaxyx(stdscr, h, w);
  y = h/2;
  x = w;
  
  while (1) {
    erase();
    move(y, x);
    addstr(str);
    refresh();

    key = getch();
    if (key == 'q') break;

    x--; if (x <= 0) x = w;

    usleep(100000);
  }

  endwin();

  return 0;
}
