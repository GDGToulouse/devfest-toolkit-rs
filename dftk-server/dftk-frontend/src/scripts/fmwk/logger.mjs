class ConsoleLogger {
  constructor(name) {
    this.name = `[${name}]`;
    this.appender = console;
  }

  debug(...data) {
    this.appender.debug("🐛 ", this.name, ...data);
  }

  info(...data) {
    this.appender.info("ℹ️ ", this.name, ...data);
  }

  warn(...data) {
    this.appender.info("⚠️ ", this.name, ...data);
  }

  error(...data) {
    this.appender.info("🚨️ ", this.name, ...data);
  }
}

export const createLogger = (name) => new ConsoleLogger(name);
