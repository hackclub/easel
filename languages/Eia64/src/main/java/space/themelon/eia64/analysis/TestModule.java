package space.themelon.eia64.analysis;

import space.themelon.eia64.runtime.Executor;
import space.themelon.eia64.syntax.Lexer;

public class TestModule {

  public final Object lock = new Object();

  public void loadModule() {
    synchronized(lock) {
      for (;;) {
        ParserX parser = new ParserX(new Executor());
        parser.parse(new Lexer("a = 5").getTokens());
      }
    }
  }
}
